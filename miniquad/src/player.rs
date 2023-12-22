use macroquad::prelude::*;
use crate::settings;

pub struct PlayerPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
    pub b: f32,
    pub ax: f32,
    pub ay: f32,
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
            },
            size: settings::PLAYERSIZE,
        }
    }
    pub fn draw(&self) {
        let x = 10.0 + self.position.x * settings::TILESCREENSIZE;
        let y = screen_height() - 10.0 - self.position.y * settings::TILESCREENSIZE;
        let s = self.size * settings::TILESCREENSIZE;
        let x1 = x + self.position.ax * settings::TILESCREENSIZE * 10.0;
        let y1 = y - self.position.ay * settings::TILESCREENSIZE * 10.0;
    
        draw_line(x, y, x1, y1, 6.0, BLUE);
        draw_circle(x, y, s, RED);
    }
    
    pub fn walk(&mut self) {
        if is_key_down(KeyCode::W) {
            self.position.x = self.position.x + settings::PLAYERSPEED * self.position.ax;
            self.position.y = self.position.y + settings::PLAYERSPEED * self.position.ay;
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
            self.position.x = self.position.x - settings::PLAYERSPEED * self.position.ax;
            self.position.y = self.position.y - settings::PLAYERSPEED * self.position.ay;
        }
    }
}