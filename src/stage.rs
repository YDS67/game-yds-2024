use glam::{vec3, Mat4};
use image::{self, EncodableLayout, ImageBuffer, Rgba};
use std::sync::mpsc::Sender;
use miniquad::*;

use crate::assets;
use crate::camera;
use crate::map;
use crate::mesh;
use crate::player;
use crate::settings;
use crate::shaders;
use crate::sprites;
use crate::text;
use crate::input::{TimeState, InputState};

struct Proj {
    proj: Mat4,
    view: Mat4,
    mvp: Mat4,
}

impl Proj {
    fn new(player: &player::Player, settings: &settings::Settings) -> Proj {
        let proj = Mat4::perspective_rh_gl(
            settings.fov_xy,
            settings.screen_aspect,
            0.01,
            settings.map_size_f,
        );
        let view = Mat4::look_to_rh(
            vec3(
                player.position.x,
                player.position.y,
                player.position.z,
            ),
            vec3(
                player.position.ax * player.position.bxy,
                player.position.ay * player.position.bxy,
                player.position.bz,
            ),
            vec3(0.0, 0.0, 1.0),
        );
        let mvp = proj * view;
        Proj { proj, view, mvp }
    }

    fn update(&mut self, player: &player::Player, settings: &settings::Settings) {
        self.proj = Mat4::perspective_rh_gl(
            settings.fov_xy,
            settings.screen_aspect,
            0.01,
            settings.map_size_f,
        );
        self.view = Mat4::look_to_rh(
            vec3(
                player.position.x,
                player.position.y,
                player.position.z,
            ),
            vec3(
                player.position.ax * player.position.bxy,
                player.position.ay * player.position.bxy,
                player.position.bz,
            ),
            vec3(0.0, 0.0, 1.0),
        );
        self.mvp = self.proj * self.view;
    }
}

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,

    settings: settings::Settings,
    pub player: player::Player,
    face_buffer: camera::FaceBuffer,
    sprite_buffer: sprites::SpriteBuffer,
    game_map: map::GameMap,
    overlay: text::Overlay,
    gui: text::GUI,
    mesh: Vec<mesh::Mesh>,
    render_pass: RenderPass,
    pipeline: Vec<Pipeline>,
    bindings: Vec<Bindings>,
    proj: Proj,
    tx: Sender<bool>,

    time_state: TimeState,
    input_state: InputState,
    request: bool,
}

impl Stage {
    pub fn new(tx: &Sender<bool>) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let settings = settings::Settings::init();
        let ass = assets::Ass::load();
        let player = player::Player::new(&settings);

        let mut game_map = map::GameMap::new(&ass);

        let rays = camera::ray_cast(&mut game_map, &player, &settings);
        let face_buffer = camera::FaceBuffer::generate(&game_map, &player, &settings);

        let sprite_buffer = sprites::SpriteBuffer::generate(&game_map, &player, &settings);

        let overlay = text::Overlay::new_from(vec!["Text default"]);
        let gui = text::GUI::new_from(vec!["Text default"], settings.screen_width_f, settings.screen_height_f);

        let mesh_main = mesh::Mesh::new_main(&face_buffer, &sprite_buffer);
        let mesh_overlay = mesh::Mesh::new_overlay(
            &overlay,
            1.0 / settings.screen_width_f,
            1.0 / settings.screen_height_f,
            false,
        );
        let mesh_gui = mesh::Mesh::new_gui(
            &gui,
            1.0 / settings.screen_width_f,
            1.0 / settings.screen_height_f,
        );
        let mesh_map = mesh::Mesh::new_map(
            &rays,
            &player,
            &settings,
            1.0 / settings.screen_width_f,
            1.0 / settings.screen_height_f,
        );
        let mesh_screen = mesh::Mesh::new_screen();

