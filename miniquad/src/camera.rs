use std::cmp::Ordering;

use crate::map;
use crate::player;
use crate::settings;

// Wall faces scheme
//      2-----------3
//      |     2     |
//      |           |
//      |1         3|
//      |     4     |
//      1-----------4
// 1 = (i,j)
// 2 = (i,j+1)
// 3 = (i+1,j+1)
// 4 = (i+1.j)

fn find_corner(i: usize, j: usize, xp: f32, yp: f32) -> u8 {
    let mut corner = 1;
    if xp <= i as f32+0.5 && yp <= j as f32+0.5 {
        corner = 1
    }
    if xp <= i as f32+0.5 && yp > j as f32+0.5 {
        corner = 2
    }
    if xp > i as f32+0.5 && yp <= j as f32+0.5 {
        corner = 4
    }
    if xp > i as f32+0.5 && yp > j as f32+0.5 {
        corner = 3
    }
    corner
}

#[derive(Debug, Clone)]
pub struct TileData {
    pub i: usize,
    pub j: usize,
    pub dist: usize,
    pub wall_corner: u8,
}

pub struct DepthBuffer {
    pub visible_tiles: Vec<TileData>,
    pub dmax: usize,
    pub len: usize,
}

impl DepthBuffer {
    pub fn generate(game_map: &map::GameMap, player: &player::Player) -> DepthBuffer {
        let mut visible_tiles: Vec<TileData> = Vec::new();
        let mut dmax = 0;
        let mut len = 0;
        let xp = player.position.x;
        let yp = player.position.y;

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                if game_map.wall_visible[i][j] {
                    let d = game_map.wall_dist[i][j];
                    if d > dmax {
                        dmax = d
                    }
                    visible_tiles.push(TileData {
                        i,
                        j,
                        dist: d,
                        wall_corner: find_corner(i, j, xp, yp),
                    });
                    len += 1;
                }
                if game_map.floor_visible[i][j] {
                    let d = game_map.floor_dist[i][j];
                    if d > dmax {
                        dmax = d
                    }
                    visible_tiles.push(TileData {
                        i,
                        j,
                        dist: d,
                        wall_corner: 0,
                    });
                    len += 1;
                }
            }
        }

        visible_tiles.sort_by(cmp_depth);

        DepthBuffer {
            visible_tiles,
            dmax,
            len,
        }
    }
}

pub fn find_visible_tiles(game_map: &mut map::GameMap, player: &player::Player) {
    game_map.wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    for k in 0..=settings::NUMRAYS {
        let phi =
            player.position.a + settings::FOVXY * (0.5 - (k as f32) / (settings::NUMRAYS as f32));
        let cphi = phi.cos();
        let sphi = phi.sin();
        let mut xr = 0.0;
        let mut yr = 0.0;
        for _l in 0..settings::MAXDRAWDIST {
            xr += 0.1 * cphi;
            yr += 0.1 * sphi;
            let x = player.position.x + xr;
            let y = player.position.y + yr;
            let d = (xr * xr + yr * yr).sqrt();
            let i = x.floor() as usize;
            let j = y.floor() as usize;
            if i as i32 >= 0 && i < settings::MAPSIZE && j as i32 >= 0 && j < settings::MAPSIZE {
                if game_map.wall_array[i][j] < 255 {
                    game_map.wall_visible[i][j] = true;
                    game_map.wall_dist[i][j] = (d * 4.0).floor() as usize;
                    break;
                } else {
                    game_map.floor_visible[i][j] = true;
                    game_map.floor_dist[i][j] = (d * 4.0).floor() as usize;
                }
            }
        }
    }
}

fn cmp_depth(a: &TileData, b: &TileData) -> Ordering {
    if a.dist < b.dist {
        return Ordering::Greater;
    } else if a.dist > b.dist {
        return Ordering::Less;
    }
    return Ordering::Equal;
}
