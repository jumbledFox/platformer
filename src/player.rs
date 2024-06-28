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
    walk_timer: f32,
    walk_dir: Option<WalkDir>,
    running_time: f32, // How long you've been running for, used for the "p meter"
    running: bool,

    pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,
    gravity: f32,
    
    // Constants
    walk_speed:    f32,
    run_beg_speed: f32,
    run_end_speed: f32,
    run_lerp_beg_time:  f32,
    run_lerp_end_time:  f32,
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

            walk_speed:          70.0,
            run_beg_speed:      110.0,
            run_end_speed:      150.0,
            run_lerp_beg_time:    0.4,
            run_lerp_end_time:    1.2,
            jump_height:        250.0,
            jump_gravity:       512.0,
            fall_gravity:      1024.0,

            ..Default::default()
        }
    }

    pub fn pos(&self)    -> Vec2 { self.pos }
    pub fn flip_x(&self) -> bool { self.flip_x }

    pub fn update(&mut self, delta: f32) -> usize {
        let grounded = self.pos.y <= 16.0;

        // Jumping
        // self.vel.y -= self.gravity * delta;
        // if is_key_pressed(KEY_JUMP) && grounded {
        //     self.vel.y = self.jump_height; 
        //     self.jumping = true;
        // }
        // let target_gravity = match is_key_down(KEY_JUMP) && self.jumping {
        //     true  => self.jump_gravity,
        //     false => self.fall_gravity,
        // };
        // self.gravity = self.gravity.lerp(target_gravity, delta * 100.0);

        // Temporary hacky thing for testing... ignore this (and all mentions of it)
        let foo = self.pos.x >= 16.0 * 11.0 || self.pos.x <= 16.0 * 1.0;

        // Moving

        // Start moving if you press a direction.
        if is_key_pressed(KEY_MOVE_LEFT)  { self.walk_dir = Some(WalkDir::Left);  }
        if is_key_pressed(KEY_MOVE_RIGHT) { self.walk_dir = Some(WalkDir::Right); }
        // Stop moving if you stop holding the direction you're going in.
        if (matches!(self.walk_dir, Some(WalkDir::Left))  && !is_key_down(KEY_MOVE_LEFT)
        ||  matches!(self.walk_dir, Some(WalkDir::Right)) && !is_key_down(KEY_MOVE_RIGHT)) && grounded {
            self.walk_dir = None;
        }

        // If you're moving fast enough and holding a direction, you're running!
        if self.vel.x.abs() >= self.run_beg_speed && self.walk_dir.is_some() {
            self.running = true;
        }
        // If you're not moving fast enough and not holding a direction, you shouldn't be running.
        if self.vel.x.abs()  < self.run_beg_speed && self.walk_dir.is_none() {
            self.running = false;
        }
        // If you're not holding down the run key, you're not running, duh!
        if !is_key_down(KEY_RUN) || foo {
            self.running = false;
        }

        // This is the timer for if you're running at maximum speed
        // If you're not grounded, it should freeze. Otherwise it should increase/decrease depending on if you're running or not.
        self.running_time += match (!grounded, self.running) {
            (true, _)  =>  0.0,
            (_, true)  =>  delta,
            (_, false) => -delta,
        };
        self.running_time = self.running_time.clamp(0.0, self.run_lerp_end_time);

        // If you're walking one way and actually going the other way, you should skid!
        let prev_skidding = self.skidding;
        let walking_opposing_velocity =
           (self.walk_dir == Some(WalkDir::Left)  && self.vel.x > 0.0)
        || (self.walk_dir == Some(WalkDir::Right) && self.vel.x < 0.0);
        self.skidding = grounded && walking_opposing_velocity;
        
        // If you started skidding this frame, make your x velocity lower so the skid is snappier (https://www.desmos.com/calculator/qmob28z1yb).
        // And, if you're running fast, go back to the beginning running speed (by setting the running_time back).
        if !prev_skidding && self.skidding {
            let g = (1.0 - 0.5) / (self.run_beg_speed - self.run_end_speed);
            self.vel.x *= (g*(self.vel.x.abs() - self.run_beg_speed) + 1.0).clamp(0.0, 1.0);
            self.running_time = self.running_time.min(self.run_lerp_beg_time);
        }

        // Set a target velocity depending on your direction
        let target_speed = || match is_key_down(KEY_RUN) && !foo {
            false => self.walk_speed,
            true  => {
                // Lerp nicely between the beginning speed and the end speed (https://www.desmos.com/calculator/vl2lxkgnsv).
                let t = (1.0 / (self.run_lerp_end_time - self.run_lerp_beg_time)) * self.running_time - (self.run_lerp_beg_time / (self.run_lerp_end_time - self.run_lerp_beg_time));
                f32::lerp(self.run_beg_speed, self.run_end_speed, t.clamp(0.0, 1.0))
            }
        };
        self.target_x_vel = match self.walk_dir {
            Some(WalkDir::Left)  => { self.flip_x = true; -target_speed() },
            Some(WalkDir::Right) => { self.flip_x = false; target_speed() },
            None => 0.0,
        };

        // Move the velocity towards the target, faster if you're moving faster.
        let vel_step = 400.0 + self.vel.x.abs();
        self.vel.x = self.vel.x.to_target(self.target_x_vel, vel_step * delta);
        
        // Animate the sprite walking depending on how fast you're going.
        self.walk_timer = (self.walk_timer + self.vel.x.abs() / 1000.0).rem_euclid(1.0);
        if self.vel.x == 0.0 { self.walk_timer = 0.0; }

        // Actually move the player!
        self.pos += self.vel * delta;
        
        // Everything below here needs reworking
        if self.pos.x >= 16.0 * 11.0 { self.pos.x = 16.0 * 11.0; self.vel.x = 0.0; }
        if self.pos.x <= 16.0 *  1.0 { self.pos.x = 16.0 *  1.0; self.vel.x = 0.0; }

        let grounded = self.grounded();
        if grounded {
            self.pos.y = 16.0;
            self.jumping = false;
        }

        let falling = self.vel.y < 0.0;
        let walk_frame = self.walk_timer < 0.5;
        match (grounded, falling, self.skidding, self.vel.x.abs()) {
            (false, false, ..)          => 5, // Jumping
            (false, true,  ..)          => 6, // Falling
            (.., true, _)               => 4, // Skidding
            (.., vel_x) if vel_x == 0.0 && self.walk_dir == None => 0, // Idle
            (.., vel_x) if vel_x < self.run_end_speed => if walk_frame {2} else {0} // Walking
            _                                         => if walk_frame {3} else {1} // Running
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