        let vertex_buffer_main = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<mesh::Vertex>(settings::MAX_VERTICES_MAIN),
        );

        let vertex_buffer_overlay = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<mesh::Vertex>(settings::MAX_VERTICES_OVERLAY),
        );

        let vertex_buffer_gui = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<mesh::Vertex>(settings::MAX_VERTICES_GUI),
        );

        let vertex_buffer_map = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<mesh::Vertex>(settings::MAX_VERTICES_MAP),
        );

        let vertex_buffer_screen = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh_screen.vertices),
        );

        let index_buffer_main = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<i16>(2*settings::MAX_INDICES_MAIN),
        );

        let index_buffer_overlay = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<i16>(2*settings::MAX_INDICES_OVERLAY),
        );

        let index_buffer_gui = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<i16>(2*settings::MAX_INDICES_GUI),
        );

        let index_buffer_map = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<i16>(2*settings::MAX_INDICES_MAP),
        );

        let index_buffer_screen = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&mesh_screen.indices),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.tile_atlas;
        let dims = pixels.dimensions();

        let mut t_params = TextureParams {
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

        let texture_main = ctx.new_texture_from_data_and_format(pixels.as_bytes(), t_params);
        ctx.texture_generate_mipmaps(texture_main);

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.font;
        let dims = pixels.dimensions();
        t_params.mipmap_filter = MipmapFilterMode::None;
        t_params.width = dims.0;
        t_params.height = dims.1;
        t_params.allocate_mipmaps = false;

        let texture_overlay = ctx.new_texture_from_data_and_format(pixels.as_bytes(), t_params);

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.wall_image_bot;
        let dims = pixels.dimensions();
        t_params.width = dims.0;
        t_params.height = dims.1;

        let texture_map = ctx.new_texture_from_data_and_format(pixels.as_bytes(), t_params);

        t_params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Linear,
            mag_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::None,
            width: settings::WIDTH,
            height: settings::HEIGHT,
            allocate_mipmaps: false,
        };

        let texture = ctx.new_render_texture(t_params);

        t_params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::Depth,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::None,
            width: settings::WIDTH,
            height: settings::HEIGHT,
            allocate_mipmaps: false,
        };

        let depth_tex = ctx.new_render_texture(t_params);

        let bindings_main = Bindings {
            vertex_buffers: vec![vertex_buffer_main],
            index_buffer: index_buffer_main,
            images: vec![texture_main],
        };

        let bindings_overlay = Bindings {
            vertex_buffers: vec![vertex_buffer_overlay],
            index_buffer: index_buffer_overlay,
            images: vec![texture_overlay],
        };

        let bindings_gui = Bindings {
            vertex_buffers: vec![vertex_buffer_gui],
            index_buffer: index_buffer_gui,
            images: vec![texture_overlay],
        };

        let bindings_map = Bindings {
            vertex_buffers: vec![vertex_buffer_map],
            index_buffer: index_buffer_map,
            images: vec![texture_map],
        };

        let bindings_screen = Bindings {
            vertex_buffers: vec![vertex_buffer_screen],
            index_buffer: index_buffer_screen,
            images: vec![texture],
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

        let shader_overlay = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_OVERLAY,
                    fragment: shaders::FRAGMENT_OVERLAY,
                },
                shaders::meta_overlay(),
            )
            .unwrap();

        let shader_gui = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_GUI,
                    fragment: shaders::FRAGMENT_GUI,
                },
                shaders::meta_gui(),
            )
            .unwrap();

        let shader_map = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_MAP,
                    fragment: shaders::FRAGMENT_MAP,
                },
                shaders::meta_map(),
            )
            .unwrap();

        let shader_screen = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_SCREEN,
                    fragment: shaders::FRAGMENT_SCREEN,
                },
                shaders::meta_screen(),
            )
            .unwrap();

        let mut p_params = PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::LessOrEqual,
            depth_write: true,      
            depth_write_offset: None,
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            ),
            alpha_blend: Some(BlendState::new(Equation::Add, 
                BlendFactor::Value(BlendValue::SourceAlpha), 
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
        };

        let pipeline_main: Pipeline = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_main,
            p_params,
        );

        p_params = PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always,
            depth_write: false,      
            depth_write_offset: None,
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            ),
            alpha_blend: Some(BlendState::new(Equation::Add, 
                BlendFactor::Value(BlendValue::SourceAlpha), 
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
        };

        let pipeline_overlay = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_overlay,
            p_params,
        );

        let pipeline_gui = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_gui,
            p_params,
        );

        let pipeline_map = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_map,
            p_params,
        );

        let p_params = PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always,
            depth_write: false,      
            depth_write_offset: None,
            color_blend: None,
            alpha_blend: None,
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
        };

        let pipeline_screen = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_screen,
            p_params,
        );

        let gui = text::GUI::new_from(vec!["Text default"], settings.screen_width_f, settings.screen_height_f);
        //gui.show = false;

        let proj = Proj::new(&player, &settings);

        let render_pass = ctx.new_render_pass(texture, Some(depth_tex));

        Stage {
            ctx,

            settings,
            player,
            game_map,
            face_buffer,
            sprite_buffer,
            overlay: text::Overlay::new_from(vec!["Text default"]),
            gui,
            pipeline: vec![pipeline_main, pipeline_overlay, pipeline_gui, pipeline_map, pipeline_screen],
            bindings: vec![bindings_main, bindings_overlay, bindings_gui, bindings_map, bindings_screen],
            mesh: vec![mesh_main, mesh_overlay, mesh_gui, mesh_map, mesh_screen],
            render_pass,
            proj,
            tx: tx.clone(),

            time_state: TimeState::init(),
            input_state: InputState::init(),
            request: false,
        }
    }

    fn show_data(&mut self) {
        self.overlay = text::Overlay::new_from(vec![
            &format!("FPS: {}", self.time_state.fps + 1),
            &format!("Press (Esc) for menu."),
            &format!("Position: ({:.1},{:.1})", self.player.position.x, self.player.position.y),
            &format!("Press (K) to take a screenshot."),
        ]);
    }

    fn show_gui(&mut self) {
        self.gui = text::GUI::new_from(vec![
            &format!("Continue"),
            &format!("-"),
            &format!("Fullscreen"),
            &format!("Light >"),
            &format!("Light <"),
            &format!("{}", if self.settings.music_playing {"Pause music"} else {"Resume music"}),
            &format!("-"),
            &format!("Quit game"),
        ], self.settings.screen_width_f, self.settings.screen_height_f);

        self.gui.gui_highlight(self.input_state.mouse.x, self.input_state.mouse.y);
    }
}

