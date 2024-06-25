use macroquad::{input::{is_key_down, is_key_pressed, KeyCode}, math::{FloatExt, Vec2}};

pub struct Player {
    flip_x: bool,
    jumping: bool,

    pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,
    gravity: f32,
    
    // Constants
    walk_speed: f32,
    run_speed:  f32,
    jump_gravity: f32,
    fall_gravity: f32,
}

impl Player {
    pub fn new(pos: Vec2) -> Player {
        Player {
            flip_x: false,
            jumping: false,

            pos,
            vel: Vec2::ZERO,
            target_x_vel: 0.0,
            gravity: 0.0,

            walk_speed: 100.0,
            run_speed:  100.0,
            jump_gravity: 256.0,
            fall_gravity: 512.0,
        }
    }

    pub fn pos(&self)    -> Vec2 { self.pos }
    pub fn flip_x(&self) -> bool { self.flip_x }

    pub fn update(&mut self, delta: f32) -> usize {
        let grounded = self.pos.y <= 16.0;

        // Jumping
        self.vel.y -= self.gravity * delta;

        if is_key_down(KeyCode::Space) && grounded {
            self.vel.y = 150.0; 
            self.jumping = true;
        }

        let target_gravity = match is_key_down(KeyCode::Space) && self.jumping {
            true  => self.jump_gravity,
            false => self.fall_gravity,
        };

        self.gravity = self.gravity.lerp(target_gravity, delta * 100.0);

        println!("{:?}", self.gravity);

        // Moving
        if is_key_pressed(KeyCode::A) { self.target_x_vel = -self.walk_speed; self.flip_x = true;  }
        if is_key_pressed(KeyCode::D) { self.target_x_vel =  self.walk_speed; self.flip_x = false; }
        if !is_key_down(KeyCode::A) && !is_key_down(KeyCode::D) { self.target_x_vel = 0.0; }

        // self.vel.x = match self.target_x_vel self.vel.x.lerp(self.target_x_vel, delta * 5.0);

        self.vel.x = self.vel.x.to_target(self.target_x_vel, delta * 400.0);

        println!("---\n{:?}\n{:?}\n", self.vel.x, self.target_x_vel);

        self.pos += self.vel * delta;

        let grounded = self.grounded();
        if grounded {
            self.pos.y = 16.0;
            self.jumping = false;
        }
        
        match (self.vel.x.abs() == 0.0, self.vel.x.abs() > 75.0, grounded, self.vel.y > -50.0) {
            (true, _,  true, _)  => 0,
            (_, false, true, _)  => 2, 
            (_, true,  true, _)  => 3, 
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