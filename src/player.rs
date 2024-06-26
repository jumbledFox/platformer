use macroquad::{input::{is_key_down, is_key_pressed, KeyCode}, math::{FloatExt, Vec2}};


const KEY_MOVE_LEFT:  KeyCode = KeyCode::Left;
const KEY_MOVE_RIGHT: KeyCode = KeyCode::Right;
const KEY_JUMP:       KeyCode = KeyCode::Z;
const KEY_RUN:        KeyCode = KeyCode::A;

#[derive(Default)]
pub struct Player {
    flip_x: bool,
    jumping: bool,
    walk_timer: f32,

    pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,
    gravity: f32,
    
    // Constants
    walk_speed:   f32,
    run_speed:    f32,
    jump_height:  f32,
    jump_gravity: f32,
    fall_gravity: f32,
}

impl Player {
    pub fn new(pos: Vec2) -> Player {
        Player {
            flip_x: false,
            jumping: false,
            walk_timer: 0.0,

            pos,

            walk_speed:     70.0,
            run_speed:     110.0,
            jump_height:   250.0,
            jump_gravity:  512.0,
            fall_gravity: 1024.0,

            ..Default::default()
        }
    }

    pub fn pos(&self)    -> Vec2 { self.pos }
    pub fn flip_x(&self) -> bool { self.flip_x }

    pub fn update(&mut self, delta: f32, recip_mul: f32) -> usize {
        let grounded = self.pos.y <= 16.0;

        // Jumping
        if is_key_pressed(KEY_JUMP) && grounded {
            self.vel.y = self.jump_height; 
            self.jumping = true;
        }

        if !is_key_down(KEY_JUMP) {
            self.jumping = false;
        }
        let target_gravity = match is_key_down(KEY_JUMP) && self.jumping {
            true  => self.jump_gravity,
            false => self.fall_gravity,
        } * (self.vel.x.recip() * recip_mul).abs().clamp(0.5, 1.0);
        self.gravity = self.gravity.lerp(target_gravity, delta * 100.0);
        self.vel.y -= self.gravity * delta;

        // println!("{:?}", self.gravity);

        // Moving
        let target_speed = match is_key_down(KEY_RUN) {
            false => self.walk_speed,
            true  => self.run_speed,
        };

        if is_key_pressed(KEY_MOVE_LEFT)  { self.target_x_vel = -target_speed; self.flip_x = true;  }
        if is_key_pressed(KEY_MOVE_RIGHT) { self.target_x_vel =  target_speed; self.flip_x = false; }
        if !is_key_down(KEY_MOVE_LEFT) && !is_key_down(KEY_MOVE_RIGHT) && grounded { self.target_x_vel = 0.0; }

        self.vel.x = self.vel.x.to_target(self.target_x_vel, delta * 400.0 * if grounded { 1.0 } else { 2.0 });

        self.walk_timer = (self.walk_timer + self.vel.x.abs() / 1000.0).rem_euclid(1.0);

        if self.vel.x == 0.0 { self.walk_timer = 0.0; }

        // println!("{:?}", self.vel.x);

        // println!("---\n{:?}\n{:?}\n", self.vel.x, self.target_x_vel);
        
        self.pos += self.vel * delta;

        let grounded = self.grounded();
        if grounded {
            self.pos.y = 16.0;
            self.jumping = false;
        }
        
        match (self.vel.x.abs() == 0.0, self.vel.x.abs() > 75.0, grounded, self.vel.y > -50.0) {
            (true, _,  true, _)  => 0,
            (_, false, true, _)  => if self.walk_timer >= 0.5 { 0 } else { 2 }, 
            (_, true,  true, _)  => if self.walk_timer >= 0.5 { 1 } else { 3 }, 
            (_, _, false, true)  => 4,
            (_, _, false, false) => 5,
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