impl EventHandler for Stage {

    // ============================
    // UPDATE
    // ============================

    fn update(&mut self) {
        self.time_state.frame_time(&mut self.settings);
        self.show_data();

        if self.gui.show {
            self.show_gui();
            self.request = self.gui.gui_control(&self.input_state, &mut self.settings);
            if self.request {
                self.tx.send(self.settings.music_playing).unwrap();
            }
            self.input_state.apply_change = false;
            self.request = false
        }

        if self.input_state.keys.esc {
            self.gui.show = true
        }

        self.player.read_key(&self.input_state);

        self.input_state.mouse.moving = false;

        self.player.walk(
            &self.game_map,
            &self.settings,
            self.input_state.mouse.dx,
            self.input_state.mouse.dy,
            self.input_state.mouse.moving,
        );
        
        let rays = camera::ray_cast(&mut self.game_map, &self.player, &self.settings);
        self.face_buffer =
            camera::FaceBuffer::generate(&self.game_map, &self.player, &self.settings);

        self.sprite_buffer = sprites::SpriteBuffer::generate(&self.game_map, &self.player, &self.settings);

        self.mesh[0] = mesh::Mesh::new_main(&self.face_buffer, &self.sprite_buffer);
        self.mesh[1] = mesh::Mesh::new_overlay(
            &self.overlay,
            1.0 / self.settings.screen_width_f,
            1.0 / self.settings.screen_height_f,
            self.input_state.mouse.left
        );
        self.mesh[2] = mesh::Mesh::new_gui(
            &self.gui,
            1.0 / self.settings.screen_width_f,
            1.0 / self.settings.screen_height_f,
        );
        self.mesh[3] = mesh::Mesh::new_map(
            &rays,
            &self.player,
            &self.settings,
            1.0 / self.settings.screen_width_f,
            1.0 / self.settings.screen_height_f,
        );

        if self.input_state.keys.k && self.input_state.apply_change {
            let cap = (settings::WIDTH*settings::HEIGHT*4) as usize;
            let mut image: Vec<u8> = vec![0; cap];
            self.ctx.texture_read_pixels(self.ctx.render_pass_texture(self.render_pass), &mut image);
            image::save_buffer_with_format(format!("screenshot-{}.png", self.time_state.frame_count), &image, 
            settings::WIDTH as u32, settings::HEIGHT as u32, image::ColorType::Rgba8, 
                image::ImageFormat::Png).expect("Can't save screenshot");
            self.input_state.apply_change = false;
        }

        self.time_state.tick_count += 1;
    }

    // ============================
    // DRAW
    // ============================

