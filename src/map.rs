use crate::assets;
use crate::settings;
use image::{self, Pixel};

pub struct GameMap {
    pub wall_top_array: Vec<Vec<u8>>,
    pub wall_bot_array: Vec<Vec<u8>>,
    pub floor_array: Vec<Vec<u8>>,
    pub ceil_array: Vec<Vec<u8>>,
    pub sprite_array: Vec<Vec<u8>>,
    pub wall_visible: Vec<Vec<bool>>,
    pub floor_visible: Vec<Vec<bool>>,
}

impl GameMap {
    pub fn new(ass: &assets::Ass) -> GameMap {
        let mut wall_top_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut wall_bot_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut floor_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut ceil_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut sprite_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let pixel1 =
                    image::ImageBuffer::get_pixel(&ass.wall_image_bot, i as u32, j as u32).to_rgba();
                let pixel2 =
                    image::ImageBuffer::get_pixel(&ass.floor_image, i as u32, j as u32).to_rgba();
                wall_bot_array[i][settings::MAPSIZE - j - 1] = pixel1[0];
                floor_array[i][settings::MAPSIZE - j - 1] = pixel2[0];
                let pixel1 =
                    image::ImageBuffer::get_pixel(&ass.wall_image_top, i as u32, j as u32).to_rgba();
                let pixel2 =
                    image::ImageBuffer::get_pixel(&ass.ceil_image, i as u32, j as u32).to_rgba();
                wall_top_array[i][settings::MAPSIZE - j - 1] = pixel1[0];
                ceil_array[i][settings::MAPSIZE - j - 1] = pixel2[0];
                let pixel1 =
                    image::ImageBuffer::get_pixel(&ass.sprite_image, i as u32, j as u32).to_rgba();
                    sprite_array[i][settings::MAPSIZE - j - 1] = pixel1[2];
            }
        }

        let wall_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];
        let floor_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];

        GameMap {
            wall_top_array,
            wall_bot_array,
            floor_array,
            ceil_array,
            sprite_array,
            wall_visible,
            floor_visible,
        }
    }
}
