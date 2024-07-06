use std::{thread, time::Duration};

use macroquad::{camera::{set_camera, Camera2D}, color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect, Vec2}, miniquad::window::{dpi_scale, screen_size}, text::draw_text, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use particles::Particles;
use stage::Stage;
use player::Player;

pub mod particles;
pub mod world;
pub mod stage;
pub mod player;

const VIEW_WIDTH:  i32 = 352;
const VIEW_HEIGHT: i32 = 224;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Platformer"),
        window_width:  VIEW_WIDTH  * 4,
        window_height: VIEW_HEIGHT * 4,
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

        clear_background(Color::from_hex(0x6dcaff));
        
        let view_size = vec2(VIEW_WIDTH as f32, VIEW_HEIGHT as f32);
        let window_size = vec2(screen_size().0, screen_size().1);
        
        let scale = (window_size / view_size).min_element().floor().max(1.0);
        let remaining_size = (window_size / scale) - view_size; 
        let camera_size = view_size + remaining_size;
    
        set_camera(
            &Camera2D::from_display_rect(Rect::new(
                0.0,
                camera_size.y,
                camera_size.x,
               -camera_size.y,
            ))
        );
        draw_texture_ex(&tiles_texture, 0.0, 0.0, WHITE, DrawTextureParams {
            source: Some(Rect::new(0.0, 0.0, 16.0, 16.0)),
            // dest_size: Some(vec2(32.0,32.0) * 4.0),
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
            // zoom: scale / screen_size,
            // target: view_area / 2.0,
            ..Default::default()
        });
        draw_text(&format!("FPS: {:?}", delta.recip()), 0.0, 10.0, 16.0, Color::from_hex(0xFFFFFF));

        if lag {
            thread::sleep(Duration::from_millis(100));
        }
        next_frame().await
    }
}