    fn draw(&mut self) {
        window::show_mouse(self.gui.show);

        self.ctx.begin_default_pass(PassAction::Clear { color: Some((0.0, 0.0, 0.0, 1.0)), depth: None, stencil: None });

        self.ctx.apply_pipeline(&self.pipeline[4]);

        self.ctx.apply_bindings(&self.bindings[4]);

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsScreen {
            }));

        self.ctx.draw(0, self.mesh[4].num * 6, 1);

        self.ctx.end_render_pass();

        self.ctx
            .begin_pass(Some(self.render_pass), PassAction::default());

        for j in 0..self.bindings.len() {
            self.ctx.buffer_update(self.bindings[j].vertex_buffers[0], BufferSource::slice(&self.mesh[j].vertices));
            self.ctx.buffer_update(self.bindings[j].index_buffer, BufferSource::slice(&self.mesh[j].indices));
        }

        self.ctx.apply_pipeline(&self.pipeline[0]);

        self.ctx.apply_bindings(&self.bindings[0]);

        self.proj.update(&self.player, &self.settings);

        let lightpos = if self.sprite_buffer.len > 0 {
            (
                self.sprite_buffer.sprites_dist[self.sprite_buffer.len-1].x,
                self.sprite_buffer.sprites_dist[self.sprite_buffer.len-1].y,
                self.sprite_buffer.sprites_dist[self.sprite_buffer.len-1].z,
            )
        } else {
            (
                self.player.position.x,
                self.player.position.y,
                self.player.position.z,
            )
        };

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsMain {
                mvp: self.proj.mvp,
                playerpos: (
                    self.player.position.x,
                    self.player.position.y,
                    self.player.position.z,
                ),
                lightpos,
                lightdist: self.settings.light_dist,
            }));
        self.ctx.draw(0, self.mesh[0].num * 6, 1);

        self.ctx.apply_pipeline(&self.pipeline[1]);

        self.ctx.apply_bindings(&self.bindings[1]);

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsOverlay {
                fontcolor: self.overlay.font_col,
            }));

        self.ctx.draw(0, self.mesh[1].num * 6 + 6, 1);

        self.ctx.apply_pipeline(&self.pipeline[3]);

        self.ctx.apply_bindings(&self.bindings[3]);

        let mwidth = 2.0*(self.settings.draw_max_dist)*self.settings.tile_screen_size;
        let mheight = 2.0*(self.settings.draw_max_dist)*self.settings.tile_screen_size;
        let x_offset = 20.0;
        let y_offset = self.settings.screen_height_f - mheight - 20.0;
        let xp = x_offset + 0.5*mwidth;
        let yp = y_offset + 0.5*mheight;
        let x = 1.0 - xp / self.settings.screen_width_f;
        let y = 1.0 - yp / self.settings.screen_height_f;
        let a = mwidth / self.settings.screen_width_f / 2.0;
        let b = mwidth / self.settings.screen_height_f / 2.0;

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsMap {
                fontcolor: (0.14117647, 0.07843137, 0.13333333, 1.0),
                actcolor: (0.1843137, 0.2666667, 0.4627451, 1.0),
                cent: (x, y, a, b),
            }));

        self.ctx.draw(0, self.mesh[3].num * 3 + 6, 1);

        if self.gui.show {
            self.ctx.apply_pipeline(&self.pipeline[2]);

            self.ctx.apply_bindings(&self.bindings[2]);
    
            self.ctx
                .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsGUI {
                    fontcolor: self.gui.font_col,
                    actcolor: self.gui.act_col,
                }));
    
            self.ctx.draw(0, self.mesh[2].num * 6, 1);    
        }
        
        self.ctx.end_render_pass();

        self.ctx.commit_frame();

        self.time_state.last_frame = date::now();

        self.time_state.frame_count += 1;
    }

    // ============================
    // INPUT HANDLING
    // ============================

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input_state.keys.read_key(keycode, true);
        if !self.input_state.apply_change {
            self.input_state.apply_change = true
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.input_state.keys.read_key(keycode, false);
        self.input_state.apply_change = false
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.settings.screen_change(width, height);
    }

    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        self.input_state.mouse_motion(&self.settings, dx, dy);
        
        if self.gui.show {
            self.input_state.mouse.moving = false
        }
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
        if !self.input_state.apply_change {
            self.input_state.apply_change = true
        }
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.input_state.mouse.left = false;
        }
        if button == MouseButton::Right {
            self.input_state.mouse.right = false;
        }
        self.input_state.apply_change = false
    }
}