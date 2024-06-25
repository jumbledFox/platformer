use macroquad::{camera::{set_camera, Camera2D}, color::{Color, BLACK, WHITE}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect}, shapes::draw_rectangle, texture::{draw_texture_ex, load_image, DrawTextureParams, Image, Texture2D}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}};


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

    let height = 32.0 + 16.0;

    let (mut x, mut y): (f32, f32) = (0.0, 0.0);
    let mut vel: (f32, f32) = (0.0, 0.0);
    let mut run_d: f32 = 0.0;
    
    let mut walk_frame_timer = 0.0;
    let mut walk_frame;

    let mut facing_right = true;

    let jump_height: f32 = 2.0;

    let air_spd = 1.5;
    let air_acc = 15.0;

    let walk_spd = 1.5;
    let walk_acc = 5.0;

    let run_spd = 3.0; 
    let run_acc = 10.0; 

    loop {
        let delta = get_frame_time();

        let mut grounded = y.max(height) == height;
        let running = is_key_down(KeyCode::LeftShift);

        let (spd, mut acc) = match (grounded, running) {
            (true, false) => (walk_spd, walk_acc),
            (true, true)  => (run_spd,  run_acc),
            _             => (air_spd,  air_acc),
        };

        if is_key_down(KeyCode::A) { run_d = -spd; }
        if is_key_down(KeyCode::D) { run_d =  spd; }

        if !is_key_down(KeyCode::A) && !is_key_down(KeyCode::D) && grounded {
            run_d = 0.0;
            acc *= 1.5;
        }
        
        if vel.0 < run_d { vel.0 += delta * acc; }
        if vel.0 > run_d { vel.0 -= delta * acc; }
        if (vel.0 - run_d).abs() < 0.2 { vel.0 = run_d; }

        if vel.0 != 0.0 {
            facing_right = vel.0.is_sign_positive();
        }

        if grounded && vel.0 != 0.0 {
            let walk_cycle_len = 0.5;
            walk_frame_timer = (walk_frame_timer + vel.0 * delta).rem_euclid(walk_cycle_len);
            walk_frame = walk_frame_timer <= walk_cycle_len / 2.0;
        } else {
            walk_frame_timer = 0.0;
            walk_frame = false;
        }

        println!("{:?}", vel.0);

        let sprite = match (grounded, vel.1 > -1.0) {
            (true, _) => (if vel.0.abs() > 2.85 {
                1
            } else {
                0
             }) + if walk_frame { 2 } else { 0 },
            (false, true)  => 4,
            (false, false) => 5,
        };

        
        let running_amount = (vel.0.abs() - walk_spd).max(0.0).abs() / (run_spd-walk_spd);

        if is_key_pressed(KeyCode::Space) && grounded { vel.1 = jump_height + 0.5 * running_amount; grounded = false; }
        
        let gravity = match is_key_down(KeyCode::Space) {
            true  => 4.0,
            false => 8.0,
        };
        vel.1 -= gravity * delta;
        if grounded {
            y = height;
            vel.1 = 0.0;
        }

        y += vel.1;

        x += vel.0;
        y += vel.1;

        // println!("---\n{:?}\n{:?}\n{:?}\n{:?}", x, y, vel, vel_d);

        
        
        let screen_size = vec2(screen_width(), screen_height());
        let scale = 5.0;
        let view_area = screen_size / scale;
        
        clear_background(Color::from_hex(0x6dcaff));
        // clear_background(BLACK);
        set_camera(&Camera2D {
            zoom: scale / screen_size,
            target: view_area,
            ..Default::default()
        });

        for i in 0..32 {
            draw_texture_ex(&tiles_texture, i as f32 * 16.0, view_area.y * 2.0 - 16.0, WHITE, DrawTextureParams {
                source: Some(Rect::new(8.0 * 16.0, 0.0, 16.0, 16.0)),
                ..Default::default()
            });
        }

        // draw_rectangle(x, view_area.y * 2.0 - y, 16.0, height, WHITE);
        draw_texture_ex(&player_texture, x, view_area.y * 2.0 - y, WHITE, DrawTextureParams {
            flip_x: !facing_right,
            source: Some(Rect::new(sprite as f32 * 16.0, 0.0, 16.0, 32.0)),
            ..Default::default()
        });
        next_frame().await
    }
}
