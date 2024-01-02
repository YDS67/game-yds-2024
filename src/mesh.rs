use crate::camera;
use crate::player;
use crate::text;

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

#[derive(Debug, Clone, Copy)]
pub struct TextureUV {
    pub u1: f32,
    pub u2: f32,
    pub v1: f32,
    pub v2: f32,
}

impl TextureUV {
    pub fn normalize(&mut self, width: f32, height: f32) {
        self.u1 = self.u1/width;
        self.u2 = self.u2/width;
        self.v1 = self.v1/height;
        self.v2 = self.v2/height;
    }
}

#[repr(C)]
pub struct Vertex {
    pos: Vec3,
    uv: Vec2,
    act: i32,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
    pub num: i32,
}

impl Mesh {
    pub fn new_main(depth_buffer: &camera::DepthBuffer, player: &player::Player) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        for l in 0..depth_buffer.len {
            if depth_buffer.faces[l].is_wall {
                let texture_u = depth_buffer.faces[l].texture_top.overflowing_rem(16).0 as f32;
                let texture_v = depth_buffer.faces[l].texture_top.overflowing_div(16).0 as f32;

                let tex_uv = TextureUV {
                    u1: texture_u * 0.0625,
                    u2: (texture_u+1.0) * 0.0625,
                    v1: texture_v * 0.0625,
                    v2: (texture_v+1.0) * 0.0625,
                };

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 2.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    }, act: 0,
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    }, act: 0,
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    }, act: 0,
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 2.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    }, act: 0,
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;

                let texture_u = depth_buffer.faces[l].texture_bot.overflowing_rem(16).0 as f32;
                let texture_v = depth_buffer.faces[l].texture_bot.overflowing_div(16).0 as f32;

                let tex_uv = TextureUV {
                    u1: texture_u * 0.0625,
                    u2: (texture_u+1.0) * 0.0625,
                    v1: texture_v * 0.0625,
                    v2: (texture_v+1.0) * 0.0625,
                };

                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    }, act: 0,
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    }, act: 0,
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    }, act: 0,
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    }, act: 0,
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

                let texture1_u = depth_buffer.faces[l].texture_top.overflowing_rem(16).0 as f32;
                let texture1_v = depth_buffer.faces[l].texture_top.overflowing_div(16).0 as f32;

                let mut tex_uv_1 = TextureUV {
                    u1: texture1_u * 0.0625,
                    u2: (texture1_u+1.0) * 0.0625,
                    v1: texture1_v * 0.0625,
                    v2: (texture1_v+1.0) * 0.0625,
                };

                let texture2_u = depth_buffer.faces[l].texture_bot.overflowing_rem(16).0 as f32;
                let texture2_v = depth_buffer.faces[l].texture_bot.overflowing_div(16).0 as f32;

                let mut tex_uv_2 = TextureUV {
                    u1: texture2_u * 0.0625,
                    u2: (texture2_u+1.0) * 0.0625,
                    v1: texture2_v * 0.0625,
                    v2: (texture2_v+1.0) * 0.0625,
                };


                if player.position.z > 1.0 && player.position.b > 0.0 {
                    z1 = 0.0;
                    z2 = 2.0;
                    tex_uv_1 = TextureUV {
                        u1: texture2_u * 0.0625,
                        u2: (texture2_u+1.0) * 0.0625,
                        v1: texture2_v * 0.0625,
                        v2: (texture2_v+1.0) * 0.0625,
                    };
                    tex_uv_2 = TextureUV {
                        u1: texture1_u * 0.0625,
                        u2: (texture1_u+1.0) * 0.0625,
                        v1: texture1_v * 0.0625,
                        v2: (texture1_v+1.0) * 0.0625,
                    };
                }
                let x = depth_buffer.faces[l].top_right_x as f32;
                let y = depth_buffer.faces[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u2,
                        y: tex_uv_1.v1,
                    }, act: 0,
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u2,
                        y: tex_uv_1.v2,
                    }, act: 0,
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u1,
                        y: tex_uv_1.v2,
                    }, act: 0,
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u1,
                        y: tex_uv_1.v1,
                    }, act: 0,
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
                    }, act: 0,
                }); // top right
                let x = depth_buffer.faces[l].bottom_right_x as f32;
                let y = depth_buffer.faces[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u2,
                        y: tex_uv_2.v2,
                    }, act: 0,
                }); // bottom right
                let x = depth_buffer.faces[l].bottom_left_x as f32;
                let y = depth_buffer.faces[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u1,
                        y: tex_uv_2.v2,
                    }, act: 0,
                }); // bottom left
                let x = depth_buffer.faces[l].top_left_x as f32;
                let y = depth_buffer.faces[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u1,
                        y: tex_uv_2.v1,
                    }, act: 0,
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

    pub fn new_overlay(overlay: &text::Overlay, scalex: f32, scaley: f32, fired: bool) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();
        let mut idx = 0;

        let mut tex_uv;

        if fired {
            tex_uv = text::string_to_uv("#")[0]
        } else {
            tex_uv = text::string_to_uv("&")[0]
        }

        let x = 0.5+text::WIDTH*overlay.scale*scalex;
        let y = 0.5-text::HEIGHT*overlay.scale*scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            }, act: 0,
        }); // top right
        let x = 0.5+text::WIDTH*overlay.scale*scalex;
        let y = 0.5+text::HEIGHT*overlay.scale*scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            }, act: 0,
        }); // bottom right
        let x = 0.5-text::WIDTH*overlay.scale*scalex;
        let y = 0.5+text::HEIGHT*overlay.scale*scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            }, act: 0,
        }); // bottom left
        let x = 0.5-text::WIDTH*overlay.scale*scalex;
        let y = 0.5-text::HEIGHT*overlay.scale*scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            }, act: 0,
        }); // top left

        indices.push(4 * idx);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 3);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 2);
        indices.push(4 * idx + 3);

        idx = idx + 1;

        for s in 0..overlay.lines.len() {
            let coords = text::string_to_uv(&overlay.lines[s]);

            for l in 0..coords.len() {
                tex_uv = coords[l];

                let lf = l as f32;

                let x = (overlay.line_x[s] + (lf+1.0)*text::WIDTH*overlay.scale)*scalex;
                let y = overlay.line_y[s]*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    }, act: 0,
                }); // top right
                let x = (overlay.line_x[s] + (lf+1.0)*text::WIDTH*overlay.scale)*scalex;
                let y = (overlay.line_y[s]+overlay.line_height)*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    }, act: 0,
                }); // bottom right
                let x = (overlay.line_x[s] + lf*text::WIDTH*overlay.scale)*scalex;
                let y = (overlay.line_y[s]+overlay.line_height)*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    }, act: 0,
                }); // bottom left
                let x = (overlay.line_x[s] + lf*text::WIDTH*overlay.scale)*scalex;
                let y = (overlay.line_y[s])*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    }, act: 0,
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

    pub fn new_gui(gui: &text::GUI, scalex: f32, scaley: f32) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();
        let mut idx = 0;

        let mut tex_uv = text::string_to_uv("=")[0];

        let x = 0.5 * (1.0 + 1.5*gui.max_width*scalex);
        let y = (gui.line_y[0] - 1.0*gui.line_height)* scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            }, act: 0,
        }); // top right
        let x = 0.5 * (1.0 + 1.5*gui.max_width*scalex);
        let y = (gui.line_y[gui.lines.len()-1] + 2.0*gui.line_height)* scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            }, act: 0,
        }); // bottom right
        let x = 0.5 * (1.0 - 1.5*gui.max_width*scalex);
        let y = (gui.line_y[gui.lines.len()-1] + 2.0*gui.line_height)* scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            }, act: 0,
        }); // bottom left
        let x = 0.5 * (1.0 - 1.5*gui.max_width*scalex);
        let y = (gui.line_y[0] - 1.0*gui.line_height)* scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            }, act: 0,
        }); // top left

        indices.push(4 * idx);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 3);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 2);
        indices.push(4 * idx + 3);

        idx = idx + 1;

        for s in 0..gui.lines.len() {
            let coords = text::string_to_uv(&gui.lines[s]);

            for l in 0..coords.len() {
                tex_uv = coords[l];

                let lf = l as f32;

                let x = (gui.line_x[s] + (lf+1.0)*text::WIDTH*gui.scale)*scalex;
                let y = gui.line_y[s]*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    }, act: gui.line_active[s],
                }); // top right
                let x = (gui.line_x[s] + (lf+1.0)*text::WIDTH*gui.scale)*scalex;
                let y = (gui.line_y[s]+gui.line_height)*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    }, act: gui.line_active[s],
                }); // bottom right
                let x = (gui.line_x[s] + lf*text::WIDTH*gui.scale)*scalex;
                let y = (gui.line_y[s]+gui.line_height)*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    }, act: gui.line_active[s],
                }); // bottom left
                let x = (gui.line_x[s] + lf*text::WIDTH*gui.scale)*scalex;
                let y = (gui.line_y[s])*scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    }, act: gui.line_active[s],
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
