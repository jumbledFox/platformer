use macroquad::{input::{is_key_down, is_key_pressed, KeyCode}, math::{FloatExt, Vec2}};

const KEY_MOVE_LEFT:  KeyCode = KeyCode::Left;
const KEY_MOVE_RIGHT: KeyCode = KeyCode::Right;
const KEY_JUMP:       KeyCode = KeyCode::Z;
const KEY_RUN:        KeyCode = KeyCode::A;

#[derive(PartialEq, Eq)]
enum WalkDir {
    Left, Right
}

#[derive(Default)]
pub struct Player {
    flip_x:   bool,
    // jumping:  bool,
    grounded: bool,
    skidding: bool,
    walk_timer: f32,
    walk_dir: Option<WalkDir>,
    running_time: f32, // How long you've been running for, used for the "p meter"
    running: bool,

    pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,
    // gravity: f32,
    
    // Constants
    walk_speed:    f32,
    run_beg_speed: f32,
    run_end_speed: f32,
    run_lerp_beg_time:  f32,
    run_lerp_end_time:  f32,
    // jump_height:   f32,
    // jump_gravity:  f32,
    // fall_gravity:  f32,
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
            // jump_height:        250.0,
            // jump_gravity:       512.0,
            // fall_gravity:      1024.0,

            ..Default::default()
        }
    }

    pub fn pos(&self)    -> Vec2 { self.pos }
    pub fn flip_x(&self) -> bool { self.flip_x }

    pub fn update(&mut self, delta: f32) -> usize {
        // Jumping - not yet properly implemented.
        /*
        self.vel.y -= self.gravity * delta;
        if is_key_pressed(KEY_JUMP) && grounded {
            self.vel.y = self.jump_height; 
            self.jumping = true;
        }
        let target_gravity = match is_key_down(KEY_JUMP) && self.jumping {
            true  => self.jump_gravity,
            false => self.fall_gravity,
        };
        self.gravity = self.gravity.lerp(target_gravity, delta * 100.0);
        */

        // Moving

        // Start moving if you press a direction.
        if is_key_pressed(KEY_MOVE_LEFT)  { self.walk_dir = Some(WalkDir::Left);  }
        if is_key_pressed(KEY_MOVE_RIGHT) { self.walk_dir = Some(WalkDir::Right); }
        // Stop moving if you stop holding the direction you're going in.
        if (matches!(self.walk_dir, Some(WalkDir::Left))  && !is_key_down(KEY_MOVE_LEFT)
        ||  matches!(self.walk_dir, Some(WalkDir::Right)) && !is_key_down(KEY_MOVE_RIGHT)) && self.grounded {
            self.walk_dir = None;
        }

        // `self.running` is updated only when these certain conditions are met
        // If you're moving fast enough and holding a direction, you're running!
        if self.vel.x.abs() >= self.run_beg_speed && self.walk_dir.is_some() {
            self.running = true;
        }
        // If you're not moving fast enough and not holding a direction, you're not running.
        if self.vel.x.abs()  < self.run_beg_speed && self.walk_dir.is_none() {
            self.running = false;
        }
        // If you're not holding down the run key, you're not running, duh!
        if !is_key_down(KEY_RUN) {
            self.running = false;
        }
        
        // This is the timer for if you're running at maximum speed.
        // If you're not grounded or skidding, it should freeze. Otherwise it should increase/decrease depending on if you're running or not.
        self.running_time += match (!self.grounded || self.skidding, self.running) {
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
        self.skidding = self.grounded && walking_opposing_velocity;
        
        // If you started skidding this frame, make your x velocity lower so the skid is snappier.
        // And, if you're running fast, go back to the beginning running speed (by setting the running_time back).
        if !prev_skidding && self.skidding {
            // This is minimum of this is 0.25, you'd never really reach this in practice but a minimum of 0.0 looks really bad in my testing. Just some food for thought...
            self.vel.x *= self.vel.x.abs().remap(self.walk_speed, self.run_end_speed, 1.0, 0.5).clamp(0.25, 1.0);
            self.running_time = self.running_time.min(self.run_lerp_beg_time);
        }

        // Set a target velocity depending on your direction
        let target_speed = || match is_key_down(KEY_RUN) {
            false => self.walk_speed,
            true  => self.running_time.remap(self.run_lerp_beg_time, self.run_lerp_end_time, self.run_beg_speed, self.run_end_speed).clamp(self.run_beg_speed, self.run_end_speed),
        };
        self.target_x_vel = match self.walk_dir {
            Some(WalkDir::Left)  => { self.flip_x = true; -target_speed() },
            Some(WalkDir::Right) => { self.flip_x = false; target_speed() },
            None => 0.0,
        };

        // Move the velocity towards the target, faster if you're moving faster.
        let vel_step = 400.0 + self.vel.x.abs();
        self.vel.x = self.vel.x.to_target(self.target_x_vel, vel_step * delta);
        
        // Actually move the player!
        self.pos += self.vel * delta;
        // This only works for a flat, infinite floor, just for testing!!
        self.grounded = self.pos.y <= 16.0;
        
        // Animate the sprite walking depending on how fast you're going.
        self.walk_timer = (self.walk_timer + self.vel.x.abs() / 1000.0).rem_euclid(1.0);
        if self.vel.x == 0.0 { self.walk_timer = 0.0; }
        
        // Everything below here needs reworking
        // The final function should NOT return the sprite number, but for now this works...
        let falling = self.vel.y < 0.0;
        let walk_frame = self.walk_timer < 0.5;
        match (self.grounded, falling, self.skidding, self.vel.x.abs()) {
            (false, false, ..) => 5, // Jumping
            (false, true,  ..) => 6, // Falling
            (.., true, _)      => 4, // Skidding
            (.., vel_x) if vel_x == 0.0 && self.walk_dir == None => 0, // Idle
            (.., vel_x) if vel_x < self.run_end_speed => if walk_frame {2} else {0} // Walking
            _                                         => if walk_frame {3} else {1} // Running
        }
    }
}

// Increments / decrements a value towards a target by 'step'.
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