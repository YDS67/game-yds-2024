#![windows_subsystem = "windows"]

use macroquad::prelude as mqp;
use miniquad::{self, conf::Platform};

mod assets;
mod camera;
mod map;
mod mesh;
mod player;
mod settings;
mod shaders;
mod stage;

fn window_conf() -> mqp::Conf {
    let mut conf = mqp::Conf {
        window_title: "Raycasting + GPU rendering".to_owned(),
        window_width: settings::WIDTH0,
        window_height: settings::HEIGHT0,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(-1);
    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_main = mqp::load_ttf_font("resources/times.ttf").await.unwrap();

    let mut settings = settings::Settings::init();

    let mut gl = unsafe { mqp::get_internal_gl() };

    let mut stage = stage::Stage::new(gl.quad_context, &settings);

    let mut img = mqp::Image {
        bytes: stage.ass.wall_image_bot.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let col = stage.game_map.wall_array[i][settings::MAPSIZE - j - 1];
            if col == 255 {
                img.set_pixel(i as u32, j as u32, mqp::BLANK);
            }
        }
    }
    let walls_texture = mqp::Texture2D::from_image(&img);
    walls_texture.set_filter(mqp::FilterMode::Nearest);

    img = mqp::Image {
        bytes: stage.ass.floor_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            if stage.game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                img.set_pixel(i as u32, j as u32, mqp::BLUE);
            }
        }
    }
    let mut floor_texture = mqp::Texture2D::from_image(&img);
    floor_texture.set_filter(mqp::FilterMode::Nearest);

    let t_par = mqp::TextParams {
        font_size: 30,
        font: Some(&font_main),
        color: mqp::BLACK,
        ..Default::default()
    };

    //let mut map_state;

    loop {
        if mqp::is_key_pressed(mqp::KeyCode::M) {
            if settings.draw_map {
                settings.draw_map = false;
            } else {
                settings.draw_map = true
            }
        }

        if mqp::is_key_pressed(mqp::KeyCode::F) {
            settings.full_screen = true;
            mqp::set_fullscreen(true);
        }

        if mqp::is_key_pressed(mqp::KeyCode::Escape) {
            break;
        }

        settings.screen_change(mqp::screen_width(), mqp::screen_height());
        settings.delta_time = 1.0/(mqp::get_fps() as f32);
        settings.player_speed = 12.0*settings.delta_time;
        
        // Ensure that macroquad's shapes are not going to be lost
        {
            //gl = unsafe { mqp::get_internal_gl() };

            gl.flush();

            gl.quad_context.clear(Some((0.5294118, 0.8078431, 0.9215686, 1.0000000)), None, None);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::clear_color(
                    0.0, 0.0, 0.0, 1.0000000,
                ));

            stage.update(gl.quad_context, &settings);
        }

        img = mqp::Image {
            bytes: stage.ass.floor_image.as_raw().to_owned(),
            width: settings::MAPSIZE as u16,
            height: settings::MAPSIZE as u16,
        };
        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if stage.game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                    let b = 255;
                    let col = mqp::Color::from_rgba(255 - b, 255 - b, b, 255);
                    img.set_pixel(i as u32, j as u32, col);
                }
            }
        }
        floor_texture = mqp::Texture2D::from_image(&img);
        floor_texture.set_filter(mqp::FilterMode::Nearest);

        if settings.draw_map {
            draw_map(&walls_texture, &floor_texture, &settings);
            stage.player.draw(&settings);
        }

        draw_words(&t_par, &stage.depth_buffer);

        mqp::next_frame().await
    }
}

fn draw_words(t_par: &mqp::TextParams, depth_buffer: &camera::DepthBuffer) {
    mqp::draw_rectangle(10.0, 10.0, 220.0, 120.0, mqp::WHITE);
    mqp::draw_rectangle_lines(10.0, 10.0, 220.0, 120.0, 4.0, mqp::BLACK);
    let fps = mqp::get_fps();
    let fps_display = fps;
    // if fps > 50 && fps < 70 {
    //     fps_display = 60
    // }
    mqp::draw_text_ex(
        &format!("FPS is {}", fps_display),
        20.0,
        40.0,
        t_par.to_owned(),
    );
    mqp::draw_text_ex("Quads drawn:", 20.0, 75.0, t_par.to_owned());
    mqp::draw_text_ex(
        &format!("{}", depth_buffer.len*2),
        20.0,
        110.0,
        t_par.to_owned(),
    );
}

fn draw_map(
    walls_texture: &mqp::Texture2D,
    floor_texture: &mqp::Texture2D,
    settings: &settings::Settings,
) {
    let size = settings::MAPSIZE as f32 * settings.tile_screen_size;
    mqp::draw_texture_ex(
        &floor_texture,
        settings.map_offset_x,
        settings.map_offset_y,
        mqp::WHITE,
        mqp::DrawTextureParams {
            dest_size: Some(mqp::vec2(size, size)),
            ..Default::default()
        },
    );
    mqp::draw_texture_ex(
        &walls_texture,
        settings.map_offset_x,
        settings.map_offset_y,
        mqp::WHITE,
        mqp::DrawTextureParams {
            dest_size: Some(mqp::vec2(size, size)),
            ..Default::default()
        },
    );
}
