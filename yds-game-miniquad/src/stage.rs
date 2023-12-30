use image::{self, EncodableLayout, ImageBuffer, Rgba};
use miniquad::*;
use glam::{vec3, Mat4};

use std::thread::sleep;
use std::time::Duration;

const FT_DESIRED: f64 = 0.01666666666667;

use crate::assets;
use crate::camera;
use crate::map;
use crate::mesh;
use crate::player;
use crate::shaders;
use crate::settings;

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,

    settings: settings::Settings,
    player: player::Player,
    depth_buffer: camera::DepthBuffer,
    game_map: map::GameMap,
    mesh_main: mesh::Mesh,
    mesh_text: mesh::Mesh,
    pipeline_main: Pipeline,
    pipeline_text: Pipeline,
    bindings_main: Bindings,
    bindings_text: Bindings,
    last_frame: std::time::Instant,
    elapsed_seconds: f64,
    text: Vec<String>,
    text_col: (f32, f32, f32, f32),
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

        let text = "text 1234";

        let mesh_main = mesh::Mesh::new_main(&depth_buffer, &player);
        let mesh_text = mesh::Mesh::new_text(&vec![text.to_string()], 25.0, 35.0,2.0/settings.screen_width_f, 2.0/settings.screen_height_f);

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

        let params = TextureParams{
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

        let params = TextureParams{
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
            pipeline_main,
            pipeline_text,
            bindings_main,
            bindings_text,
            mesh_main,
            mesh_text,
            last_frame: Some(std::time::Instant::now()).unwrap(),
            elapsed_seconds: 0.0,
            text: vec!["          ".to_string(),text.to_string(),"          ".to_string()],
            text_col: (1.0, 1.0, 0.0, 1.0),
        }
    }

    fn fps_measure(&mut self) {
        self.elapsed_seconds = self.last_frame.elapsed().as_secs_f64();
        if self.elapsed_seconds < FT_DESIRED {
            sleep(Duration::from_secs_f64(FT_DESIRED - self.elapsed_seconds));
        }
        self.elapsed_seconds = self.last_frame.elapsed().as_secs_f64();
        self.settings.delta_time = self.elapsed_seconds as f32;

        let fps = 1. / self.elapsed_seconds;
        self.settings.player_speed = 12.0*self.settings.delta_time;
        self.text = vec![
            format!("FPS: {:.0},", fps),
            format!("Quads drawn: {},", self.mesh_main.num),
            format!("Player moving? {}.", self.player.movement.moving),
        ];
    }

}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.fps_measure();

        self.player.walk(&self.game_map, &self.settings);

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
            self.depth_buffer = camera::DepthBuffer::generate(&self.game_map, &self.player, &self.settings);
    
            self.mesh_main = mesh::Mesh::new_main(&self.depth_buffer, &self.player);
            self.mesh_text = mesh::Mesh::new_text(&self.text, 10.0, 10.0,2.0/self.settings.screen_width_f, 2.0/self.settings.screen_height_f);

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
        self.ctx.begin_default_pass(miniquad::PassAction::clear_color(0., 0., 0., 1.0000000));

        self.ctx.apply_pipeline(&self.pipeline_main);

        self.ctx.apply_bindings(&self.bindings_main);

        let proj = Mat4::perspective_rh_gl(self.settings.fov_xy, self.settings.screen_aspect, 0.01, settings::MAPSIZE as f32);
        let view = Mat4::look_to_rh(
            vec3(self.player.position.x, self.player.position.y, self.player.position.z),
            vec3(self.player.position.ax*self.player.position.bxy, self.player.position.ay*self.player.position.bxy, self.player.position.bz),
            vec3(0.0, 0.0, 1.0),
        );
        let mvp = proj * view;

        self.ctx.apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsMain {
            mvp,
            playerpos: (
                self.player.position.x,
                self.player.position.y,
                self.player.position.z,
            ),
        }));
        self.ctx.draw(0, self.mesh_main.num * 6, 1);

        self.ctx.apply_pipeline(&self.pipeline_text);

        self.ctx.apply_bindings(&self.bindings_text);

        self.ctx.apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsText {
            fontcolor: self.text_col,
        }));

        self.ctx.draw(0, self.mesh_text.num * 6, 1);

        self.ctx.end_render_pass();

        self.ctx.commit_frame();

        self.last_frame = Some(std::time::Instant::now()).unwrap();
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if keycode == KeyCode::F {
            miniquad::window::set_fullscreen(true)
        }
        if keycode == KeyCode::Escape {
            miniquad::window::quit()
        } 
        self.player.read_key_down(keycode)
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.player.read_key_up(keycode)
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.settings.screen_change(width, height);
    }
}