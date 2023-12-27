use image::{self, EncodableLayout, ImageBuffer, Rgba};
use miniquad::*;

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
    pub num: i32,
}

impl Stage {
    pub fn new(ctx: &mut dyn RenderingBackend, settings: &settings::Settings) -> Stage {
        let ass = assets::Ass::load();
        let player = player::Player::new(&settings);

        let mut game_map = map::GameMap::new(&ass);

        camera::find_visible_tiles(&mut game_map, &player, &settings);
        let depth_buffer = camera::DepthBuffer::generate(&game_map, &player);

        let mesh = mesh::Mesh::new(&depth_buffer);

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

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.wall_atlas.clone();
        let dims = pixels.dimensions();
        let texture = ctx.new_texture_from_rgba8(dims.0 as u16, dims.1 as u16, pixels.as_bytes());

        ctx.texture_set_min_filter(texture, FilterMode::Linear, MipmapFilterMode::None);
        ctx.texture_set_mag_filter(texture, FilterMode::Nearest);

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
                VertexAttribute::new("col", VertexFormat::Float4),
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
            num: mesh.idx,
            mesh,
        }
    }

    pub fn update(&mut self, ctx: &mut dyn RenderingBackend, settings: &settings::Settings) {
        self.player.walk(&self.game_map, &settings);
        camera::find_visible_tiles(&mut self.game_map, &self.player, &settings);
        self.depth_buffer = camera::DepthBuffer::generate(&self.game_map, &self.player);

        self.mesh = mesh::Mesh::new(&self.depth_buffer);
        self.num = self.mesh.idx;

        for b in 0..self.bindings.vertex_buffers.len() {
            ctx.delete_buffer(self.bindings.vertex_buffers[b]);
            //ctx.buffer_update(self.bindings.vertex_buffers[b], BufferSource::slice(&self.mesh.vertices));
        }
        ctx.delete_buffer(self.bindings.index_buffer);
        //ctx.buffer_update(self.bindings.index_buffer, BufferSource::slice(&self.mesh.indices));

        // for t in 0..self.bindings.images.len() {
        //     ctx.delete_texture(self.bindings.images[t]);
        // }

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

        ctx.apply_pipeline(&self.pipeline);

        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
            playerpos: (
                self.player.position.x,
                self.player.position.y,
                self.player.position.z,
            ),
            playerdir: (
                self.player.position.ax,
                self.player.position.ay,
                self.player.position.bz,
                self.player.position.bxy,
            ),
        }));
        ctx.draw(0, &self.num * 6, 1);

        ctx.end_render_pass();
    }
}
