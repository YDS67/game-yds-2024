use macroquad::prelude::*;
use miniquad;

mod assets;
mod camera;
mod map;
mod player;
mod settings;
mod shaders;
mod stage;
mod mesh;

fn window_conf() -> Conf {
    Conf {
        window_title: "Raycasting + GPU rendering".to_owned(),
        high_dpi: true,
        window_width: settings::WIDTH,
        window_height: settings::HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_main = load_ttf_font("resources/times.ttf").await.unwrap();
    
    let mut stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        stage::Stage::new(ctx)
    };

    let mut img = Image {
        bytes: stage.ass.wall_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let col = stage.game_map.wall_array[i][settings::MAPSIZE - j - 1];
            if col == 255 {
                img.set_pixel(i as u32, j as u32, BLANK);
            }
        }
    }
    let walls_texture = Texture2D::from_image(&img);
    walls_texture.set_filter(FilterMode::Nearest);

    img = Image {
        bytes: stage.ass.floor_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            if stage.game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                img.set_pixel(i as u32, j as u32, BLUE);
            }
        }
    }
    let mut floor_texture = Texture2D::from_image(&img);
    floor_texture.set_filter(FilterMode::Nearest);

    let t_par = TextParams {
        font_size: 30,
        font: Some(&font_main),
        color: BLACK,
        ..Default::default()
    };

    let mut request_map = false;

    loop {
        clear_background(Color::from_rgba(135, 206, 235, 255));

        if is_key_pressed(KeyCode::M) {
            if request_map {
                request_map = false;
            } else {
                request_map = true
            }
        }

        {
            let gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            //gl.flush();

            stage.update(gl.quad_context);

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context.apply_bindings(&stage.bindings);

            gl.quad_context
                .apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
                    playerpos: (stage.player.position.x, stage.player.position.y, stage.player.position.z),
                    playerdir: (
                        stage.player.position.ax,
                        stage.player.position.ay,
                        stage.player.position.bz,
                        stage.player.position.bxy,
                    ),
                }));
            gl.quad_context.draw(0, &stage.num*6, 1);

            gl.quad_context.end_render_pass();
        }

        img = Image {
            bytes: stage.ass.floor_image.as_raw().to_owned(),
            width: settings::MAPSIZE as u16,
            height: settings::MAPSIZE as u16,
        };
        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if stage.game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                    let d = stage.game_map.wall_dist[i][settings::MAPSIZE - j - 1] / stage.game_map.dmax;
                    let b = 255 - d as u8;
                    let col = Color::from_rgba(255 - b, 255 - b, b, 255);
                    img.set_pixel(i as u32, j as u32, col);
                }
            }
        }
        floor_texture = Texture2D::from_image(&img);
        floor_texture.set_filter(FilterMode::Nearest);

        if request_map {
            draw_map(&walls_texture, &floor_texture);
            stage.player.draw();
        }

        draw_words(&t_par, &stage.depth_buffer);

        next_frame().await
    }
}

fn draw_words(t_par: &TextParams, depth_buffer: &camera::DepthBuffer) {
    draw_rectangle(10.0, 10.0, 220.0, 120.0, WHITE);
    draw_rectangle_lines(10.0, 10.0, 220.0, 120.0, 4.0, BLACK);
    let fps = get_fps();
    let mut fps_display = fps;
    if fps > 50 && fps < 70 {
        fps_display = 60
    }
    draw_text_ex(
        &format!("FPS is {}", fps_display),
        20.0,
        40.0,
        t_par.to_owned(),
    );
    draw_text_ex("Quads drawn:", 20.0, 75.0, t_par.to_owned());
    draw_text_ex(
        &format!("{}", depth_buffer.len),
        20.0,
        110.0,
        t_par.to_owned(),
    );
}

fn draw_map(walls_texture: &Texture2D, floor_texture: &Texture2D) {
    let size = settings::MAPSIZE as f32 * settings::TILESCREENSIZE;
    draw_texture_ex(
        &floor_texture,
        settings::MAPOFFSETX,
        20.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
    draw_texture_ex(
        &walls_texture,
        settings::MAPOFFSETX,
        20.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
}
