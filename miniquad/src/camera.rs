use std::cmp::Ordering;

use crate::player;
use crate::settings;
use crate::map;

pub struct DepthBuffer {
    pub visible_tiles: Vec<[usize; 4]>,
    pub dmax: usize,
}

impl DepthBuffer {
    pub fn generate(game_map: &map::GameMap) -> DepthBuffer {
        let mut visible_tiles = Vec::new();
        let mut dmax = 0;

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if game_map.wall_visible[i][j] {
                    let d = game_map.wall_dist[i][j];
                    if d > dmax {dmax = d}
                    visible_tiles.push([d, i, j, 2])
                }
                if game_map.floor_visible[i][j] {
                    let d = game_map.floor_dist[i][j];
                    if d > dmax {dmax = d}
                    visible_tiles.push([d, i, j, 1])
                }
            }
        }

        visible_tiles.sort_by(cmp_depth);

        DepthBuffer {
            visible_tiles,
            dmax,
        }
    }
}

pub fn find_visible_tiles(game_map: &mut map::GameMap, player: &player::Player) {
    game_map.wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    for k in 0..=settings::NUMRAYS {
        let phi = player.position.a + settings::FOVXY * (0.5 - (k as f32)/(settings::NUMRAYS as f32));
        let cphi = phi.cos();
        let sphi = phi.sin();
        for l in 0..settings::MAXDRAWDIST {
            let xr = 0.5 * (l as f32) * cphi;
            let yr = 0.5 * (l as f32) * sphi;
            let x = player.position.x + xr;
            let y = player.position.y + yr;
            let d = (xr*xr + yr*yr).sqrt();
            let i = x.floor() as i32;
            let j = y.floor() as i32;
            if i >= 0 && i < settings::MAPSIZE as i32 && j >= 0 && j < settings::MAPSIZE as i32 {
                if game_map.wall_array[i as usize][j as usize] < 255 {
                    game_map.wall_visible[i as usize][j as usize] = true;
                    game_map.wall_dist[i as usize][j as usize] = (d*4.0).floor() as usize;
                    break
                } else {
                    game_map.floor_visible[i as usize][j as usize] = true;
                    game_map.floor_dist[i as usize][j as usize] = (d*4.0).floor() as usize;
                }
            }
        }
    }
}

fn cmp_depth(a: &[usize; 4], b: &[usize; 4]) -> Ordering {
    if a[0] < b[0] {
        return Ordering::Greater;
    } else if a[0] > b[0] {
        return Ordering::Less;
    }
    return Ordering::Equal;
}