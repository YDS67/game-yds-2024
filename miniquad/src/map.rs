use crate::assets;
use crate::settings;
use image::{self, Pixel, DynamicImage, ImageBuffer, Rgba};
use std::path::Path;

pub struct GameMap {
    pub wall_array: [[u8; settings::MAPSIZE]; settings::MAPSIZE],
    pub floor_array: [[u8; settings::MAPSIZE]; settings::MAPSIZE],
    pub dist_field: [[u8; settings::MAPSIZE]; settings::MAPSIZE],
    pub dmax: u8,
    pub wall_visible: [[bool; settings::MAPSIZE]; settings::MAPSIZE],
    pub floor_visible: [[bool; settings::MAPSIZE]; settings::MAPSIZE],
    pub wall_dist: [[usize; settings::MAPSIZE]; settings::MAPSIZE],
    pub floor_dist: [[usize; settings::MAPSIZE]; settings::MAPSIZE],
}

impl GameMap {
    pub fn new(ass: &assets::Ass) -> GameMap {
        let mut wall_array = [[0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut floor_array = [[0; settings::MAPSIZE]; settings::MAPSIZE];
        let mut dist_field = [[0; settings::MAPSIZE]; settings::MAPSIZE];

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let pixel1 =
                    image::ImageBuffer::get_pixel(&ass.wall_image, i as u32, j as u32).to_rgba();
                let pixel2 =
                    image::ImageBuffer::get_pixel(&ass.floor_image, i as u32, j as u32).to_rgba();
                 let pixel3 =
                     image::ImageBuffer::get_pixel(&ass.dist_image, i as u32, j as u32).to_rgba();
                wall_array[i][settings::MAPSIZE - j - 1] = pixel1[0];
                floor_array[i][settings::MAPSIZE - j - 1] = pixel2[0];
                dist_field[i][settings::MAPSIZE - j - 1] = pixel3[0];
            }
        }

        let dmax = 255;
        //let mut dmax: u8 = 0;

        // for i in 0..settings::MAPSIZE {
        //     for j in 0..settings::MAPSIZE {
        //         for d in 1..(4*settings::MAPSIZE) {
        //             for k in 0..settings::MAPSIZE {
        //                 for l in 0..settings::MAPSIZE {
        //                     if (i - k) * (i - k) + (j - l) * (j - l) <= ((d * d) as f32 * 0.25) as usize
        //                         && wall_array[k][l] < 255
        //                     {
        //                         dist_field[i][j] = d as u8;
        //                         if d as u8 > dmax {
        //                             dmax = d as u8
        //                         }
        //                         break;
        //                     }
        //                 }
        //                 if dist_field[i][j] == d as u8 {
        //                     break;
        //                 }
        //             }
        //             if dist_field[i][j] == d as u8 {
        //                 break;
        //             }
        //         }
        //     }
        // }

        // let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(settings::MAPSIZE as u32, settings::MAPSIZE as u32);

        // for i in 0..settings::MAPSIZE {
        //     for j in 0..settings::MAPSIZE {
        //         let d = dist_field[i][settings::MAPSIZE - j - 1];
        //         let b = (d as f32 / dmax as f32 * 255.0) as u8;
        //         img.put_pixel(i as u32, j as u32, image::Rgba([b,b,b,255]));
        //     }
        // }

        // img.save(Path::new("resources/dist_field_256.png")).unwrap();

        let wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
        let floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];

        let wall_dist = [[settings::MAXDRAWDIST; settings::MAPSIZE]; settings::MAPSIZE];
        let floor_dist = [[settings::MAXDRAWDIST; settings::MAPSIZE]; settings::MAPSIZE];

        GameMap {
            wall_array,
            floor_array,
            dist_field,
            dmax,
            wall_visible,
            floor_visible,
            wall_dist,
            floor_dist,
        }
    }
}
