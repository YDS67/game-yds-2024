use image::{self, EncodableLayout, ImageBuffer, Rgba};
use miniquad::*;
use glam::{vec3, Mat4};

use crate::assets;
use crate::camera;
use crate::map;
use crate::mesh;
use crate::player;
use crate::shaders;
use crate::settings;

pub struct Stage {
    pub ass: assets::Ass,
    pub player: player::Player,
    pub depth_buffer: camera::DepthBuffer,
    pub game_map: map::GameMap,
    pub mesh: mesh::Mesh,
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut dyn RenderingBackend, settings: &settings::Settings) -> Stage {
        let ass = assets::Ass::load();
        let player = player::Player::new(&settings);

        let mut game_map = map::GameMap::new(&ass);

        camera::find_visible_tiles(&mut game_map, &player, &settings);
        let depth_buffer = camera::DepthBuffer::generate(&game_map, &player, &settings);

        let mesh = mesh::Mesh::new(&depth_buffer, &player);

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&mesh.vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&mesh.indices),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.tile_atlas.clone();
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
        let texture = ctx.new_texture_from_data_and_format(pixels.as_bytes(), params);
        ctx.texture_generate_mipmaps(texture);
        

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX,
                    fragment: shaders::FRAGMENT,
                },
                shaders::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage {
            ass,
            player,
            game_map,
            depth_buffer,
            pipeline,
            bindings,
            mesh,
        }
    }

    pub fn update(&mut self, ctx: &mut dyn RenderingBackend, settings: &settings::Settings) {
        self.player.walk(&self.game_map, &settings);
        if self.player.movement.moving {
            camera::find_visible_tiles(&mut self.game_map, &self.player, &settings);
            self.depth_buffer = camera::DepthBuffer::generate(&self.game_map, &self.player, &settings);
    
            self.mesh = mesh::Mesh::new(&self.depth_buffer, &self.player);
    
            for b in 0..self.bindings.vertex_buffers.len() {
                ctx.delete_buffer(self.bindings.vertex_buffers[b]);
            }
            ctx.delete_buffer(self.bindings.index_buffer);
    
            let vertex_buffer = ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Stream,
                BufferSource::slice(&self.mesh.vertices),
            );
    
            let index_buffer = ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Stream,
                BufferSource::slice(&self.mesh.indices),
            );
    
            self.bindings.vertex_buffers = vec![vertex_buffer];
    
            self.bindings.index_buffer = index_buffer;
    
        }
        ctx.apply_pipeline(&self.pipeline);

        ctx.apply_bindings(&self.bindings);

        let proj = Mat4::perspective_rh_gl(settings.fov_xy, settings.screen_aspect, 0.01, settings::MAPSIZE as f32);
        let view = Mat4::look_to_rh(
            vec3(self.player.position.x, self.player.position.y, self.player.position.z),
            vec3(self.player.position.ax*self.player.position.bxy, self.player.position.ay*self.player.position.bxy, self.player.position.bz),
            vec3(0.0, 0.0, 1.0),
        );
        let mvp = proj * view;

        ctx.apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
            mvp,
            playerpos: (
                self.player.position.x,
                self.player.position.y,
                self.player.position.z,
            ),
        }));
        ctx.draw(0, &self.mesh.num * 6, 1);

        ctx.end_render_pass();

    }
}
