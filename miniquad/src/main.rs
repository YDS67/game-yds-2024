use macroquad::prelude::*;

mod assets;
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

    let _game_map = map::GameMap::new(&ass.map_image);
    let img = Image {
        bytes: ass.map_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    let map_texture = Texture2D::from_image(&img);
    map_texture.set_filter(FilterMode::Nearest);

    let stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        stage::Stage::new(ctx, &ass)
    }
    .await;

    let t_par = TextParams {
        font_size: 30,
        font: Some(&ass.font_main),
        color: BLACK,
        ..Default::default()
    };

    loop {
        clear_background(WHITE);

        // Render some primitives in camera space

        set_default_camera();

        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            gl.quad_context
                .apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
                    offset: (0.0, 0.0),
                }));
            gl.quad_context.draw(0, 6, 1);

            gl.quad_context.end_render_pass();
        }

        draw_map(&map_texture);
        player.draw();
        draw_words(&t_par);

        player.walk();

        next_frame().await
    }
}

fn draw_words(t_par: &TextParams) {
    draw_rectangle(10.0, 10.0, 220.0, 80.0, LIGHTGRAY);
    draw_rectangle_lines(10.0, 10.0, 220.0, 80.0, 4.0, BLACK);
    draw_text_ex("Awesome game", 20.0, 40.0, t_par.clone());
    draw_text_ex(
        &format!("FPS is {}", get_fps()),
        20.0,
        70.0,
        t_par.to_owned(),
    );
}

fn draw_map(map_texture: &Texture2D) {
    draw_texture_ex(
        &map_texture,
        10.0,
        screen_height() - 10.0 - 256.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(256.0, 256.0)),
            ..Default::default()
        },
    );
}
