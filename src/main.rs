use std::{thread, time::Duration};

use macroquad::{camera::{set_camera, Camera2D}, color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect}, text::draw_text, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use particles::Particles;
use stage::Stage;
use player::Player;

pub mod particles;
pub mod stage;
pub mod player;

const SCALE: i32 = 8;
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
    let particles_texture = Texture2D::from_file_with_format(include_bytes!("../res/particles.png"), None);
    particles_texture.set_filter(macroquad::texture::FilterMode::Nearest);

    let mut stage = Stage::new(vec![
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        7,0,0,0,0,0,0,0,3,3,3,3,3,3,0,3,0,3,0,3,0,3,0,3,0,3,0,3,3,3,3,3,
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0,0,0,0,
        7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        7,0,0,0,0,0,8,8,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        4,5,5,5,6,0,0,3,3,3,3,3,3,3,3,3,0,0,3,0,0,0,0,8,9,8,9,8,0,0,0,3,
        0,0,7,0,0,0,0,3,3,3,3,3,3,3,3,3,0,3,3,3,0,0,0,0,0,0,0,0,0,0,0,3,
        0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,0,0,0,0,0,0,0,0,0,0,3,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,
        2,2,2,2,2,2,2,2,2,2,2,0,0,2,2,0,0,0,0,0,2,2,2,0,0,0,2,2,2,0,2,2,
        2,2,2,2,2,2,2,2,2,2,2,0,0,2,2,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,2,2,
        2,2,2,2,2,2,2,2,2,2,2,0,0,2,2,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1,2,2,
    ], 32);
    let mut particles = Particles::default();
    let mut player = Player::new(vec2(2.0, -1.5) * 16.0);
    let mut lag = false;

    loop {
        let delta = get_frame_time();
        player.update(delta, &mut stage, &mut particles);
        if is_key_pressed(macroquad::input::KeyCode::Q) { lag = !lag; }

        particles.update(delta);

        let screen_size = vec2(screen_width(), screen_height());
        let scale = SCALE_F;
        let view_area = screen_size / scale * 2.0;
        
        clear_background(Color::from_hex(0x6dcaff));
        // clear_background(Color::from_hex(0x000000));
        set_camera(&Camera2D {
            zoom: scale / screen_size,
            // target: view_area / 2.0,
            target: vec2(player.pos().x.max(view_area.x / 2.0), view_area.y / 2.0),
            ..Default::default()
        });

        for (y, row) in stage.tiles().chunks(stage.width()).enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 0 { continue; }
                draw_texture_ex(&tiles_texture, x as f32*16.0, y as f32*16.0, WHITE, DrawTextureParams {
                    source: Some(Rect::new((*tile as f32 - 1.0) * 16.0, 0.0, 16.0, 16.0)),
                    ..Default::default()
                });
            }
        }
        
        player.draw(&player_texture);
        particles.draw(&particles_texture);

        set_camera(&Camera2D {
            zoom: scale / screen_size,
            target: view_area / 2.0,
            ..Default::default()
        });
        draw_text(&format!("FPS: {:?}", delta.recip()), 0.0, 10.0, 16.0, Color::from_hex(0xFFFFFF));

        if lag {
            thread::sleep(Duration::from_millis(100));
        }
        next_frame().await
    }
}
