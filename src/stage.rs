use glam::{vec3, Mat4};
use image::{self, EncodableLayout, ImageBuffer, Rgba};
use miniquad::*;

use std::thread::sleep;
use std::time::Duration;

const FT_DESIRED: f32 = 0.01666666666667;

use crate::assets;
use crate::camera;
use crate::map;
use crate::mesh;
use crate::player;
use crate::settings;
use crate::shaders;
use crate::text;

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,

    settings: settings::Settings,
    player: player::Player,
    depth_buffer: camera::DepthBuffer,
    game_map: map::GameMap,
    text: text::Text,
    mesh_main: mesh::Mesh,
    mesh_text: mesh::Mesh,
    pipeline_main: Pipeline,
    pipeline_text: Pipeline,
    bindings_main: Bindings,
    bindings_text: Bindings,

    time_state: TimeState,
    input_state: InputState,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let settings = settings::Settings::init();
        let ass = assets::Ass::load();
        let player = player::Player::new(&settings);

        let mut game_map = map::GameMap::new(&ass);

        camera::find_visible_tiles(&mut game_map, &player, &settings);
        let depth_buffer = camera::DepthBuffer::generate(&game_map, &player, &settings);

        let text = text::Text::new_from(vec!["Text default"]);

        let mesh_main = mesh::Mesh::new_main(&depth_buffer, &player);
        let mesh_text = mesh::Mesh::new_text(
            &text,
            1.0 / settings.screen_width_f,
            1.0 / settings.screen_height_f,
            false,
        );

        let vertex_buffer_main = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh_main.vertices),
        );

        let vertex_buffer_text = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh_text.vertices),
        );

        let index_buffer_main = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh_main.indices),
        );

        let index_buffer_text = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh_text.indices),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.tile_atlas;
        let dims = pixels.dimensions();

        let params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Linear,
            width: dims.0,
            height: dims.1,
            allocate_mipmaps: true,
        };
        let texture_main = ctx.new_texture_from_data_and_format(pixels.as_bytes(), params);
        ctx.texture_generate_mipmaps(texture_main);

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.font;
        let dims = pixels.dimensions();

        let params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::None,
            width: dims.0,
            height: dims.1,
            allocate_mipmaps: false,
        };
        let texture_text = ctx.new_texture_from_data_and_format(pixels.as_bytes(), params);

        let bindings_main = Bindings {
            vertex_buffers: vec![vertex_buffer_main],
            index_buffer: index_buffer_main,
            images: vec![texture_main],
        };

        let bindings_text = Bindings {
            vertex_buffers: vec![vertex_buffer_main, vertex_buffer_text],
            index_buffer: index_buffer_text,
            images: vec![texture_text],
        };

        let shader_main = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_MAIN,
                    fragment: shaders::FRAGMENT_MAIN,
                },
                shaders::meta_main(),
            )
            .unwrap();

        let shader_text = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_TEXT,
                    fragment: shaders::FRAGMENT_TEXT,
                },
                shaders::meta_text(),
            )
            .unwrap();

        let pipeline_main = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader_main,
        );

        let pipeline_text = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader_text,
        );

        Stage {
            ctx,

            settings,
            player,
            game_map,
            depth_buffer,
            text: text::Text::new_from(vec!["          ", "          "]),
            pipeline_main,
            pipeline_text,
            bindings_main,
            bindings_text,
            mesh_main,
            mesh_text,
            time_state: TimeState::init(),
            input_state: InputState::init(),
        }
    }

    fn frame_time(&mut self) {
        self.time_state.frame_time = self.time_state.last_frame.elapsed().as_secs_f32();
        if self.time_state.frame_time < FT_DESIRED {
            sleep(Duration::from_secs_f32(
                FT_DESIRED - self.time_state.frame_time,
            ));
        }
        self.time_state.frame_time = self.time_state.last_frame.elapsed().as_secs_f32();
        self.settings.delta_time = self.time_state.frame_time;
        self.time_state.fps = (1. / self.time_state.frame_time).floor() as i32;

        self.settings.player_speed = 12.0 * self.settings.delta_time;
    }

    fn show_data(&mut self) {
        self.text = text::Text::new_from(vec![
            &format!("FPS: {}", self.time_state.fps + 1),
            &format!("Line active: {}", self.text.act_no),
            &format!("Light distance: {}", self.settings.light_dist),
        ]);
        self.text.center();
    }

    fn gui_highlight(&mut self, x: f32, y: f32) {
        let mut some_active = false;
        for l in 0..self.text.lines.len() {
            if x > self.text.line_x[l + 1]
                && x < self.text.line_x[l + 1] + self.text.line_width[l + 1]
                && y > self.text.line_y[l + 1]
                && y < self.text.line_y[l + 1] + self.text.line_height
            {
                self.text.act_no = l + 1;
                some_active = true
            }
        }

        if !some_active {
            self.text.act_no = 0;
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.frame_time();
        self.show_data();
        self.gui_highlight(self.input_state.mouse.x, self.input_state.mouse.y);

        if self.input_state.keys.f && !self.settings.full_screen {
            miniquad::window::set_fullscreen(true);
            let screen = miniquad::window::screen_size();
            self.settings.full_screen = true;
            self.settings.screen_change(screen.0, screen.1);
        }
        if self.input_state.keys.l {
            self.settings.light_dist += 1.0
        }
        if self.input_state.keys.k && self.settings.light_dist > 0.0 {
            self.settings.light_dist -= 1.0
        }
        if self.input_state.keys.esc {
            miniquad::window::quit()
        }

        self.player.read_key(&self.input_state);

        self.player.walk(
            &self.game_map,
            &self.settings,
            self.input_state.mouse.dx,
            self.input_state.mouse.dy,
            self.input_state.mouse.moving,
        );

        if true {
            for b in 0..self.bindings_main.vertex_buffers.len() {
                self.ctx.delete_buffer(self.bindings_main.vertex_buffers[b]);
            }
            self.ctx.delete_buffer(self.bindings_main.index_buffer);

            for b in 0..self.bindings_text.vertex_buffers.len() {
                self.ctx.delete_buffer(self.bindings_text.vertex_buffers[b]);
            }
            self.ctx.delete_buffer(self.bindings_text.index_buffer);

            camera::find_visible_tiles(&mut self.game_map, &self.player, &self.settings);
            self.depth_buffer =
                camera::DepthBuffer::generate(&self.game_map, &self.player, &self.settings);

            self.mesh_main = mesh::Mesh::new_main(&self.depth_buffer, &self.player);
            self.mesh_text = mesh::Mesh::new_text(
                &self.text,
                1.0 / self.settings.screen_width_f,
                1.0 / self.settings.screen_height_f,
                self.input_state.mouse.left
            );

            let vertex_buffer_main = self.ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&self.mesh_main.vertices),
            );

            let vertex_buffer_text = self.ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&self.mesh_text.vertices),
            );

            let index_buffer_main = self.ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&self.mesh_main.indices),
            );

            let index_buffer_text = self.ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&self.mesh_text.indices),
            );

            self.bindings_main.vertex_buffers = vec![vertex_buffer_main];

            self.bindings_main.index_buffer = index_buffer_main;

            self.bindings_text.vertex_buffers = vec![vertex_buffer_text];

            self.bindings_text.index_buffer = index_buffer_text;
        }
    }

    fn draw(&mut self) {
        window::show_mouse(false);

        self.ctx
            .begin_default_pass(miniquad::PassAction::clear_color(0., 0., 0., 1.0000000));

        self.ctx.apply_pipeline(&self.pipeline_main);

        self.ctx.apply_bindings(&self.bindings_main);

        let proj = Mat4::perspective_rh_gl(
            self.settings.fov_xy,
            self.settings.screen_aspect,
            0.01,
            settings::MAPSIZE as f32,
        );
        let view = Mat4::look_to_rh(
            vec3(
                self.player.position.x,
                self.player.position.y,
                self.player.position.z,
            ),
            vec3(
                self.player.position.ax * self.player.position.bxy,
                self.player.position.ay * self.player.position.bxy,
                self.player.position.bz,
            ),
            vec3(0.0, 0.0, 1.0),
        );
        let mvp = proj * view;

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsMain {
                mvp,
                playerpos: (
                    self.player.position.x,
                    self.player.position.y,
                    self.player.position.z,
                ),
                lightdist: self.settings.light_dist,
            }));
        self.ctx.draw(0, self.mesh_main.num * 6, 1);

        self.ctx.apply_pipeline(&self.pipeline_text);

        self.ctx.apply_bindings(&self.bindings_text);

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsText {
                fontcolor: self.text.font_col,
                actcolor: self.text.act_col,
                activeline: (
                    self.text.line_y[self.text.act_no] / self.settings.screen_height_f,
                    (self.text.line_y[self.text.act_no] + self.text.line_height)
                        / self.settings.screen_height_f,
                ),
            }));

        self.ctx.draw(0, self.mesh_text.num * 6, 1);

        self.ctx.end_render_pass();

        self.ctx.commit_frame();

        self.time_state.last_frame = Some(std::time::Instant::now()).unwrap();
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input_state.keys.read_key(keycode, true)
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.input_state.keys.read_key(keycode, false)
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.settings.screen_change(width, height);
    }

    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        let moving_x;
        let moving_y;
        if dx.abs() < settings::TOLERANCE*self.settings.screen_width_f
        //    || self.input_state.mouse.dx.abs() < settings::TOLERANCE
        {
            self.input_state.mouse.dx = 0.5 * settings::TOLERANCE;
            moving_x = false;
        } else {
            self.input_state.mouse.dx = 0.5 * dx / self.settings.screen_width_f;
            moving_x = true;
        }
        if dy.abs() < settings::TOLERANCE*self.settings.screen_width_f
        //    || self.input_state.mouse.dy.abs() < settings::TOLERANCE
        {
            self.input_state.mouse.dy = 0.5 * settings::TOLERANCE;
            moving_y = false;
        } else {
            self.input_state.mouse.dy = 0.5 * dy / self.settings.screen_width_f;
            moving_y = true;
        }
        self.input_state.mouse.moving = moving_x || moving_y
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.input_state.mouse.x = x;
        self.input_state.mouse.y = y;
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.input_state.mouse.left = true;
        }
        if button == MouseButton::Right {
            self.input_state.mouse.right = true;
        }
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.input_state.mouse.left = false;
        }
        if button == MouseButton::Right {
            self.input_state.mouse.right = false;
        }
    }
}

