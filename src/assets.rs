use image::{self, ImageBuffer, Rgba};
use std::path::Path;

use crate::settings;

const DEF_IMAGE_SIZE: u32 = settings::MAPSIZE as u32;

pub struct Ass {
    pub tile_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub sprite_atlas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub floor_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub ceil_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub sprite_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub wall_image_bot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub wall_image_top: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub font: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub fn load() -> Ass {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(DEF_IMAGE_SIZE, DEF_IMAGE_SIZE);

        for pixel in img.enumerate_pixels_mut() {
            *pixel.2 = image::Rgba([255,255,255,255]);
        }

        let mut images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();

        let paths: Vec<&str> = vec![
            "assets/texture_atlas.png",
            "assets/sprite_atlas.png",
            "assets/floor_map.png",
            "assets/ceil_map.png",
            "assets/sprite_map.png",
            "assets/wall_map_bot.png",
            "assets/wall_map_top.png",
            "assets/monospace.png",
        ];

        for path in paths {
            let image_result = image::open(Path::new(path));
            let image = match image_result {
                Ok(image_result) => image_result.to_rgba8(),
                Err(_image_result) => img.clone()
            };
            images.push(image)
        }

        Ass {
            tile_atlas: images[0].clone(),
            sprite_atlas: images[1].clone(),
            floor_image: images[2].clone(),
            ceil_image: images[3].clone(),
            sprite_image: images[4].clone(),
            wall_image_bot: images[5].clone(),
            wall_image_top: images[6].clone(),
            font: images[7].clone(),
        }
    }
}
