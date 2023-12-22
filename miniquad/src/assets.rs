use image::{self, ImageBuffer, Rgba};
use macroquad::prelude::*;
use std::path::Path;

pub struct Ass {
    pub font_main: Font,
    pub wall_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub map_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub async fn load() -> Ass {
        Ass {
            font_main: load_ttf_font("resources/times.ttf").await.unwrap(),
            wall_atlas: image::open(Path::new("resources/walls0.png"))
                .unwrap()
                .to_rgba8(),
            map_image: image::open(Path::new("resources/map0.png"))
                .unwrap()
                .to_rgba8(),
        }
    }
}
