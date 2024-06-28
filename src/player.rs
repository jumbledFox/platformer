use macroquad::{input::{is_key_down, is_key_pressed, KeyCode}, math::{FloatExt, Vec2}};


const KEY_MOVE_LEFT:  KeyCode = KeyCode::Left;
const KEY_MOVE_RIGHT: KeyCode = KeyCode::Right;
const KEY_JUMP:       KeyCode = KeyCode::Z;
const KEY_RUN:        KeyCode = KeyCode::A;

#[derive(Default)]
pub struct Player {
    flip_x:   bool,
    jumping:  bool,
    skidding: bool,
    skid_start_speed: f32,
    walk_timer: f32,
    walk_dir: Option<WalkDir>,
    running_time: f32, // How long you've been running for, used for the "p meter"

    pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,
    gravity: f32,
    
    // Constants
    walk_speed:    f32,

    run_beg_speed: f32,

    run_mid_speed: f32,
    run_mid_time:  f32,

    run_end_speed: f32,
    run_end_time:  f32,

    jump_height:   f32,
    jump_gravity:  f32,
    fall_gravity:  f32,
}

#[derive(PartialEq, Eq)]
enum WalkDir {
    Left, Right
}

impl Player {
    pub fn new(pos: Vec2) -> Player {
        Player {
            pos,

            walk_speed:      70.0,
            run_beg_speed:  110.0,
            run_mid_speed:  120.0,
            run_mid_time:     0.5,
            run_end_speed:  140.0,
            run_end_time:     1.2,
            jump_height:    250.0,
            jump_gravity:   512.0,
            fall_gravity:  1024.0,

            ..Default::default()
        }
    }

    pub fn pos(&self)    -> Vec2 { self.pos }
    pub fn flip_x(&self) -> bool { self.flip_x }

