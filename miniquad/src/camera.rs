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

#[derive(Debug, Clone)]
pub struct FaceData {
    pub top_right_x: usize,
    pub top_right_y: usize,
    pub bottom_right_x: usize,
    pub bottom_right_y: usize,
    pub top_left_x: usize,
    pub top_left_y: usize,
    pub bottom_left_x: usize,
    pub bottom_left_y: usize,
    pub is_wall: bool,
    pub dist: f32,
}

pub struct DepthBuffer {
    pub faces: Vec<FaceData>,
    pub len: usize,
    pub dmax_current: f32,
    pub dmax: f32,
}

impl DepthBuffer {
    pub fn generate(game_map: &map::GameMap, player: &player::Player, settings: &settings::Settings) -> DepthBuffer {
        let mut faces: Vec<FaceData> = Vec::new();
        let mut len = 0;
        let xp = player.position.x;
        let yp = player.position.y;
        let mut dmax_current = 1.0;

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let xi = i as f32;
                let yj = j as f32;
                if game_map.wall_visible[i][j] {
                
                    let dist = (xi-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
                    if dist > dmax_current {dmax_current = dist}
                    faces.push(FaceData { // face 1
                        top_right_x: i,
                        top_right_y: j,
                        bottom_right_x: i,
                        bottom_right_y: j,
                        bottom_left_x: i,
                        bottom_left_y: j+1,
                        top_left_x: i,
                        top_left_y: j+1,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                    let dist = (xi+2.0/4.0-xp).powi(2)+(yj+4.0/4.0-yp).powi(2);
                    if dist > dmax_current {dmax_current = dist}
                    faces.push(FaceData { // face 2
                        top_right_x: i,
                        top_right_y: j+1,
                        bottom_right_x: i,
                        bottom_right_y: j+1,
                        bottom_left_x: i+1,
                        bottom_left_y: j+1,
                        top_left_x: i+1,
                        top_left_y: j+1,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                    let dist = (xi+4.0/4.0-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
                    if dist > dmax_current {dmax_current = dist}
                    faces.push(FaceData { // face 3
                        top_right_x: i+1,
                        top_right_y: j+1,
                        bottom_right_x: i+1,
                        bottom_right_y: j+1,
                        bottom_left_x: i+1,
                        bottom_left_y: j,
                        top_left_x: i+1,
                        top_left_y: j,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                    let dist = (xi+2.0/4.0-xp).powi(2)+(yj-yp).powi(2);
                    if dist > dmax_current {dmax_current = dist}
                    faces.push(FaceData { // face 4
                        top_right_x: i+1,
                        top_right_y: j,
                        bottom_right_x: i+1,
                        bottom_right_y: j,
                        bottom_left_x: i,
                        bottom_left_y: j,
                        top_left_x: i,
                        top_left_y: j,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                } else if game_map.floor_visible[i][j] {
                    let dist = (xi+2.0/4.0-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
                    if dist > dmax_current {dmax_current = dist}
                    faces.push(FaceData {
                        top_right_x: i+1,
                        top_right_y: j+1,
                        bottom_right_x: i+1,
                        bottom_right_y: j,
                        bottom_left_x: i,
                        bottom_left_y: j,
                        top_left_x: i,
                        top_left_y: j+1,
                        is_wall: false,
                        dist,
                    });
                    len += 1;
                }
            }
        }

        faces.sort_by(cmp_depth);

        DepthBuffer {
            faces,
            len,
            dmax_current,
            dmax: settings.light_dist,
        }
    }
}

pub fn find_visible_tiles(game_map: &mut map::GameMap, player: &player::Player, settings: &settings::Settings) {
    game_map.wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    for k in 0..=settings.draw_rays_num {
        let phi =
            player.position.a + settings.fov_xy * (0.5 - (k as f32) / (settings.draw_rays_num as f32));
        let cphi = phi.cos();
        let sphi = phi.sin();
        let mut xr = 0.0;
        let mut yr = 0.0;
        for _l in 0..settings.draw_max_dist {
            xr += 0.1 * cphi;
            yr += 0.1 * sphi;
            let x = player.position.x + xr;
            let y = player.position.y + yr;
            let d = xr * xr + yr * yr;
            let i = x.floor() as usize;
            let j = y.floor() as usize;
            if i as i32 >= 0 && i < settings::MAPSIZE && j as i32 >= 0 && j < settings::MAPSIZE {
                if game_map.wall_array[i][j] < 255 {
                    game_map.wall_visible[i][j] = true;
                    game_map.wall_dist[i][j] = d;
                    if game_map.dmax < d {game_map.dmax = d}
                    break;
                } else {
                    game_map.floor_visible[i][j] = true;
                    game_map.floor_dist[i][j] = d;
                }
            }
        }
    }
}

fn cmp_depth(a: &FaceData, b: &FaceData) -> Ordering {
    if a.dist < b.dist {
        return Ordering::Greater;
    } else if a.dist > b.dist {
        return Ordering::Less;
    }
    return Ordering::Equal;
}
