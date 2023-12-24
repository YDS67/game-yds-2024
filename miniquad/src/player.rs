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
    pub bx: f32,
    pub by: f32,
    pub cxp: bool,
    pub cyp: bool,
    pub cxm: bool,
    pub cym: bool,
}

pub struct Player {
    pub position: PlayerPos,
    pub size: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: PlayerPos {
                x: settings::PLAYERX0,
                y: settings::PLAYERY0,
                z: settings::PLAYERHEIGHT,
                a: settings::PLAYERA0,
                b: settings::PLAYERB0,
                ax: settings::PLAYERA0.cos(),
                ay: settings::PLAYERA0.sin(),
                bx: settings::PLAYERB0.cos(),
                by: settings::PLAYERB0.sin(),
                cxp: false,
                cyp: false,
                cxm: false,
                cym: false,
            },
            size: settings::PLAYERSIZE,
        }
    }

    pub fn draw(&self) {
        let x = settings::MAPOFFSETX + self.position.x * settings::TILESCREENSIZE;
        let y = settings::HEIGHTF - 10.0 - self.position.y * settings::TILESCREENSIZE;
        let s = self.size * settings::TILESCREENSIZE * 2.0;

        draw_circle(x, y, s, RED);
    }

    fn coll_check(&mut self, game_map: &map::GameMap) {
        let i = (self.position.x).floor() as usize;
        let j = (self.position.y).floor() as usize;
        let ip = (self.position.x + self.size).floor() as usize;
        let jp = (self.position.y + self.size).floor() as usize;
        let im = (self.position.x - self.size).floor() as usize;
        let jm = (self.position.y - self.size).floor() as usize;

        self.position.cxp = false;
        self.position.cxm = false;
        self.position.cyp = false;
        self.position.cym = false;

        if game_map.wall_array[ip][j] < 255
            || game_map.wall_array[i][jp] < 255
            || game_map.wall_array[im][j] < 255
            || game_map.wall_array[i][jm] < 255
            || game_map.wall_array[ip][jp] < 255
            || game_map.wall_array[im][jm] < 255
            || game_map.wall_array[im][jp] < 255
            || game_map.wall_array[ip][jm] < 255
        {
            self.position.cxp = true;
            self.position.cxm = true;
            self.position.cyp = true;
            self.position.cym = true;
        } 

        let ip = (self.position.x + self.size * self.position.ax).floor() as usize;
        let jp = (self.position.y + self.size * self.position.ay).floor() as usize;
        let im = (self.position.x - self.size * self.position.ax).floor() as usize;
        let jm = (self.position.y - self.size * self.position.ay).floor() as usize;

        if game_map.wall_array[ip][j] == 255
        {
            self.position.cxp = false;
        }

        if game_map.wall_array[i][jp] == 255
        {
            self.position.cyp = false;
        } 

        if game_map.wall_array[im][j] == 255
        {
            self.position.cxm = false;
        }

        if game_map.wall_array[i][jm] == 255
        {
            self.position.cym = false;
        }
    }

    pub fn walk(&mut self, game_map: &map::GameMap) {
        self.coll_check(game_map);

        if is_key_down(KeyCode::W) {
            if !self.position.cxp {
                self.position.x = self.position.x + settings::PLAYERSPEED * self.position.ax;
            }
            if !self.position.cyp {
                self.position.y = self.position.y + settings::PLAYERSPEED * self.position.ay;
            }
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.position.a = angle_round(self.position.a + 0.2 * settings::PLAYERSPEED);
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.position.a = angle_round(self.position.a - 0.2 * settings::PLAYERSPEED);
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if is_key_down(KeyCode::S) {
            if !self.position.cxm {
                self.position.x = self.position.x - settings::PLAYERSPEED * self.position.ax;
            }
            if !self.position.cym {
                self.position.y = self.position.y - settings::PLAYERSPEED * self.position.ay;
            }
        }

        if is_key_down(KeyCode::Down) && self.position.b < settings::PI/2.0 {
            self.position.b = self.position.b + 0.1 * settings::PLAYERSPEED;
            self.position.bx = self.position.b.cos();
            self.position.by = self.position.b.sin();
        }

        if is_key_down(KeyCode::Up) && self.position.b > -settings::PI/2.0 {
            self.position.b = self.position.b - 0.1 * settings::PLAYERSPEED;
            self.position.bx = self.position.b.cos();
            self.position.by = self.position.b.sin();
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