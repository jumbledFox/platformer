use macroquad::{camera::{set_camera, Camera2D}, color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use player::Player;

pub mod player;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Platformer"),
        high_dpi: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf())]

async fn main() {
    let player_texture = Texture2D::from_file_with_format(include_bytes!("sprites.png"), None);
    player_texture.set_filter(macroquad::texture::FilterMode::Nearest);
    let tiles_texture = Texture2D::from_file_with_format(include_bytes!("tiles.png"), None);
    tiles_texture.set_filter(macroquad::texture::FilterMode::Nearest);

    let mut p = Player::new(vec2(32.0, 32.0));
    let mut helmet = false;

    loop {
        let delta = get_frame_time();
        let sprite = p.update(delta);
        if is_key_pressed(macroquad::input::KeyCode::Q) { helmet = !helmet; }


        let screen_size = vec2(screen_width(), screen_height());
        let scale = 5.0;
        let view_area = screen_size / (scale / 2.0);
        
        clear_background(Color::from_hex(0x6dcaff));
        // clear_background(BLACK);
        set_camera(&Camera2D {
            zoom: scale / screen_size,
            target: view_area / 2.0,
            ..Default::default()
        });


        for i in 0..(view_area.x / 16.0).ceil() as usize {
            draw_texture_ex(&tiles_texture, i as f32 * 16.0, view_area.y - 16.0, WHITE, DrawTextureParams {
                source: Some(Rect::new(9.0 * 16.0, 0.0, 16.0, 16.0)),
                ..Default::default()
            });

            if i % 12 == 0 {
                for j in 2..5 {
                    draw_texture_ex(&tiles_texture, i as f32 * 16.0, view_area.y - 16.0 * j as f32, WHITE, DrawTextureParams {
                        source: Some(Rect::new(6.0 * 16.0, 0.0, 16.0, 16.0)),
                        ..Default::default()
                    });
                }
            }
            if i % 12 == 6 {
                for j in 2..4 {
                    draw_texture_ex(&tiles_texture, i as f32 * 16.0, view_area.y - 16.0 * j as f32, WHITE, DrawTextureParams {
                        source: Some(Rect::new(6.0 * 16.0, 0.0, 16.0, 16.0)),
                        ..Default::default()
                    });
                }
            }
            if i % 12 == 3 {
                draw_texture_ex(&tiles_texture, i as f32 * 16.0, view_area.y - 16.0 * 4.0, WHITE, DrawTextureParams {
                    source: Some(Rect::new(7.0 * 16.0, 0.0, 16.0, 16.0)),
                    ..Default::default()
                });
            }
        }
        draw_texture_ex(&player_texture, p.pos().x, view_area.y - p.pos().y - 32.0, WHITE, DrawTextureParams {
            flip_x: p.flip_x(),
            source: Some(Rect::new(sprite as f32 * 16.0, if helmet { 32.0 } else { 0.0 }, 16.0, 32.0)),
            ..Default::default()
        });
        next_frame().await
    }
}
