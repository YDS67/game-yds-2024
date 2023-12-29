use crate::assets;
use crate::settings;
use image::{self, Pixel};

pub struct GameMap {
    pub wall_array: Vec<Vec<u8>>,
    pub floor_array: Vec<Vec<u8>>,
    pub wall_visible: Vec<Vec<bool>>,
    pub floor_visible: Vec<Vec<bool>>,
    pub wall_dist: Vec<Vec<f32>>,
    pub floor_dist: Vec<Vec<f32>>,
}

impl GameMap {
    pub fn new(ass: &assets::Ass) -> GameMap {
        let mut wall_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut floor_array = vec![vec![0; settings::MAPSIZE]; settings::MAPSIZE];

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let pixel1 =
                    image::ImageBuffer::get_pixel(&ass.wall_image_bot, i as u32, j as u32).to_rgba();
                let pixel2 =
                    image::ImageBuffer::get_pixel(&ass.floor_image, i as u32, j as u32).to_rgba();
                wall_array[i][settings::MAPSIZE - j - 1] = pixel1[0];
                floor_array[i][settings::MAPSIZE - j - 1] = pixel2[0];
            }
        }

        let wall_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];
        let floor_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];

        let wall_dist = vec![vec![1.0; settings::MAPSIZE]; settings::MAPSIZE];
        let floor_dist = vec![vec![1.0; settings::MAPSIZE]; settings::MAPSIZE];

        GameMap {
            wall_array,
            floor_array,
            wall_visible,
            floor_visible,
            wall_dist,
            floor_dist,
        }
    }
}
