use macroquad::{color::{BLUE, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, FloatExt, Rect, Vec2}, shapes::draw_circle, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::{particles::Particles, stage::Stage};

const KEY_MOVE_LEFT:  KeyCode = KeyCode::A;         // KeyCode::Left;
const KEY_MOVE_RIGHT: KeyCode = KeyCode::D;         // KeyCode::Right;
const KEY_JUMP:       KeyCode = KeyCode::Space;     // KeyCode::Z;
const KEY_RUN:        KeyCode = KeyCode::LeftShift; // KeyCode::A;

#[derive(PartialEq, Eq)]
enum WalkDir {
    Left, Right
}

#[derive(Default)]
pub struct Player {
    flip_x:   bool,
    jumping:  bool,
    grounded: bool,
    skidding: bool,
    step_anim: f32,
    walk_dir: Option<WalkDir>,
    running_time: f32, // How long you've been running for, used for the "p meter"
    running: bool,
    sprite: usize,

    pos: Vec2,
    prev_pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,
    gravity: f32,
    
    // Constants
    walk_speed:    f32,
    run_beg_speed: f32,
    run_end_speed: f32,
    run_lerp_beg_time:  f32,
    run_lerp_end_time:  f32,
    jump_vel:     f32,
    jump_gravity: f32,
    fall_gravity: f32,
    max_fall_speed: f32,
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
            jump_vel:           250.0,
            jump_gravity:       600.0,
            fall_gravity:      1100.0,
            max_fall_speed:     500.0,

            ..Default::default()
        }
    }

    pub fn pos(&self)    -> Vec2 { self.pos }
    pub fn flip_x(&self) -> bool { self.flip_x }

    pub fn update(&mut self, delta: f32, stage: &mut Stage, particles: &mut Particles) {
        // Jumping
        if !self.grounded {
            self.vel.y = (self.vel.y + self.gravity * delta).min(self.max_fall_speed);
        }

        // If you're no-longer holding the jump key, or you were jumping and you're either grounded or now going down, you're not jumping anymore!
        if !is_key_down(KEY_JUMP) || self.jumping && (self.grounded || self.vel.y > 0.0) {
            self.jumping = false;
        }
        // Jump if you try to and are able
        if is_key_pressed(KEY_JUMP) && self.grounded {
            self.jumping = true;
            self.vel.y = -self.jump_vel;
        } 
        // Your gravity should be decreased when holding the jump key
        self.gravity = match self.jumping {
            true  => self.jump_gravity,
            false => self.fall_gravity,
        };
        // TODO: Your gravity should be increased depending on your horizontal speed

        // Moving

        // Start moving if you press a direction.
        if is_key_pressed(KEY_MOVE_LEFT)  { self.walk_dir = Some(WalkDir::Left);  }
        if is_key_pressed(KEY_MOVE_RIGHT) { self.walk_dir = Some(WalkDir::Right); }
        // Stop moving if you stop holding the direction you're going in.
        if (matches!(self.walk_dir, Some(WalkDir::Left))  && !is_key_down(KEY_MOVE_LEFT)
        ||  matches!(self.walk_dir, Some(WalkDir::Right)) && !is_key_down(KEY_MOVE_RIGHT)) && self.grounded {
            self.walk_dir = None;
        }

        // TODO: Think about this
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
        self.prev_pos = self.pos;
        self.pos += self.vel * delta;

        // Collisions and resolution
        // Foot collisions
        let foot_l = vec2( 5.5, 32.0);
        let foot_r = vec2(10.5, 32.0);

        let feet_colliding = stage.tile_solid_pos(foot_l + self.pos) || stage.tile_solid_pos(foot_r + self.pos);

        // If you're not grounded and your feet are in a tile, you should go to the top of it
        if feet_colliding && self.pos.y >= self.prev_pos.y {
            self.pos.y = (self.pos.y / 16.0).floor() * 16.0;
            self.vel.y = 0.0;
        }
        self.grounded = feet_colliding;

        // Side collisions
        let side_l = vec2( 3.5, 26.0);
        let side_r = vec2(12.5, 26.0);

        if stage.tile_solid_pos(side_l + self.pos) && self.pos.x < self.prev_pos.x {
            self.pos.x = (self.pos.x / 16.0).ceil() * 16.0 - side_l.x;
            self.vel.x = 0.0;
            self.running_time = 0.0;
        }
        if stage.tile_solid_pos(side_r + self.pos) && self.pos.x > self.prev_pos.x {
            self.pos.x = (self.pos.x / 16.0).ceil() * 16.0 - side_r.x;
            self.vel.x = 0.0;
            self.running_time = 0.0;
        }

        // Head collision
        let head = vec2( 8.0, 16.5);
        
        if stage.tile_solid_pos(head + self.pos) && self.pos.y < self.prev_pos.y {
            self.pos.y = (self.pos.y / 16.0).ceil() * 16.0;
            self.vel.y = 0.0;
            particles.break_tile(((head + self.pos) / 16.0).floor() * 16.0 + vec2(0.0, -16.0));
            stage.set_tile_pos(0, head + self.pos - vec2(0.0, 16.0));
        }

        // Animate the sprite walking depending on how fast you're going.
        self.step_anim = (self.step_anim + self.vel.x.abs() / 1000.0).rem_euclid(1.0);
        // If you've stopped, make sure you
        if self.vel.x == 0.0 { self.step_anim = 0.99999999; }
        
        // Everything below here needs reworking
        let falling = self.vel.y >= 0.0;
        let walk_frame = self.step_anim < 0.5;
        self.sprite = match (self.grounded, falling, self.skidding, self.vel.x.abs()) {
            (false, false, ..) => 5, // Jumping
            (false, true,  ..) => 6, // Falling
            (.., true, _)      => 4, // Skidding
            (.., vel_x) if vel_x == 0.0 && self.walk_dir == None => 0, // Idle
            (.., vel_x) if vel_x < self.run_end_speed => if walk_frame {2} else {0} // Walking
            _                                         => if walk_frame {3} else {1} // Running
        };
    }

    pub fn draw(&self, tex: &Texture2D) {
        draw_texture_ex(tex, self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            flip_x: self.flip_x,
            source: Some(Rect::new(self.sprite as f32 * 16.0, 0.0, 16.0, 32.0)),
            ..Default::default()
        });

        let foot_l = vec2( 5.5, 32.0) + self.pos;
        let foot_r = vec2(10.5, 32.0) + self.pos;
        let side_l = vec2( 3.5, 26.0) + self.pos;
        let side_r = vec2(12.5, 26.0) + self.pos;
        let head   = vec2( 8.0, 16.5) + self.pos;
        draw_circle(foot_l.x, foot_l.y, 0.5, YELLOW);
        draw_circle(foot_r.x, foot_r.y, 0.5, YELLOW);
        draw_circle(side_l.x, side_l.y, 0.5, RED);
        draw_circle(side_r.x, side_r.y, 0.5, RED);
        draw_circle(head.x,   head.y,   0.5, BLUE);
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