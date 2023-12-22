use image::{self, Pixel, ImageBuffer, Rgba};

use crate::settings;

pub struct GameMap {
    pub map_array: [[u8; settings::MAPSIZE]; settings::MAPSIZE],
}

impl GameMap {
    pub fn new(map_image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> GameMap {
        let mut map_array = [[0; settings::MAPSIZE]; settings::MAPSIZE];

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let pixel = image::ImageBuffer::get_pixel(&map_image, i as u32, j as u32).to_rgba();
                map_array[i][settings::MAPSIZE-j-1] = pixel[0];
            }
        }

        GameMap { map_array }
    }
}
