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

// #[repr(C)]
// struct Vec4 {
//     x: f32,
//     y: f32,
//     z: f32,
//     w: f32,
// }

pub struct TextureUV {
    pub u1: f32,
    pub u2: f32,
    pub v1: f32,
    pub v2: f32,
}

#[repr(C)]
pub struct Vertex {
    pos: Vec3,
    uv: Vec2,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
    pub num: i32,
}

impl Mesh {
    pub fn new_main(depth_buffer: &camera::DepthBuffer, player: &player::Player) -> Mesh {
        #[rustfmt::skip]
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        for l in 0..depth_buffer.len {
            if depth_buffer.faces[l].is_wall {
                let tex_uv = TextureUV {
                    u1: 2. * 0.0625,
                    u2: 3. * 0.0625,
                    v1: 0. * 0.0625,
                    v2: 1. * 0.0625,
                };

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 2.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 2.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;

                let tex_uv = TextureUV {
                    u1: 3. * 0.0625,
                    u2: 4. * 0.0625,
                    v1: 0. * 0.0625,
                    v2: 1. * 0.0625,
                };

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;
            } else {
                //ceiling and floor
                let mut z1: f32 = 2.0;
                let mut z2: f32 = 0.0;
                let mut tex_uv_1 = TextureUV {
                    u1: 7. * 0.0625,
                    u2: 8. * 0.0625,
                    v1: 1. * 0.0625,
                    v2: 2. * 0.0625,
                };
                let mut tex_uv_2 = TextureUV {
                    u1: 3. * 0.0625,
                    u2: 4. * 0.0625,
                    v1: 1. * 0.0625,
                    v2: 2. * 0.0625,
                };
                if player.position.z > 1.0 && player.position.b > 0.0 {
                    z1 = 0.0;
                    z2 = 2.0;
                    tex_uv_1 = TextureUV {
                        u1: 3. * 0.0625,
                        u2: 4. * 0.0625,
                        v1: 1. * 0.0625,
                        v2: 2. * 0.0625,
                    };
                    tex_uv_2 = TextureUV {
                        u1: 7. * 0.0625,
                        u2: 8. * 0.0625,
                        v1: 1. * 0.0625,
                        v2: 2. * 0.0625,
                    };
                }
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u2,
                        y: tex_uv_1.v1,
                    },
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u2,
                        y: tex_uv_1.v2,
                    },
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u1,
                        y: tex_uv_1.v2,
                    },
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u1,
                        y: tex_uv_1.v1,
                    },
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u2,
                        y: tex_uv_2.v1,
                    },
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u2,
                        y: tex_uv_2.v2,
                    },
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u1,
                        y: tex_uv_2.v2,
                    },
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u1,
                        y: tex_uv_2.v1,
                    },
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;
            }
        }

        Mesh {
            vertices,
            indices,
            num: idx as i32,
        }
    }
}
