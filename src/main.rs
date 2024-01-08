#![windows_subsystem = "windows"]

use miniquad::{self, conf::Platform, conf::Conf};

use std::thread;

mod assets;
mod camera;
mod map;
mod mesh;
mod player;
mod settings;
mod shaders;
mod stage;
mod text;
mod input;
mod sprites;
mod audio;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Game".to_owned(),
        window_width: settings::WIDTH0,
        window_height: settings::HEIGHT0,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(0);
    conf
}

fn main() {
    thread::spawn(|| {audio::playback()});
    miniquad::start(window_conf(), move || Box::new(stage::Stage::new()));
    
}