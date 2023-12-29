#![windows_subsystem = "windows"]

use miniquad::{self, conf::Platform, conf::Conf};

mod assets;
mod camera;
mod map;
mod mesh;
mod player;
mod settings;
mod shaders;
mod stage;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Raycasting + GPU rendering".to_owned(),
        window_width: settings::WIDTH0,
        window_height: settings::HEIGHT0,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(-1);
    conf
}

fn main() {
    miniquad::start(window_conf(), move || Box::new(stage::Stage::new()));
}