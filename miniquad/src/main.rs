use macroquad::prelude::*;
use std::cmp::Ordering;

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
    let mut depth_buffer = camera::DepthBuffer::generate(&game_map);

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

        for tile in depth_buffer.visible_tiles {
            if tile[3] == 1 {
                floor_face_draw(&player, &game_map, tile[1], tile[2])
            } else if tile[3] == 2 {
                wall_face_draw(&player, &game_map, tile[1], tile[2])
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
        
        camera::find_visible_tiles(&mut game_map, &player);
        depth_buffer = camera::DepthBuffer::generate(&game_map);


        img = Image {
            bytes: ass.floor_image.as_raw().to_owned(),
            width: settings::MAPSIZE as u16,
            height: settings::MAPSIZE as u16,
        };
        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if game_map.floor_visible[i][settings::MAPSIZE - j - 1] {
                    let d = game_map.floor_dist[i][settings::MAPSIZE - j - 1];
                    let b = 255 - d as u8;
                    let col = Color::from_rgba(255-b, 255-b, b, 255);
                    img.set_pixel(i as u32, j as u32, col);
                }
            }
        }
        floor_texture = Texture2D::from_image(&img);
        floor_texture.set_filter(FilterMode::Nearest);

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

struct ProjResult {
    u: f32,
    v: f32,
    d: f32,
}

fn project_point(player: &player::Player, tile_x: f32, tile_y: f32, tile_z: f32) -> ProjResult {
    let dxy = ((player.position.x - tile_x).powi(2) + (player.position.y - tile_y).powi(2)).sqrt();
    let at = -(player.position.y - tile_y).signum() * (-(player.position.x - tile_x) / dxy).acos();
    let phi = player::angle_round(settings::FOVXY / 2.0 + player.position.a - at);
    let u = settings::WIDTHF / settings::FOVXY * phi;

    let d = ((player.position.x - tile_x).powi(2)
        + (player.position.y - tile_y).powi(2)
        + (player.position.z - tile_z).powi(2))
    .sqrt();
    let bt = settings::PI / 2.0 - (-(player.position.z - tile_z) / d).acos();
    let theta = player::angle_round(settings::FOVZ / 2.0 + player.position.b - bt);
    let v = settings::WIDTHF / settings::FOVXY * theta;
    ProjResult { u, v, d }
}

fn wall_face_draw(player: &player::Player, game_map: &map::GameMap, i: usize, j: usize) {
    let tile_x = i as f32 + 0.5;
    let tile_y = j as f32 + 0.5;
    let tile_z = 0.5;

    let proj000 = project_point(player, tile_x - 0.5, tile_y - 0.5, tile_z - 0.5);
    let proj001 = project_point(player, tile_x - 0.5, tile_y - 0.5, tile_z + 0.5);
    let proj010 = project_point(player, tile_x - 0.5, tile_y + 0.5, tile_z - 0.5);
    let proj100 = project_point(player, tile_x + 0.5, tile_y - 0.5, tile_z - 0.5);
    let proj011 = project_point(player, tile_x - 0.5, tile_y + 0.5, tile_z + 0.5);
    let proj110 = project_point(player, tile_x + 0.5, tile_y + 0.5, tile_z - 0.5);
    let proj101 = project_point(player, tile_x + 0.5, tile_y - 0.5, tile_z + 0.5);
    let proj111 = project_point(player, tile_x + 0.5, tile_y + 0.5, tile_z + 0.5);
    
    let val = game_map.wall_array[i][j];
    let col = Color::from_rgba(100-val, val, 155-val, 255);

    let d1 = proj000.d + proj001.d + proj101.d + proj100.d;
    let d2 = proj000.d + proj001.d + proj011.d + proj010.d;
    let d3 = proj010.d + proj011.d + proj111.d + proj110.d;
    let d4 = proj110.d + proj111.d + proj101.d + proj100.d;

    let face1 = [&proj000, &proj001, &proj100, &proj101];
    let face2 = [&proj000, &proj001, &proj010, &proj011];
    let face3 = [&proj010, &proj011, &proj110, &proj111];
    let face4 = [&proj111, &proj110, &proj101, &proj100];

    let mut faces = vec![(d1, face1), (d2, face2), (d3, face3), (d4, face4)];
    faces.sort_by(cmp_depth);

    for face in faces {
        face_draw(face.1[0], face.1[1], face.1[2], face.1[3], col)
    }
    
}

fn floor_face_draw(player: &player::Player, game_map: &map::GameMap, i: usize, j: usize) {
    let tile_x = i as f32 + 0.5;
    let tile_y = j as f32 + 0.5;
    let tile_z = 0.0;
    let proj000 = project_point(player, tile_x - 0.5, tile_y - 0.5, tile_z );
    let proj010 = project_point(player, tile_x - 0.5, tile_y + 0.5, tile_z );
    let proj100 = project_point(player, tile_x + 0.5, tile_y - 0.5, tile_z );
    let proj110 = project_point(player, tile_x + 0.5, tile_y + 0.5, tile_z );
    let val = game_map.floor_array[i][j];
    let col = Color::from_rgba(val, val, val, 255);
    face_draw(&proj000, &proj010, &proj100, &proj110, col)
}

fn face_draw(proj1: &ProjResult, proj2: &ProjResult, proj3: &ProjResult, proj4: &ProjResult, col: Color) {
    draw_triangle(Vec2{x: proj2.u, y: proj2.v}, Vec2{x: proj3.u, y: proj3.v}, Vec2{x: proj1.u, y: proj1.v}, col);
    draw_triangle(Vec2{x: proj3.u, y: proj3.v}, Vec2{x: proj2.u, y: proj2.v}, Vec2{x: proj4.u, y: proj4.v}, col);
}

fn cmp_depth(a: &(f32, [&ProjResult; 4]), b: &(f32, [&ProjResult; 4])) -> Ordering {
    if a.0 < b.0 {
        return Ordering::Greater;
    } else if a.0 > b.0 {
        return Ordering::Less;
    }
    return Ordering::Equal;
}