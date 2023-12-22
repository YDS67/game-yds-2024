use macroquad::prelude::*;

mod raw_miniquad;
mod shaders;

#[macroquad::main("Rendering a quad in macroquad")]
async fn main() {
    let stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        raw_miniquad::Stage::new(ctx)
    };

    loop {
        clear_background(LIGHTGRAY);

        // Render some primitives in camera space

        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

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
                    .apply_uniforms(miniquad::UniformsSource::table(
                        &shaders::Uniforms {
                            offset: (0.0, 0.0),
                        },
                    ));
                gl.quad_context.draw(0, 6, 1);

            gl.quad_context.end_render_pass();
        }

        // Back to screen space, render some text

        set_default_camera();
        draw_text("Hello, happy tree from WGPU tutorial", 200.0, 50.0, 30.0, BLACK);
        draw_text(&format!("FPS is {}", get_fps()), 200.0, 80.0, 30.0, BLACK);
        next_frame().await
    }
}