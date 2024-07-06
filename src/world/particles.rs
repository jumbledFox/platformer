use macroquad::{color::WHITE, math::{Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

pub enum Particle {
    BrokenBlock { pos: Vec2, vel: Vec2, corner: u8 },
}

impl Particle {
    pub fn update(&mut self, delta: f32) {
        match self {
            &mut Self::BrokenBlock { mut pos, mut vel, corner: _ } => {
                pos += vel * delta;
                vel.y = (vel.y + 1100.0).min(500.0);
            }
        }
    }

    pub fn draw(&self, tex: &Texture2D) {
        let (x, y, source) = match self {
            &Self::BrokenBlock { pos, vel: _, corner } => {
                (
                    pos.x,
                    pos.y,
                    Rect::new((corner%2) as f32 * 11.0, (corner/2) as f32 * 11.0, 11.0, 11.0)
                )
            }
        };

        draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
            source: Some(source),
            ..Default::default()
        });
    }
}