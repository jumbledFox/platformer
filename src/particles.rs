use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, rand::gen_range, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

const GRAVITY: f32 = 1100.0;
const MAX_FALL_SPEED: f32 = 500.0;

#[derive(PartialEq)]
pub enum ParticleKind {
    BrokenTile { corner: usize },
}

pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    kind: ParticleKind,
}

impl Particle {
    pub fn update(&mut self, delta: f32) {
        match self.kind {
            ParticleKind::BrokenTile { corner: _ } => {
                self.pos += self.vel * delta;
                self.vel.y = (self.vel.y + GRAVITY * delta).min(MAX_FALL_SPEED);
            }
        }
    }

    pub fn draw(&self, tex: &Texture2D) {
        let source = match &self.kind {
            ParticleKind::BrokenTile { corner } => {
                Rect::new((corner%2) as f32 * 11.0, (corner/2) as f32 * 11.0, 11.0, 11.0)
            } 
        };

        draw_texture_ex(tex, self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            source: Some(source),
            ..Default::default()
        });
    }
}

#[derive(Default)]
pub struct Particles {
    particles: Vec<Particle>,
}

impl Particles {
    pub fn update(&mut self, delta: f32) {
        for p in &mut self.particles {
            p.update(delta);
        }
    }

    pub fn draw(&self, tex: &Texture2D) {
        for p in &self.particles {
            p.draw(tex);
        }
    }

    pub fn break_tile(&mut self, pos: Vec2) {
        let top_vel_x = || gen_range( 90.0, 130.0);
        let bot_vel_x = || gen_range( 60.0,  80.0);
        let top_vel_y = || gen_range(150.0, 180.0);
        let bot_vel_y = || gen_range(100.0, 150.0);

        self.particles.push( Particle { pos: pos + vec2(-1.0, -1.0), vel: vec2(-top_vel_x(), -top_vel_y()), kind: ParticleKind::BrokenTile { corner: 0 }});
        self.particles.push( Particle { pos: pos + vec2( 6.0, -1.0), vel: vec2( top_vel_x(), -top_vel_y()), kind: ParticleKind::BrokenTile { corner: 1 }});
        self.particles.push( Particle { pos: pos + vec2(-1.0,  6.0), vel: vec2(-bot_vel_x(), -bot_vel_y()), kind: ParticleKind::BrokenTile { corner: 2 }});
        self.particles.push( Particle { pos: pos + vec2( 6.0,  6.0), vel: vec2( bot_vel_x(), -bot_vel_y()), kind: ParticleKind::BrokenTile { corner: 3 }});
    }
}