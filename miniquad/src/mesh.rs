use crate::camera;
use crate::player;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
#[repr(C)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
#[repr(C)]
pub struct Vertex {
    pos: Vec3,
    uv: Vec2,
    col: Vec4,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
    pub num: i32,
}

impl Mesh {
    pub fn new(depth_buffer: &camera::DepthBuffer, player: &player::Player) -> Mesh {
        #[rustfmt::skip]
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        for l in 0..depth_buffer.len {
            let d = 1.0/(1.0 + (depth_buffer.faces[l].dist / depth_buffer.dmax).powi(2));
            if depth_buffer.faces[l].is_wall {
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 2.*0.0625, y: 0.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 1.0 }, uv: Vec2 { x: 2.*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 1.0 }, uv: Vec2 { x: 3.*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 3.*0.0625, y: 0.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top left

                indices.push(4*idx);
                indices.push(4*idx+1);
                indices.push(4*idx+3);
                indices.push(4*idx+1);
                indices.push(4*idx+2);
                indices.push(4*idx+3);

                idx = idx+1;

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 1.0 }, uv: Vec2 { x: 3.*0.0625, y: 0.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 3.*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 4.*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 1.0 }, uv: Vec2 { x: 4.*0.0625, y: 0.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top left

                indices.push(4*idx);
                indices.push(4*idx+1);
                indices.push(4*idx+3);
                indices.push(4*idx+1);
                indices.push(4*idx+2);
                indices.push(4*idx+3);

                idx = idx+1;

            } else {
                //ceiling and floor
                let mut z1: f32 = 2.0;
                let mut z2: f32 = 0.0;
                let mut v1: f32 = 7.0;
                let mut v2: f32 = 3.0;
                if player.position.z > 1.0 && player.position.b > 0.0 {
                    z1 = 0.0;
                    z2 = 2.0;
                    v1 = 3.0;
                    v2 = 7.0;
                }
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z1 }, uv: Vec2 { x: (v1+1.0)*0.0625, y: 2.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z1 }, uv: Vec2 { x: (v1+1.0)*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z1 }, uv: Vec2 { x: v1*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z1 }, uv: Vec2 { x: v1*0.0625, y: 2.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top left

                indices.push(4*idx);
                indices.push(4*idx+1);
                indices.push(4*idx+3);
                indices.push(4*idx+1);
                indices.push(4*idx+2);
                indices.push(4*idx+3);

                idx = idx+1;

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z2 }, uv: Vec2 { x: (v2+1.0)*0.0625, y: 2.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z2 }, uv: Vec2 { x: (v2+1.0)*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z2 }, uv: Vec2 { x: v2*0.0625, y: 1.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: z2 }, uv: Vec2 { x: v2*0.0625, y: 2.*0.0625 },
                    col: Vec4{x: d, y: 0.8*d, z: 0.7*d, w: 1.0} }); // top left

                indices.push(4*idx);
                indices.push(4*idx+1);
                indices.push(4*idx+3);
                indices.push(4*idx+1);
                indices.push(4*idx+2);
                indices.push(4*idx+3);

                idx = idx+1;
            }
        }

        Mesh { vertices, indices, num: idx as i32 }
    }
}