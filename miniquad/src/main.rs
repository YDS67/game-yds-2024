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

    let game_map = map::GameMap::new(&ass);
    let mut img = Image {
        bytes: ass.wall_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let col = game_map.wall_array[i][settings::MAPSIZE-j-1];
            if col == 255 {
                img.set_pixel(i as u32, j as u32, BLANK);
            }
        }
    }
    let walls_texture = Texture2D::from_image(&img);
    walls_texture.set_filter(FilterMode::Nearest);

    img = Image {
        bytes: ass.floor_image.as_raw().to_owned(),
        width: settings::MAPSIZE as u16,
        height: settings::MAPSIZE as u16,
    };
    let floor_texture = Texture2D::from_image(&img);
    floor_texture.set_filter(FilterMode::Nearest);

    let thickness: f32 = 50.0;

    // let stage = {
    //     let InternalGlContext {
    //         quad_context: ctx, ..
    //     } = unsafe { get_internal_gl() };

    //     stage::Stage::new(ctx, &ass)
    // }
    // .await;

    let t_par = TextParams {
        font_size: 30,
        font: Some(&ass.font_main),
        color: BLACK,
        ..Default::default()
    };

    loop {
        clear_background(Color::from_rgba(135, 206, 235, 255));

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if game_map.wall_array[i][j] < 255 {
                    let wall_x = i as f32 + 0.5;
                    let wall_y = j as f32 + 0.5;
                    let wall_z = 0.5;
                    let proj = project_point(&player, wall_x, wall_y, wall_z);
                    let visible = proj.visible;
                    let d = proj.d;
                    if visible
                    {
                        let proj1 = project_point(&player, wall_x + 0.5, wall_y + 0.5, wall_z + 0.5);
                        let proj2 = project_point(&player, wall_x - 0.5, wall_y + 0.5, wall_z + 0.5);
                        let proj3 = project_point(&player, wall_x + 0.5, wall_y - 0.5, wall_z + 0.5);
                        let proj4 = project_point(&player, wall_x + 0.5, wall_y + 0.5, wall_z - 0.5);
                        let proj5 = project_point(&player, wall_x - 0.5, wall_y - 0.5, wall_z + 0.5);
                        let proj6 = project_point(&player, wall_x - 0.5, wall_y + 0.5, wall_z - 0.5);
                        let proj7 = project_point(&player, wall_x + 0.5, wall_y - 0.5, wall_z - 0.5);
                        let proj8 = project_point(&player, wall_x - 0.5, wall_y - 0.5, wall_z - 0.5);
                        let col = BLACK;
                        draw_line(proj1.u, proj1.v, proj2.u, proj2.v, thickness/d, col);
                        draw_line(proj1.u, proj1.v, proj3.u, proj3.v, thickness/d, col);
                        draw_line(proj1.u, proj1.v, proj4.u, proj4.v, thickness/d, col);
                        draw_line(proj8.u, proj8.v, proj7.u, proj7.v, thickness/d, col);
                        draw_line(proj8.u, proj8.v, proj6.u, proj6.v, thickness/d, col);
                        draw_line(proj8.u, proj8.v, proj5.u, proj5.v, thickness/d, col);
                        draw_line(proj2.u, proj2.v, proj5.u, proj5.v, thickness/d, col);
                        draw_line(proj2.u, proj2.v, proj6.u, proj6.v, thickness/d, col);
                        draw_line(proj3.u, proj3.v, proj5.u, proj5.v, thickness/d, col);
                        draw_line(proj3.u, proj3.v, proj7.u, proj7.v, thickness/d, col);
                        draw_line(proj4.u, proj4.v, proj6.u, proj6.v, thickness/d, col);
                        draw_line(proj4.u, proj4.v, proj7.u, proj7.v, thickness/d, col);
                    }
                } else {
                    let wall_x = i as f32 + 0.5;
                    let wall_y = j as f32 + 0.5;
                    let wall_z = 0.0;
                    let proj = project_point(&player, wall_x, wall_y, wall_z);
                    let visible = proj.visible;
                    let d = proj.d;
                    if visible
                    {
                        let proj4 = project_point(&player, wall_x + 0.5, wall_y + 0.5, wall_z );
                        let proj6 = project_point(&player, wall_x - 0.5, wall_y + 0.5, wall_z);
                        let proj7 = project_point(&player, wall_x + 0.5, wall_y - 0.5, wall_z);
                        let proj8 = project_point(&player, wall_x - 0.5, wall_y - 0.5, wall_z);
                        let col = GRAY;
                        draw_line(proj8.u, proj8.v, proj7.u, proj7.v, 0.5*thickness/d, col);
                        draw_line(proj8.u, proj8.v, proj6.u, proj6.v, 0.5*thickness/d, col);
                        draw_line(proj4.u, proj4.v, proj6.u, proj6.v, 0.5*thickness/d, col);
                        draw_line(proj4.u, proj4.v, proj7.u, proj7.v, 0.5*thickness/d, col);
                    }
                }
            }
        }

        // Render some primitives in camera space

        // {
        //     let mut gl = unsafe { get_internal_gl() };

        //     // Ensure that macroquad's shapes are not going to be lost
        //     gl.flush();

        //     gl.quad_context.apply_pipeline(&stage.pipeline);

        //     gl.quad_context
        //         .begin_default_pass(miniquad::PassAction::Nothing);
        //     gl.quad_context.apply_bindings(&stage.bindings);

        //     gl.quad_context
        //         .apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
        //             offset: (0.0, 0.0),
        //         }));
        //     gl.quad_context.draw(0, 6, 1);

        //     gl.quad_context.end_render_pass();
        // }

        draw_map(&walls_texture, &floor_texture);
        player.draw();
        draw_words(&t_par, &player);

        player.walk(&game_map);

        next_frame().await
    }
}

fn draw_words(t_par: &TextParams, player: &player::Player) {
    draw_rectangle(10.0, 10.0, 256.0, 140.0, WHITE);
    draw_rectangle_lines(10.0, 10.0, 256.0, 140.0, 4.0, BLACK);
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
    draw_texture_ex(
        &floor_texture,
        10.0,
        screen_height() - 10.0 - 256.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(256.0, 256.0)),
            ..Default::default()
        },
    );
    draw_texture_ex(
        &walls_texture,
        10.0,
        screen_height() - 10.0 - 256.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(256.0, 256.0)),
            ..Default::default()
        },
    );
}

struct ProjResult {
    u: f32,
    v: f32,
    d: f32,
    visible: bool,
}

fn project_point(player: &player::Player, wall_x: f32, wall_y: f32, wall_z: f32) -> ProjResult {
    let dxy = ((player.position.x - wall_x).powi(2) + (player.position.y - wall_y).powi(2)).sqrt();
    let at = -(player.position.y - wall_y).signum() * (-(player.position.x - wall_x) / dxy).acos();
    let phi = player::angle_round(settings::FOVXY / 2.0 + player.position.a - at);
    let u = settings::WIDTHF / settings::FOVXY * phi;

    let d = ((player.position.x - wall_x).powi(2)
        + (player.position.y - wall_y).powi(2)
        + (player.position.z - wall_z).powi(2))
    .sqrt();
    let bt = settings::PI / 2.0 - (-(player.position.z - wall_z) / d).acos();
    let theta = player::angle_round(settings::ASPECT * settings::FOVXY / 2.0 + player.position.b - bt);
    let v = settings::WIDTHF / settings::FOVXY * theta;
    let visible = phi > 0.0 && phi < settings::FOVXY && theta > 0.0 && theta < settings::ASPECT * settings::FOVXY;
    ProjResult {
        u, v, d, visible,
    }
}