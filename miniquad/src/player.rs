use crate::map;
use crate::settings;
use macroquad::prelude::*;

pub struct PlayerPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
    pub b: f32,
    pub ax: f32,
    pub ay: f32,
    pub bxy: f32,
    pub bz: f32,
    pub cxp: bool,
    pub cyp: bool,
    pub cxm: bool,
    pub cym: bool,
    pub cxl: bool,
    pub cyl: bool,
    pub cxr: bool,
    pub cyr: bool,
}

pub struct Player {
    pub position: PlayerPos,
    pub radius: f32,
}

impl Player {
    pub fn new(settings: &settings::Settings) -> Player {
        let a = settings.player_a0;
        let b = settings.player_b0;
        Player {
            position: PlayerPos {
                x: settings.player_x0,
                y: settings.player_y0,
                z: settings.player_height,
                a,
                b,
                ax: a.cos(),
                ay: a.sin(),
                bxy: b.cos(),
                bz: b.sin(),
                cxp: false,
                cyp: false,
                cxm: false,
                cym: false,
                cxl: false,
                cyl: false,
                cxr: false,
                cyr: false,
            },
            radius: settings.player_radius,
        }
    }

    pub fn draw(&self, settings: &settings::Settings) {
        let x = settings.map_offset_x + self.position.x * settings.tile_screen_size;
        let y = 20.0 + (settings::MAPSIZE as f32 - self.position.y) * settings.tile_screen_size;
        let s = self.radius * settings.tile_screen_size * 2.0;

        draw_circle(x, y, s, RED);
    }

    fn coll_check(&mut self, game_map: &map::GameMap) {
        let i = (self.position.x).floor() as usize;
        let j = (self.position.y).floor() as usize;
        let ip = (self.position.x + self.radius * self.position.ax).floor() as usize;
        let jp = (self.position.y + self.radius * self.position.ay).floor() as usize;
        let im = (self.position.x - self.radius * self.position.ax).floor() as usize;
        let jm = (self.position.y - self.radius * self.position.ay).floor() as usize;
        let il = (self.position.x - self.radius * self.position.ay).floor() as usize;
        let jl = (self.position.y + self.radius * self.position.ax).floor() as usize;
        let ir = (self.position.x + self.radius * self.position.ay).floor() as usize;
        let jr = (self.position.y - self.radius * self.position.ax).floor() as usize;

        self.position.cxp = false;
        self.position.cxm = false;
        self.position.cyp = false;
        self.position.cym = false;
        self.position.cxl = false;
        self.position.cxr = false;
        self.position.cyl = false;
        self.position.cyr = false;

        if game_map.wall_array[ip][jp] < 255
            || game_map.wall_array[im][jm] < 255
            || game_map.wall_array[il][jl] < 255
            || game_map.wall_array[ir][jr] < 255
        {
            self.position.cxp = true;
            self.position.cxm = true;
            self.position.cyp = true;
            self.position.cym = true;
            self.position.cxl = true;
            self.position.cxr = true;
            self.position.cyl = true;
            self.position.cyr = true;
        }

        if game_map.wall_array[ip][j] == 255 {
            self.position.cxp = false;
        }

        if game_map.wall_array[i][jp] == 255 {
            self.position.cyp = false;
        }

        if game_map.wall_array[im][j] == 255 {
            self.position.cxm = false;
        }

        if game_map.wall_array[i][jm] == 255 {
            self.position.cym = false;
        }

        if game_map.wall_array[il][j] == 255 {
            self.position.cxl = false;
        }

        if game_map.wall_array[ir][j] == 255 {
            self.position.cxr = false;
        }

        if game_map.wall_array[i][jl] == 255 {
            self.position.cyl = false;
        }

        if game_map.wall_array[i][jr] == 255 {
            self.position.cyr = false;
        }
    }

    pub fn walk(&mut self, game_map: &map::GameMap, settings: &settings::Settings) {
        self.coll_check(game_map);

        if is_key_down(KeyCode::W) {
            if !self.position.cxp {
                self.position.x = self.position.x + settings.player_speed * self.position.ax;
            }
            if !self.position.cyp {
                self.position.y = self.position.y + settings.player_speed * self.position.ay;
            }
        }

        if is_key_down(KeyCode::S) {
            if !self.position.cxm {
                self.position.x = self.position.x - settings.player_speed * self.position.ax;
            }
            if !self.position.cym {
                self.position.y = self.position.y - settings.player_speed * self.position.ay;
            }
        }

        if is_key_down(KeyCode::A) {
            if !self.position.cxl {
                self.position.x = self.position.x - settings.player_speed * self.position.ay;
            }
            if !self.position.cyl {
                self.position.y = self.position.y + settings.player_speed * self.position.ax;
            }
        }

        if is_key_down(KeyCode::D) {
            if !self.position.cxr {
                self.position.x = self.position.x + settings.player_speed * self.position.ay;
            }
            if !self.position.cyr {
                self.position.y = self.position.y - settings.player_speed * self.position.ax;
            }
        }

        if is_key_down(KeyCode::Left) {
            self.position.a = angle_round(self.position.a + 0.1 * settings.player_speed);
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if is_key_down(KeyCode::Right) {
            self.position.a = angle_round(self.position.a - 0.1 * settings.player_speed);
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if is_key_down(KeyCode::Down) && self.position.b < settings::PI / 4.0 {
            self.position.b = self.position.b + 0.1 * settings.player_speed;
            self.position.bxy = self.position.b.cos();
            self.position.bz = self.position.b.sin();
        }

        if is_key_down(KeyCode::Up) && self.position.b > -settings::PI / 4.0 {
            self.position.b = self.position.b - 0.1 * settings.player_speed;
            self.position.bxy = self.position.b.cos();
            self.position.bz = self.position.b.sin();
        }
    }
}

pub fn angle_round(angle: f32) -> f32 {
    let mut in_degrees = angle * 180.0 / settings::PI;
    while in_degrees < -180.0 {
        in_degrees = 360.0 + in_degrees
    }
    while in_degrees > 179.9 {
        in_degrees = -360.0 + in_degrees
    }
    in_degrees * settings::PI / 180.0
}
