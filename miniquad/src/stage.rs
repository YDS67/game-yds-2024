use image::{self, EncodableLayout, ImageBuffer, Rgba};
use miniquad::*;
use macroquad::prelude::*;

use crate::shaders;
use crate::assets;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

pub struct Stage {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
    pub font: Font,
}

impl Stage {
    pub async fn new(ctx: &mut dyn RenderingBackend) -> Stage {
        let ass = assets::Ass::load().await;

        #[rustfmt::skip]
            let quad: [Vertex; 4] = [
                Vertex { pos : Vec2 { x: -0.5, y: -0.7 }, uv: Vec2 { x: 0., y: 3./5. } },
                Vertex { pos : Vec2 { x:  0.5, y: -0.3 }, uv: Vec2 { x: 1., y: 3./5. } },
                Vertex { pos : Vec2 { x:  0.5, y:  0.3 }, uv: Vec2 { x: 1., y: 2./5. } },
                Vertex { pos : Vec2 { x: -0.5, y:  0.7 }, uv: Vec2 { x: 0., y: 2./5. } },
            ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&quad),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices[..]),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.wall_atlas;
        let dims = pixels.dimensions();
        let texture = ctx.new_texture_from_rgba8(dims.0 as u16, dims.1 as u16, pixels.as_bytes());

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
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage {
            pipeline, 
            bindings,
            font: ass.font_main,
        }
    }
}
