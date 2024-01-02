use image::{self, ImageBuffer, Rgba};
use std::path::Path;

pub struct Ass {
    pub tile_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub floor_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub ceil_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub wall_image_bot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub wall_image_top: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub font: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub fn load() -> Ass {
        Ass {
            tile_atlas: image::open(Path::new("resources/texture_atlas.png"))
                .unwrap()
                .to_rgba8(),
            floor_image: image::open(Path::new("resources/floor_map.png"))
                .unwrap()
                .to_rgba8(),
            ceil_image: image::open(Path::new("resources/ceil_map.png"))
                .unwrap()
                .to_rgba8(),
            wall_image_bot: image::open(Path::new("resources/wall_map_bot.png"))
                .unwrap()
                .to_rgba8(),
            wall_image_top: image::open(Path::new("resources/wall_map_top.png"))
                .unwrap()
                .to_rgba8(),
            font: image::open(Path::new("resources/monospace.png"))
                .unwrap()
                .to_rgba8(),
        }
    }
}
