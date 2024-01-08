use std::cmp::Ordering;

use crate::settings;
use crate::map;
use crate::player;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub top_right_x: f32,
    pub top_right_y: f32,
    pub top_right_z: f32,
    pub bottom_right_x: f32,
    pub bottom_right_y: f32,
    pub bottom_right_z: f32,
    pub top_left_x: f32,
    pub top_left_y: f32,
    pub top_left_z: f32,
    pub bottom_left_x: f32,
    pub bottom_left_y: f32,
    pub bottom_left_z: f32,
    pub texture: u8,
    pub dist: f32,
}

impl Sprite {
    fn add_static_sprite(i: usize, j: usize, sprites: &mut Vec<Sprite>, game_map: &map::GameMap, player: &player::Player) -> usize {
        let xp = player.position.x;
        let yp = player.position.y;
        let texture = game_map.sprite_array[i][j];
        let mut res = 0;
        if texture < 255 {
            let xi = i as f32;
            let yj = j as f32;
            let dist = (xi+2.0/4.0-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
            let offset_z = -0.1;
            let sprite = Sprite {
                x: xi + 0.5,
                y: yj + 0.5,
                z: 2.0 + offset_z,
                top_right_x: xi + 1.0, 
                top_right_y: yj + 1.0, 
                top_right_z: 2.0 + offset_z, 
                bottom_right_x: xi + 1.0, 
                bottom_right_y: yj, 
                bottom_right_z: 2.0 + offset_z, 
                bottom_left_x: xi, 
                bottom_left_y: yj, 
                bottom_left_z: 2.0 + offset_z, 
                top_left_x: xi, 
                top_left_y: yj + 1.0, 
                top_left_z: 2.0 + offset_z, 
                texture, 
                dist, 
            };
            sprites.push(sprite);
            res = 1;
        }
        res
    }
}

pub struct SpriteBuffer {
    pub sprites_dist: Vec<Sprite>,
    pub len: usize,
}

impl SpriteBuffer {
    pub fn generate(game_map: &map::GameMap, player: &player::Player, settings: &settings::Settings) -> SpriteBuffer {
        let mut sprites: Vec<Sprite> = Vec::new();
        let mut len = 0;
        let xp = player.position.x;
        let yp = player.position.y;
        let ip = xp.floor() as i32;
        let jp = yp.floor() as i32;

        let i1 = (ip-(settings.draw_max_dist as i32)-2).max(0) as usize;
        let i2 = 1+(ip+(settings.draw_max_dist as i32)+2).min(settings::MAPSIZE as i32 - 1) as usize;
        let j1 = (jp-(settings.draw_max_dist as i32)-2).max(0) as usize;
        let j2 = 1+(jp+(settings.draw_max_dist as i32)+2).min(settings::MAPSIZE as i32 - 1) as usize;

        for i in i1..i2 {
            for j in j1..j2 {
                // if game_map.floor_visible[i][j] {
                //     len += Sprite::add_static_sprite(i, j, &mut sprites, game_map, player);
                // }
                len += Sprite::add_static_sprite(i, j, &mut sprites, game_map, player);
            }
        }

        sprites.sort_by(cmp_dist);

        SpriteBuffer {
            sprites_dist: sprites,
            len,
        }
    }
}

fn cmp_dist(a: &Sprite, b: &Sprite) -> Ordering {
    if a.dist < b.dist {
        return Ordering::Greater;
    } else if a.dist > b.dist {
        return Ordering::Less;
    }
    return Ordering::Equal;
}