use image::{self, ImageBuffer, Rgba};
use macroquad::prelude::*;
use std::path::Path;

pub struct Ass {
    pub font_main: Font,
    pub wall_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub floor_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub wall_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub async fn load() -> Ass {
        Ass {
            font_main: load_ttf_font("resources/times.ttf").await.unwrap(),
            wall_atlas: image::open(Path::new("resources/walls0.png"))
                .unwrap()
                .to_rgba8(),
            floor_image: image::open(Path::new("resources/map_floor.png"))
                .unwrap()
                .to_rgba8(),
            wall_image: image::open(Path::new("resources/map_walls.png"))
                .unwrap()
                .to_rgba8(),
        }
    }
}
