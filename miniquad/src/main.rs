use macroquad::prelude::*;

mod assets;
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
    let stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        stage::Stage::new(ctx)
    }
    .await;

    let t_par = TextParams {
        font_size: 30,
        font: Some(&stage.font),
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

        draw_words(&t_par);

        next_frame().await
    }
}

fn draw_words (t_par: &TextParams) {
    
    draw_rectangle(0.0, 0.0, 220.0, 80.0, LIGHTGRAY);
    draw_rectangle_lines(0.0, 0.0, 220.0, 80.0, 2.0, BLACK);
    draw_text_ex("Awesome game", 10.0, 30.0, t_par.clone());
    draw_text_ex(&format!("FPS is {}0", (get_fps()+2) / 10), 10.0, 60.0, t_par.to_owned());
}