    pub fn update(&mut self, delta: f32, _recip_mul: f32) -> usize {
        let grounded = self.pos.y <= 16.0;

        // Jumping
        if is_key_pressed(KEY_JUMP) && grounded {
            self.vel.y = self.jump_height; 
            self.jumping = true;
        }

        if !is_key_down(KEY_JUMP) {
            self.jumping = false;
        }

        let gravity_mul = (self.vel.x.recip() * 10.0).abs().clamp(0.0, 1.0);
        let target_gravity = match is_key_down(KEY_JUMP) && self.jumping {
            true  => self.jump_gravity,
            false => self.fall_gravity,
        } * gravity_mul;
        self.gravity = self.gravity.lerp(target_gravity, delta * 100.0);
        self.vel.y -= self.gravity * delta;

        // println!("{:?}", gravity_mul);

        // Moving

        // Start moving if you press a direction
        if is_key_pressed(KEY_MOVE_LEFT)  { self.walk_dir = Some(WalkDir::Left);  }
        if is_key_pressed(KEY_MOVE_RIGHT) { self.walk_dir = Some(WalkDir::Right); }
        // Stop moving if you stop holding the direction you're going in
        if (matches!(self.walk_dir, Some(WalkDir::Left))  && !is_key_down(KEY_MOVE_LEFT)
        ||  matches!(self.walk_dir, Some(WalkDir::Right)) && !is_key_down(KEY_MOVE_RIGHT)) && grounded {
            self.walk_dir = None;
        }

        // If you're walking one way and actually going the other way, you should skid!
        let prev_skidding = self.skidding;
        let walking_opposing_velocity =
           (self.walk_dir == Some(WalkDir::Left) && self.vel.x.is_sign_positive())
        || (self.walk_dir == Some(WalkDir::Right) && self.vel.x.is_sign_negative());
        self.skidding = grounded && walking_opposing_velocity;
        
        // If you started skidding this frame, make your x velocity lower so the skid is snappier (https://www.desmos.com/calculator/g18oxsmchz), and make sure you're not 'end' running
        if !prev_skidding && self.skidding {
            self.vel.x *= (0.4/-70.0) * self.vel.x.abs() + 1.3;
            self.running_time = self.running_time.min(self.run_mid_time);
        }

        // This is the timer for if you're running at maximum speed
        // It should only be altered if you're grounded and running in a direction, increasing if you're running at the right speed or skidding
        self.running_time += match (grounded, (self.vel.x.abs() >= self.run_beg_speed || self.skidding || self.walk_dir.is_some()) && is_key_down(KEY_RUN)) {
            (false, _) => 0.0,
            (_, true)  =>  delta,
            (_, false) => -delta,
        };
        self.running_time = self.running_time.clamp(0.0, self.run_end_time);

        let target_speed = match is_key_down(KEY_RUN) {
            false    => self.walk_speed,
            _ if self.running_time >= self.run_end_time => self.run_end_speed,
            _ if self.running_time >= self.run_mid_time => self.run_mid_speed,
            _                                           => self.run_beg_speed,
        };
        println!("{:?}", target_speed);
        
        // let target_speed = match target_speed.abs() < self.skid_start_speed.abs() {
        //     true  => target_speed.signum() * self.skid_start_speed,
        //     false => target_speed,
        // };

        self.target_x_vel = match self.walk_dir {
            Some(WalkDir::Left)  => { self.flip_x = true; -target_speed },
            Some(WalkDir::Right) => { self.flip_x = false; target_speed },
            None => 0.0,
        };

        // let vel_step = match grounded {
        //     true  => 300.0,
        //     false => 100.0,
        // };
        let vel_step = 400.0 + self.vel.x.abs();

        self.vel.x = self.vel.x.to_target(self.target_x_vel, vel_step * delta);

        self.walk_timer = (self.walk_timer + self.vel.x.abs() / 1000.0).rem_euclid(1.0);
        if self.vel.x == 0.0 { self.walk_timer = 0.0; }

        // Start skidding if you're going a direction opposed to your walk dir
        // if matches!(self.walk_dir, Some(WalkDir::Right) if self.vel.x <= -self.target_x_vel)
        // || matches!(self.walk_dir, Some(WalkDir::Left)  if self.vel.x >= -self.target_x_vel) {
        //     println!("start skidding");
        //     self.skid_start_speed = self.vel.x;
        //     self.skidding = true;
        // }

        // Stop skidding if you're going the intended direction and you're above a certain velocity
        // if self.skidding
        // && (self.vel.x >= self.skid_start_speed && matches!(self.walk_dir, Some(WalkDir::Right))
        // ||  self.vel.x <= self.skid_start_speed && matches!(self.walk_dir, Some(WalkDir::Left))
        // || self.target_x_vel == 0.0) {
        //     self.skidding = false;
        // }

        // println!("vel_x: {:?}\nskid_start_speed: {:?} target_x_vel: {:?}", self.vel.x, self.skid_start_speed, self.target_x_vel);
    

        // println!("{:?}", self.running_time);
        
        self.pos += self.vel * delta;

        let grounded = self.grounded();
        if grounded {
            self.pos.y = 16.0;
            self.jumping = false;
        }

        let falling = self.vel.y < 0.0;
        let walk_frame = self.walk_timer < 0.5;
        match (grounded, falling, self.skidding, self.vel.x.abs()) {
            (false, false, ..) => 5, // Jumping
            (false, true,  ..) => 6, // Falling
            (.., true, _)    => 4, // Skidding
            (.., vel_x) if vel_x == 0.0 => 0, // Idle
            (.., vel_x) if vel_x < 130.0 => if walk_frame {2} else {0} // Walking
            _                            => if walk_frame {3} else {1} // Running
        }
    }

    fn grounded(&self) -> bool {
        self.pos.y <= 16.0
    }
}

pub trait ToTarget {
    fn to_target(self, target: Self, step: Self) -> Self;
}

impl ToTarget for f32 {
    fn to_target(self, target: Self, step: Self) -> Self {
        match (self - target).is_sign_positive() {
            true  => (self - step).max(target),
            false => (self + step).min(target),
        }
    }
}