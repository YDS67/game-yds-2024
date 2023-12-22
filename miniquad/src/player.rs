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
                cxp: false,
                cyp: false,
                cxm: false,
                cym: false,
            },
            size: settings::PLAYERSIZE,
        }
    }

    pub fn draw(&self) {
        let x = 10.0 + self.position.x * settings::TILESCREENSIZE;
        let y = screen_height() - 10.0 - self.position.y * settings::TILESCREENSIZE;
        let s = self.size * settings::TILESCREENSIZE * 10.0;
        let x1 = x + self.position.ax * settings::TILESCREENSIZE * 10.0;
        let y1 = y - self.position.ay * settings::TILESCREENSIZE * 10.0;

        draw_line(x, y, x1, y1, 6.0, BLUE);
        draw_circle(x, y, s, RED);
    }

    fn coll_check(&mut self, game_map: &map::GameMap) {
        let i = (self.position.x).floor() as usize;
        let j = (self.position.y).floor() as usize;
        let ip = (self.position.x + self.size * self.position.ax).floor() as usize;
        let jp = (self.position.y + self.size * self.position.ay).floor() as usize;
        let im = (self.position.x - self.size * self.position.ax).floor() as usize;
        let jm = (self.position.y - self.size * self.position.ay).floor() as usize;
        if game_map.map_array[ip][j] < 255
        {
            self.position.cxp = true;
        } else {
            self.position.cxp = false;
        }
        if game_map.map_array[i][jp] < 255
        {
            self.position.cyp = true;
        } else {
            self.position.cyp = false;
        }
        if game_map.map_array[im][j] < 255
        {
            self.position.cxm = true;
        } else {
            self.position.cxm = false;
        }
        if game_map.map_array[i][jm] < 255
        {
            self.position.cym = true;
        } else {
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

        if is_key_down(KeyCode::A) {
            self.position.a = self.position.a + 0.1 * settings::PLAYERSPEED;
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if is_key_down(KeyCode::D) {
            self.position.a = self.position.a - 0.2 * settings::PLAYERSPEED;
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
    }
}
