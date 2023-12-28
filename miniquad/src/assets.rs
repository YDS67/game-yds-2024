use image::{self, ImageBuffer, Rgba};
use std::path::Path;

pub struct Ass {
    pub wall_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub floor_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub wall_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub fn load() -> Ass {
        Ass {
            wall_atlas: image::open(Path::new("resources/walls.png"))
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
