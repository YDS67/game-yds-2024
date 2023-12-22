use macroquad::prelude::*;
use image::{self, ImageBuffer, Rgba};
use std::path::Path;

pub struct Ass {
    pub font_main: Font,
    pub wall_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub async fn load() -> Ass {
        Ass {
            font_main: load_ttf_font("resources/times.ttf")
            .await
            .unwrap(),
            wall_atlas: image::open(Path::new("resources/walls.png"))
            .unwrap().to_rgba8(),
        }
    }
}