struct TimeState {
    last_frame: std::time::Instant,
    frame_time: f32,
    fps: i32,
}

impl TimeState {
    fn init() -> TimeState {
        TimeState {
            last_frame: Some(std::time::Instant::now()).unwrap(),
            frame_time: 1.0 / 60.0,
            fps: 60,
        }
    }
}

pub struct KeysState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub k: bool,
    pub l: bool,
    pub f: bool,
    pub esc: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
    pub enter: bool,
}

impl KeysState {
    fn read_key(&mut self, keycode: KeyCode, state: bool) {
        if keycode == KeyCode::W {
            self.w = state
        }
        if keycode == KeyCode::S {
            self.s = state
        }
        if keycode == KeyCode::Left {
            self.left = state
        }
        if keycode == KeyCode::Right {
            self.right = state
        }
        if keycode == KeyCode::A {
            self.a = state
        }
        if keycode == KeyCode::D {
            self.d = state
        }
        if keycode == KeyCode::Down {
            self.down = state
        }
        if keycode == KeyCode::Up {
            self.up = state
        }
        if keycode == KeyCode::Space {
            self.space = state
        }
        if keycode == KeyCode::Escape {
            self.esc = state
        }
        if keycode == KeyCode::Enter {
            self.enter = state
        }
        if keycode == KeyCode::K {
            self.k = state
        }
        if keycode == KeyCode::L {
            self.l = state
        }
        if keycode == KeyCode::Q {
            self.q = state
        }
        if keycode == KeyCode::E {
            self.e = state
        }
        if keycode == KeyCode::F {
            self.f = state
        }
    }
}

pub struct MouseState {
    pub left: bool,
    pub right: bool,
    pub moving: bool,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct InputState {
    pub keys: KeysState,
    pub mouse: MouseState,
}

impl InputState {
    fn init() -> InputState {
        InputState {
            keys: KeysState {
                w: false,
                a: false,
                s: false,
                d: false,
                q: false,
                e: false,
                k: false,
                l: false,
                f: false,
                left: false,
                right: false,
                up: false,
                down: false,
                space: false,
                enter: false,
                esc: false,
            },
            mouse: MouseState {
                left: false,
                right: false,
                moving: false,
                x: 0.0,
                y: 0.0,
                dx: 0.5 * settings::TOLERANCE,
                dy: 0.5 * settings::TOLERANCE,
            },
        }
    }
}
