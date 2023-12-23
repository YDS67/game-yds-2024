use image::{self, Pixel, ImageBuffer, Rgba};
use crate::settings;
use crate::assets;

pub struct GameMap {
    pub wall_array: [[u8; settings::MAPSIZE]; settings::MAPSIZE],
    pub floor_array: [[u8; settings::MAPSIZE]; settings::MAPSIZE],
    pub wall_visible: [[bool; settings::MAPSIZE]; settings::MAPSIZE],
    pub floor_visible: [[bool; settings::MAPSIZE]; settings::MAPSIZE],
}

impl GameMap {
    pub fn new(ass: &assets::Ass) -> GameMap {
        let mut wall_array = [[0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut floor_array = [[0; settings::MAPSIZE]; settings::MAPSIZE];

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let pixel1 = image::ImageBuffer::get_pixel(&ass.wall_image, i as u32, j as u32).to_rgba();
                let pixel2 = image::ImageBuffer::get_pixel(&ass.floor_image, i as u32, j as u32).to_rgba();
                wall_array[i][settings::MAPSIZE-j-1] = pixel1[0];
                floor_array[i][settings::MAPSIZE-j-1] = pixel2[0];
            }
        }

        let wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
        let floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];

        GameMap { 
            wall_array,
            floor_array,
            wall_visible,
            floor_visible,
        }
    }
}
