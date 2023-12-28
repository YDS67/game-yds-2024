pub const PI: f32 = 3.1415926538;
pub const MAPSIZE: usize = 256;
pub const WIDTH0: i32 = 1280;
pub const HEIGHT0: i32 = 800;

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
    pub player_x0: f32,
    pub player_y0: f32,
    pub player_a0: f32,
    pub player_b0: f32,
    pub fov_xy: f32,
    pub fov_z: f32,
    pub delta_tile: f32,
    pub player_speed: f32,
    pub player_radius: f32,
    pub draw_max_dist: usize,
    pub draw_min_dist: usize,
    pub light_dist: f32,
    pub draw_rays_num: usize,
    pub tolerance: f32,
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
        let player_height = 1.0;
        let tile_screen_size = 1.0;
        let map_offset_x = screen_width_f - tile_screen_size * (MAPSIZE as f32) - 20.0;
        let player_x0 = 4.5;
        let player_y0 = 4.5;
        let player_a0 = 0.75;
        let player_b0 = 0.0;
        let fov_xy = PI / 4.0;
        let fov_z = PI / 4.0 / screen_aspect;
        let delta_tile = 1.0/60.0;
        let player_speed = 0.2;
        let player_radius = 0.5;
        let draw_max_dist = MAPSIZE*2;
        let draw_min_dist = 5*5;
        let light_dist = ((MAPSIZE as f32)/15.0).powi(2);
        let draw_rays_num = 250;
        let tolerance = 1e-16;
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
            player_x0,
            player_y0,
            player_a0,
            player_b0,
            fov_xy,
            fov_z,
            delta_tile,
            player_speed,
            player_radius,
            draw_max_dist,
            draw_min_dist,
            light_dist,
            draw_rays_num,
            tolerance,
        }
    }
}
