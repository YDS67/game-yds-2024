use crate::camera;

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
    pub fn new(depth_buffer: &camera::DepthBuffer) -> Mesh {
        #[rustfmt::skip]
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        for l in 0..depth_buffer.len {
            let d = 1.0 - (depth_buffer.faces[l].dist / depth_buffer.dmax).powf(0.5);
            if depth_buffer.faces[l].is_wall {
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 1., y: 2./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 1., y: 4./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 0., y: 4./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 0., y: 2./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // top left

                indices.push(4*idx);
                indices.push(4*idx+1);
                indices.push(4*idx+3);
                indices.push(4*idx+1);
                indices.push(4*idx+2);
                indices.push(4*idx+3);

                idx = idx+1;

            } else {
                //ceiling
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 1., y: 2./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 1., y: 1./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 0., y: 1./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 2.0 }, uv: Vec2 { x: 0., y: 2./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // top left

                indices.push(4*idx);
                indices.push(4*idx+1);
                indices.push(4*idx+3);
                indices.push(4*idx+1);
                indices.push(4*idx+2);
                indices.push(4*idx+3);

                idx = idx+1;

                // floor
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 1., y: 5./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 1., y: 4./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 0., y: 4./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex { pos : Vec3 { x, y, z: 0.0 }, uv: Vec2 { x: 0., y: 5./5. },
                    col: Vec4{x: d, y: d, z: d, w: 1.0} }); // top left

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