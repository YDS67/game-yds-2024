pub const PI: f32 = 3.1415926538;
pub const MAPSIZE: usize = 256;
pub const WIDTH0: i32 = 1280;
pub const HEIGHT0: i32 = 800;
pub const TOLERANCE: f32 = 1e-3;
pub const FT_DESIRED: f32 = 0.01666666666667;
pub const MAX_FACES_MAIN: usize = 50000;
pub const MAX_VERTICES_MAIN: usize = MAX_FACES_MAIN*4;
pub const MAX_INDICES_MAIN: usize = MAX_FACES_MAIN*6;

pub struct Settings {
    pub screen_width: i32,
    pub screen_height: i32,
    pub full_screen: bool,
    pub draw_map: bool,
    pub draw_menu: bool,
    pub screen_width_f: f32,
    pub screen_height_f: f32,
    pub screen_aspect: f32,
    pub player_height: f32,
    pub tile_screen_size: f32,
    pub map_offset_x: f32,
    pub map_offset_y: f32,
    pub player_x0: f32,
    pub player_y0: f32,
    pub player_a0: f32,
    pub player_b0: f32,
    pub fov_xy: f32,
    pub fov_z: f32,
    pub delta_time: f32,
    pub player_speed: f32,
    pub player_radius: f32,
    pub draw_max_dist: usize,
    pub draw_steps: usize,
    pub draw_min_dist: usize,
    pub light_dist: f32,
    pub draw_rays_num: usize,
    pub mouse_sensitivity: f32,
}

impl Settings {
    pub fn init() -> Settings {
        let screen_width = 1280;
        let screen_height = 800;
        let full_screen = false;
        let draw_map = false;
        let draw_menu = false;
        let screen_width_f = screen_width as f32;
        let screen_height_f = screen_height as f32;
        let screen_aspect = screen_width_f/screen_height_f;
        let player_height = 0.5;
        let tile_screen_size = 2.0;
        let map_offset_x = 20.0;
        let map_offset_y = screen_height_f - tile_screen_size * (MAPSIZE as f32) - 20.0;
        let player_x0 = MAPSIZE as f32 / 2.0;
        let player_y0 = 4.5;
        let player_a0 = 1.55;
        let player_b0 = 0.0;
        let fov_xy = PI / 4.0;
        let fov_z = fov_xy / screen_aspect;
        let delta_time = 1.0/60.0;
        let player_speed = 12.0*delta_time;
        let player_radius = 0.5;
        let draw_max_dist = 100;
        let draw_steps = 10;
        let draw_min_dist = 3*3;
        let light_dist = 10.0;
        let draw_rays_num = 750;
        let mouse_sensitivity = 10.0;
        Settings {
            screen_width,
            screen_height,
            full_screen,
            draw_map,
            draw_menu,
            screen_width_f,
            screen_height_f,
            screen_aspect,
            player_height,
            tile_screen_size,
            map_offset_x,
            map_offset_y,
            player_x0,
            player_y0,
            player_a0,
            player_b0,
            fov_xy,
            fov_z,
            delta_time,
            player_speed,
            player_radius,
            draw_max_dist,
            draw_steps,
            draw_min_dist,
            light_dist,
            draw_rays_num,
            mouse_sensitivity,
        }
    }

    pub fn screen_change(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width as i32;
        self.screen_height = screen_height as i32;
        self.screen_width_f = screen_width;
        self.screen_height_f = screen_height;
        self.screen_aspect = screen_width/screen_height;
        self.map_offset_y = screen_height - self.tile_screen_size * (MAPSIZE as f32) - 20.0;
    }
}
