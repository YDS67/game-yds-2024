use macroquad::prelude::*;

mod assets;
mod camera;
mod map;
mod player;
mod settings;
mod shaders;
mod stage;

fn window_conf() -> Conf {
    Conf {
        window_title: "Awesome game".to_owned(),
        high_dpi: true,
        window_width: settings::WIDTH,
        window_height: settings::HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let ass = assets::Ass::load().await;
    let mut player = player::Player::new();

    let mut game_map = map::GameMap::new(&ass);

    let mut img = Image {
        bytes: ass.wall_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let col = game_map.wall_array[i][settings::MAPSIZE - j - 1];
            if col == 255 {
                img.set_pixel(i as u32, j as u32, BLANK);
            }
        }
    }
    let walls_texture = Texture2D::from_image(&img);
    walls_texture.set_filter(FilterMode::Nearest);

    camera::find_visible_tiles(&mut game_map, &player);
    let mut depth_buffer = camera::DepthBuffer::generate(&game_map, &player);

    img = Image {
        bytes: ass.floor_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            if game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                img.set_pixel(i as u32, j as u32, BLUE);
            }
        }
    }
    let mut floor_texture = Texture2D::from_image(&img);
    floor_texture.set_filter(FilterMode::Nearest);

    let InternalGlContext {
        quad_context: ctx, ..
    } = unsafe { get_internal_gl() };

    let t_par = TextParams {
        font_size: 30,
        font: Some(&ass.font_main),
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
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            let stage = stage::Stage::new(ctx, &ass, &depth_buffer);

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            gl.quad_context
                .apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
                    playerpos: (player.position.x, player.position.y, player.position.z),
                    playerdir: (
                        player.position.ax,
                        player.position.ay,
                        player.position.bz,
                        player.position.bxy,
                    ),
                }));
            gl.quad_context.draw(0, &stage.num*6, 1);

            gl.quad_context.end_render_pass();
        }

        camera::find_visible_tiles(&mut game_map, &player);
        depth_buffer = camera::DepthBuffer::generate(&game_map, &player);

        img = Image {
            bytes: ass.floor_image.as_raw().to_owned(),
            width: settings::MAPSIZE as u16,
            height: settings::MAPSIZE as u16,
        };
        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                    let d = game_map.dist_field[i][settings::MAPSIZE - j - 1];
                    let b = 255 - (d as f32 / game_map.dmax as f32 * 255.0) as u8;
                    let col = Color::from_rgba(255 - b, 255 - b, b, 255);
                    img.set_pixel(i as u32, j as u32, col);
                }
            }
        }
        floor_texture = Texture2D::from_image(&img);
        floor_texture.set_filter(FilterMode::Nearest);

        if request_map {
            draw_map(&walls_texture, &floor_texture);
            player.draw();
        }

        draw_words(&t_par, &player);

        player.walk(&game_map);

        next_frame().await
    }
}

fn draw_words(t_par: &TextParams, player: &player::Player) {
    draw_rectangle(10.0, 10.0, 220.0, 140.0, WHITE);
    draw_rectangle_lines(10.0, 10.0, 220.0, 140.0, 4.0, BLACK);
    draw_text_ex("Awesome game", 20.0, 40.0, t_par.clone());
    let fps = get_fps();
    let mut fps_display = fps;
    if fps > 50 && fps < 70 {
        fps_display = 60
    }
    draw_text_ex(
        &format!("FPS is {}", fps_display),
        20.0,
        70.0,
        t_par.to_owned(),
    );
    draw_text_ex("Player position:", 20.0, 100.0, t_par.to_owned());
    draw_text_ex(
        &format!("({:.1},{:.1})", player.position.x, player.position.y),
        20.0,
        130.0,
        t_par.to_owned(),
    );
}

fn draw_map(walls_texture: &Texture2D, floor_texture: &Texture2D) {
    let size = settings::MAPSIZE as f32 * settings::TILESCREENSIZE;
    draw_texture_ex(
        &floor_texture,
        settings::MAPOFFSETX,
        settings::HEIGHTF - 10.0 - size,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
    draw_texture_ex(
        &walls_texture,
        settings::MAPOFFSETX,
        settings::HEIGHTF - 10.0 - size,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
}
