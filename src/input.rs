use miniquad::*;
use std::thread::sleep;
use std::time::Duration;

use crate::settings;

pub struct TimeState {
    pub last_frame: f64,
    pub frame_time: f64,
    pub fps: i32,
    pub frame_count: i32,
    pub tick_count: i32,
}

impl TimeState {
    pub fn init() -> TimeState {
        TimeState {
            last_frame: date::now(),
            frame_time: 1.0 / 60.0,
            fps: 60,
            frame_count: 0,
            tick_count: 0,
        }
    }

    pub fn frame_time(&mut self, settings: &mut settings::Settings) {
        self.frame_time = date::now() - self.last_frame;
        if self.frame_time < settings::FT_DESIRED {
            sleep(Duration::from_secs_f64(
                settings::FT_DESIRED - self.frame_time,
            ));
        }
        self.frame_time = date::now() - self.last_frame;
        settings.delta_time = self.frame_time as f32;
        self.fps = (1. / self.frame_time).floor() as i32;

        settings.player_speed = 12.0 * settings.delta_time;
    }
}

pub struct KeysState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub k: bool,
    pub l: bool,
    pub f: bool,
    pub m: bool,
    pub esc: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
    pub enter: bool,
}

impl KeysState {
    pub fn read_key(&mut self, keycode: KeyCode, state: bool) {
        match keycode {
            KeyCode::W => self.w = state,
            KeyCode::S => self.s = state,
            KeyCode::A => self.a = state,
            KeyCode::D => self.d = state,
            KeyCode::Left => self.left = state,
            KeyCode::Right => self.right = state,
            KeyCode::Up => self.up = state,
            KeyCode::Down => self.down = state,
            KeyCode::Space => self.space = state,
            KeyCode::Escape => self.esc = state,
            KeyCode::Enter => self.enter = state,
            KeyCode::K => self.k = state,
            KeyCode::L => self.l = state,
            KeyCode::Q => self.q = state,
            KeyCode::E => self.e = state,
            KeyCode::F => self.f = state,
            KeyCode::M => self.m = state,
            _ => {},
        }
    }
}

pub struct MouseState {
    pub left: bool,
    pub right: bool,
    pub moving: bool,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct InputState {
    pub keys: KeysState,
    pub mouse: MouseState,
    pub apply_change: bool,
}

impl InputState {
    pub fn init() -> InputState {
        InputState {
            keys: KeysState {
                w: false,
                a: false,
                s: false,
                d: false,
                q: false,
                e: false,
                k: false,
                l: false,
                f: false,
                m: false,
                left: false,
                right: false,
                up: false,
                down: false,
                space: false,
                enter: false,
                esc: false,
            },
            mouse: MouseState {
                left: false,
                right: false,
                moving: false,
                x: 0.0,
                y: 0.0,
                dx: 0.5 * settings::TOLERANCE,
                dy: 0.5 * settings::TOLERANCE,
            },
            apply_change: false,
        }
    }

    pub fn mouse_motion(&mut self, settings: &settings::Settings, dx: f32, dy: f32) {
        let moving_x;
        let moving_y;
        if dx.abs() < settings::TOLERANCE*settings.screen_width_f
        {
            self.mouse.dx = 0.0;
            moving_x = false;
        } else {
            self.mouse.dx = 0.5 * dx / settings.screen_width_f;
            moving_x = true;
        }
        if dy.abs() < settings::TOLERANCE*settings.screen_width_f
        {
            self.mouse.dy = 0.0;
            moving_y = false;
        } else {
            self.mouse.dy = 0.5 * dy / settings.screen_width_f;
            moving_y = true;
        }
        self.mouse.moving = moving_x || moving_y;
    }
}
