use crate::camera;
use crate::player;
use crate::settings;
use crate::sprites;
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
        self.u1 = self.u1 / width;
        self.u2 = self.u2 / width;
        self.v1 = self.v1 / height;
        self.v2 = self.v2 / height;
    }
}

#[repr(C)]
pub struct Vertex {
    pos: Vec3,
    uv: Vec2,
    act: f32,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
    pub num: i32,
}

impl Mesh {
    pub fn new_main(
        depth_buffer: &camera::DepthBuffer,
        sprite_buffer: &sprites::SpriteBuffer,
    ) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        let du = 2.0 * 64.0 / 1024.0;
        let uw = 64.0 / 1024.0;

        for l in 0..depth_buffer.len {
            if depth_buffer.faces_dist[l].is_wall {
                let texture_u =
                    1.0 + depth_buffer.faces_dist[l].texture_top.overflowing_rem(32).0 as f32 / 4.0;
                let texture_v =
                    1.0 + depth_buffer.faces_dist[l].texture_top.overflowing_div(32).0 as f32;

                let tex_uv = TextureUV {
                    u1: texture_u * du - 1.5 * uw,
                    u2: texture_u * du - 0.5 * uw,
                    v1: texture_v * du - 1.5 * uw,
                    v2: texture_v * du - 0.5 * uw,
                };

                let x = depth_buffer.faces_dist[l].top_right_x as f32;
                let y = depth_buffer.faces_dist[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 2.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = depth_buffer.faces_dist[l].bottom_right_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = depth_buffer.faces_dist[l].bottom_left_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = depth_buffer.faces_dist[l].top_left_x as f32;
                let y = depth_buffer.faces_dist[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 2.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;

                let texture_u =
                    1.0 + depth_buffer.faces_dist[l].texture_bot.overflowing_rem(32).0 as f32 / 4.0;
                let texture_v =
                    1.0 + depth_buffer.faces_dist[l].texture_bot.overflowing_div(32).0 as f32;

                let tex_uv = TextureUV {
                    u1: texture_u * du - 1.5 * uw,
                    u2: texture_u * du - 0.5 * uw,
                    v1: texture_v * du - 1.5 * uw,
                    v2: texture_v * du - 0.5 * uw,
                };

                let x = depth_buffer.faces_dist[l].top_right_x as f32;
                let y = depth_buffer.faces_dist[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = depth_buffer.faces_dist[l].bottom_right_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = depth_buffer.faces_dist[l].bottom_left_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = depth_buffer.faces_dist[l].top_left_x as f32;
                let y = depth_buffer.faces_dist[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 1.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
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
                let z1: f32 = 2.0;
                let z2: f32 = 0.0;

                let texture1_u =
                    1.0 + depth_buffer.faces_dist[l].texture_top.overflowing_rem(32).0 as f32 / 4.0;
                let texture1_v =
                    1.0 + depth_buffer.faces_dist[l].texture_top.overflowing_div(32).0 as f32;

                let tex_uv_1 = TextureUV {
                    u1: texture1_u * du - 1.5 * uw,
                    u2: texture1_u * du - 0.5 * uw,
                    v1: texture1_v * du - 1.5 * uw,
                    v2: texture1_v * du - 0.5 * uw,
                };

                let texture2_u =
                    1.0 + depth_buffer.faces_dist[l].texture_bot.overflowing_rem(32).0 as f32 / 4.0;
                let texture2_v =
                    1.0 + depth_buffer.faces_dist[l].texture_bot.overflowing_div(32).0 as f32;

                let tex_uv_2 = TextureUV {
                    u1: texture2_u * du - 1.5 * uw,
                    u2: texture2_u * du - 0.5 * uw,
                    v1: texture2_v * du - 1.5 * uw,
                    v2: texture2_v * du - 0.5 * uw,
                };

                let x = depth_buffer.faces_dist[l].top_right_x as f32;
                let y = depth_buffer.faces_dist[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u2,
                        y: tex_uv_1.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = depth_buffer.faces_dist[l].bottom_right_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u2,
                        y: tex_uv_1.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = depth_buffer.faces_dist[l].bottom_left_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u1,
                        y: tex_uv_1.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = depth_buffer.faces_dist[l].top_left_x as f32;
                let y = depth_buffer.faces_dist[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z1 },
                    uv: Vec2 {
                        x: tex_uv_1.u1,
                        y: tex_uv_1.v1,
                    },
                    act: 0.0,
                }); // top left

                indices.push(4 * idx);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);

                idx = idx + 1;

                let x = depth_buffer.faces_dist[l].top_right_x as f32;
                let y = depth_buffer.faces_dist[l].top_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u2,
                        y: tex_uv_2.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = depth_buffer.faces_dist[l].bottom_right_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_right_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u2,
                        y: tex_uv_2.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = depth_buffer.faces_dist[l].bottom_left_x as f32;
                let y = depth_buffer.faces_dist[l].bottom_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u1,
                        y: tex_uv_2.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = depth_buffer.faces_dist[l].top_left_x as f32;
                let y = depth_buffer.faces_dist[l].top_left_y as f32;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: z2 },
                    uv: Vec2 {
                        x: tex_uv_2.u1,
                        y: tex_uv_2.v1,
                    },
                    act: 0.0,
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

        let du = 64.0 / 1024.0;
        let uw = 64.0 / 1024.0;

        if sprite_buffer.len > 0 {
            for l in 0..sprite_buffer.len {
                let texture_u =
                    1.0 + sprite_buffer.sprites_dist[l].texture.overflowing_rem(16).0 as f32;
                let texture_v =
                    5.0 + sprite_buffer.sprites_dist[l].texture.overflowing_div(16).0 as f32;

                let tex_uv = TextureUV {
                    u1: texture_u * du - 1.0 * uw,
                    u2: texture_u * du,
                    v1: texture_v * du - 1.0 * uw,
                    v2: texture_v * du,
                };

                let x = sprite_buffer.sprites_dist[l].top_right_x;
                let y = sprite_buffer.sprites_dist[l].top_right_y;
                let z = sprite_buffer.sprites_dist[l].top_right_z;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = sprite_buffer.sprites_dist[l].bottom_right_x;
                let y = sprite_buffer.sprites_dist[l].bottom_right_y;
                let z = sprite_buffer.sprites_dist[l].bottom_right_z;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = sprite_buffer.sprites_dist[l].bottom_left_x;
                let y = sprite_buffer.sprites_dist[l].bottom_left_y;
                let z = sprite_buffer.sprites_dist[l].bottom_left_z;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = sprite_buffer.sprites_dist[l].top_left_x;
                let y = sprite_buffer.sprites_dist[l].top_left_y;
                let z = sprite_buffer.sprites_dist[l].top_left_z;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
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

        let x = 0.5 + text::WIDTH * overlay.scale * scalex;
        let y = 0.5 - text::HEIGHT * overlay.scale * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 0.5 + text::WIDTH * overlay.scale * scalex;
        let y = 0.5 + text::HEIGHT * overlay.scale * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = 0.5 - text::WIDTH * overlay.scale * scalex;
        let y = 0.5 + text::HEIGHT * overlay.scale * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = 0.5 - text::WIDTH * overlay.scale * scalex;
        let y = 0.5 - text::HEIGHT * overlay.scale * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            },
            act: 0.0,
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

                let x = (overlay.line_x[s] + (lf + 1.0) * text::WIDTH * overlay.scale) * scalex;
                let y = overlay.line_y[s] * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = (overlay.line_x[s] + (lf + 1.0) * text::WIDTH * overlay.scale) * scalex;
                let y = (overlay.line_y[s] + overlay.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = (overlay.line_x[s] + lf * text::WIDTH * overlay.scale) * scalex;
                let y = (overlay.line_y[s] + overlay.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = (overlay.line_x[s] + lf * text::WIDTH * overlay.scale) * scalex;
                let y = (overlay.line_y[s]) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
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

        let x = 0.5 * (1.0 + 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[0] - 1.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 0.5 * (1.0 + 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[gui.lines.len() - 1] + 2.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = 0.5 * (1.0 - 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[gui.lines.len() - 1] + 2.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = 0.5 * (1.0 - 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[0] - 1.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            },
            act: 0.0,
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

                let x = (gui.line_x[s] + (lf + 1.0) * text::WIDTH * gui.scale) * scalex;
                let y = gui.line_y[s] * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: gui.line_active[s] as f32,
                }); // top right
                let x = (gui.line_x[s] + (lf + 1.0) * text::WIDTH * gui.scale) * scalex;
                let y = (gui.line_y[s] + gui.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: gui.line_active[s] as f32,
                }); // bottom right
                let x = (gui.line_x[s] + lf * text::WIDTH * gui.scale) * scalex;
                let y = (gui.line_y[s] + gui.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: gui.line_active[s] as f32,
                }); // bottom left
                let x = (gui.line_x[s] + lf * text::WIDTH * gui.scale) * scalex;
                let y = (gui.line_y[s]) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: gui.line_active[s] as f32,
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

    pub fn new_map(
        rays: &Vec<camera::Ray>,
        player: &player::Player,
        settings: &settings::Settings,
        scalex: f32,
        scaley: f32,
    ) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut tex_uv = TextureUV {
            u1: player.position.x - settings.draw_max_dist,
            u2: player.position.x + settings.draw_max_dist,
            v1: settings.map_size_f - player.position.y - settings.draw_max_dist,
            v2: settings.map_size_f - player.position.y + settings.draw_max_dist,
        };

        tex_uv.normalize(settings.map_size_f, settings.map_size_f);

        let width = 2.0 * settings.draw_max_dist * settings.tile_screen_size;
        let height = 2.0 * settings.draw_max_dist * settings.tile_screen_size;
        let x_offset = 20.0;
        let y_offset = settings.screen_height_f - height - 20.0;

        let x = 1.0 - (x_offset) * scalex;
        let y = 1.0 - (y_offset + height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 1.0 - (x_offset) * scalex;
        let y = 1.0 - (y_offset) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = 1.0 - (x_offset + width) * scalex;
        let y = 1.0 - (y_offset) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = 1.0 - (x_offset + width) * scalex;
        let y = 1.0 - (y_offset + height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top left

        indices.push(0);
        indices.push(1);
        indices.push(3);
        indices.push(1);
        indices.push(2);
        indices.push(3);

        tex_uv = TextureUV {
            u1: 0.0,
            u2: 1.0 / 256.0,
            v1: 0.0,
            v2: 1.0 / 256.0,
        };

        // VISIBLE TILES
        let act = 1.0;

        // player position vertex
        let xp = 1.0 - (x_offset + 0.5 * width) * scalex;
        let yp = 1.0 - (y_offset + 0.5 * height) * scaley;

        vertices.push(Vertex {
            pos: Vec3 {
                x: xp,
                y: yp,
                z: 0.0,
            },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            },
            act,
        });

        let mut idx = 0;

        for l in 0..rays.len() {
            if true {
                let xt = x_offset + 0.5 * width
                    - settings.tile_screen_size * (rays[l].x - player.position.x);
                let yt = y_offset
                    + 0.5 * height
                    + settings.tile_screen_size * (rays[l].y - player.position.y);

                let x1 = 1.0 - xt * scalex;
                let y1 = 1.0 - yt * scaley;

                // visible walls and floor
                vertices.push(Vertex {
                    pos: Vec3 {
                        x: x1,
                        y: y1,
                        z: 0.0,
                    },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act,
                });

                idx = idx + 1;
            }
        }

        for i in 0..(idx - 2) {
            indices.push(4);
            indices.push(i + 5);
            indices.push(i + 6);
        }

        Mesh {
            vertices,
            indices,
            num: idx as i32,
        }
    }
}
