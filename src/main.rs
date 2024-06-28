use macroquad::{camera::{set_camera, Camera2D}, color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use player::Player;

pub mod player;

const SCALE: i32 = 6;
const SCALE_F: f32 = SCALE as f32;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Platformer"),
        window_width:  352 * SCALE,
        window_height: 224 * SCALE,
        high_dpi: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf())]

async fn main() {
    let player_texture = Texture2D::from_file_with_format(include_bytes!("../res/sprites.png"), None);
    player_texture.set_filter(macroquad::texture::FilterMode::Nearest);
    let tiles_texture = Texture2D::from_file_with_format(include_bytes!("../res/tiles.png"), None);
    tiles_texture.set_filter(macroquad::texture::FilterMode::Nearest);

    let mut p = Player::new(vec2(2.0, 1.0) * 16.0);
    let mut s = 0;

    loop {
        let delta = get_frame_time();
        let sprite = p.update(delta);
        if is_key_pressed(macroquad::input::KeyCode::Q) { s = (s+1)%3; }

        let screen_size = vec2(screen_width(), screen_height());
        let scale = SCALE_F;
        let view_area = screen_size / scale * 2.0;
        
        clear_background(Color::from_hex(0x6dcaff));
        // clear_background(Color::from_hex(0x000000));
        set_camera(&Camera2D {
            zoom: scale / screen_size,
            // target: view_area / 2.0,
            target: vec2(p.pos().x, view_area.y / 2.0),
            ..Default::default()
        });

        let draw_tile = |x: f32, y: f32, t: f32| {
            draw_texture_ex(&tiles_texture, x*16.0, y*16.0, WHITE, DrawTextureParams {
                source: Some(Rect::new(t * 16.0, 0.0, 16.0, 16.0)),
                ..Default::default()
            });
        };

        let beg = (p.pos().x / 16.0 - view_area.x / 16.0 / 2.0).floor() as usize;
        let end = beg + (view_area.x / 16.0).ceil() as usize;
        
        let bottom_y = (view_area.y / 16.0 - 1.0).ceil() * 16.0;
        for i in beg..end {
            let x = i as f32;
            let y = bottom_y / 16.0;
            draw_tile(x, y, 9.0);
            match i % 12 {
                0 => { for j in 1..4 { draw_tile(x, y - j as f32, 6.0); } },
                6 => { for j in 1..3 { draw_tile(x, y - j as f32, 6.0); } },
                3 => { draw_tile(x, y - 3.0 as f32,7.0); },
                _=> {},
            }
        }
        
        draw_texture_ex(&player_texture, p.pos().x, bottom_y - p.pos().y - 16.0, WHITE, DrawTextureParams {
            flip_x: p.flip_x(),
            source: Some(Rect::new(sprite as f32 * 16.0, 32.0 * s as f32, 16.0, 32.0)),
            ..Default::default()
        });

        next_frame().await
    }
}
