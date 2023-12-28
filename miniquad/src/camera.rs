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
    pub fn generate(
        game_map: &map::GameMap,
        player: &player::Player,
        settings: &settings::Settings,
    ) -> DepthBuffer {
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
                    let dist = (xi - xp).powi(2) + (yj + 2.0 / 4.0 - yp).powi(2);
                    if dist > dmax_current {
                        dmax_current = dist
                    }
                    faces.push(FaceData {
                        // face 1
                        top_right_x: i,
                        top_right_y: j,
                        bottom_right_x: i,
                        bottom_right_y: j,
                        bottom_left_x: i,
                        bottom_left_y: j + 1,
                        top_left_x: i,
                        top_left_y: j + 1,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                    let dist = (xi + 2.0 / 4.0 - xp).powi(2) + (yj + 4.0 / 4.0 - yp).powi(2);
                    if dist > dmax_current {
                        dmax_current = dist
                    }
                    faces.push(FaceData {
                        // face 2
                        top_right_x: i,
                        top_right_y: j + 1,
                        bottom_right_x: i,
                        bottom_right_y: j + 1,
                        bottom_left_x: i + 1,
                        bottom_left_y: j + 1,
                        top_left_x: i + 1,
                        top_left_y: j + 1,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                    let dist = (xi + 4.0 / 4.0 - xp).powi(2) + (yj + 2.0 / 4.0 - yp).powi(2);
                    if dist > dmax_current {
                        dmax_current = dist
                    }
                    faces.push(FaceData {
                        // face 3
                        top_right_x: i + 1,
                        top_right_y: j + 1,
                        bottom_right_x: i + 1,
                        bottom_right_y: j + 1,
                        bottom_left_x: i + 1,
                        bottom_left_y: j,
                        top_left_x: i + 1,
                        top_left_y: j,
                        is_wall: true,
                        dist,
                    });
                    len += 1;

                    let dist = (xi + 2.0 / 4.0 - xp).powi(2) + (yj - yp).powi(2);
                    if dist > dmax_current {
                        dmax_current = dist
                    }
                    faces.push(FaceData {
                        // face 4
                        top_right_x: i + 1,
                        top_right_y: j,
                        bottom_right_x: i + 1,
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
                    let dist = (xi + 2.0 / 4.0 - xp).powi(2) + (yj + 2.0 / 4.0 - yp).powi(2);
                    if dist > dmax_current {
                        dmax_current = dist
                    }
                    faces.push(FaceData {
                        top_right_x: i + 1,
                        top_right_y: j + 1,
                        bottom_right_x: i + 1,
                        bottom_right_y: j,
                        bottom_left_x: i,
                        bottom_left_y: j,
                        top_left_x: i,
                        top_left_y: j + 1,
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

pub fn find_visible_tiles(
    game_map: &mut map::GameMap,
    player: &player::Player,
    settings: &settings::Settings,
) {
    game_map.wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    let xp = player.position.x;
    let yp = player.position.y;
    let ip = xp.floor() as usize;
    let jp = yp.floor() as usize;
    game_map.floor_visible[ip][jp] = true;
    game_map.floor_dist[ip][jp] = 1.0;

    for k in 0..=settings.draw_rays_num {
        let phi = player.position.a
            + settings.tolerance
            + settings.fov_xy * (0.5 - (k as f32) / (settings.draw_rays_num as f32));
        let cphi = phi.cos();
        let sphi = phi.sin();

        // let mut phi_cond: u8 = 0;
        // if sphi > settings::INVSQRT2 {
        //     phi_cond = 1
        // }
        // if sphi < -settings::INVSQRT2 {
        //     phi_cond = 2
        // }
        // if cphi > settings::INVSQRT2 {
        //     phi_cond = 3
        // }
        // if cphi < -settings::INVSQRT2 {
        //     phi_cond = 4
        // }

        // if phi_cond > 0 {
        //     let tphi = phi.tan();
        //     let ctphi = 1.0 / tphi;

        //     let mut x = xp.floor();
        //     let mut y = yp.floor();
        //     let mut d;
        //     let mut i;
        //     let mut j;

        //     for _l in 0..settings::MAXDIST {
        //         if phi_cond == 1 {
        //             x += ctphi.abs() * cphi.signum();
        //             y += 1.0;
        //         }
        //         if phi_cond == 2 {
        //             x += ctphi.abs() * cphi.signum();
        //             y -= 1.0;
        //         }
        //         if phi_cond == 3 {
        //             x += 1.0;
        //             y += tphi.abs() * sphi.signum();
        //         }
        //         if phi_cond == 4 {
        //             x -= 1.0;
        //             y += tphi.abs() * sphi.signum();
        //         }

        //         d = (x - xp).powi(2) + (y - yp).powi(2);
        //         if game_map.dmax < d {
        //             game_map.dmax = d
        //         }
        //         if d > settings.draw_max_dist {break}

        //         i = x.floor() as usize;
        //         j = y.floor() as usize;

        //         if i as i32 >= 0
        //             && i < settings::MAPSIZE
        //             && j as i32 >= 0
        //             && j < settings::MAPSIZE
        //         {
        //             if game_map.wall_array[i][j] < 255 {
        //                 game_map.wall_visible[i][j] = true;
        //                 game_map.wall_dist[i][j] = d;
        //                 break;
        //             } else {
        //                 game_map.floor_visible[i][j] = true;
        //                 game_map.floor_dist[i][j] = d;
        //             }
        //         }

        //     }
        // }

        let mut x = xp;
        let mut y = yp;

        for _l in 0..settings::MAXDIST {
            x += 0.1 * cphi;
            y += 0.1 * sphi;
            let d = (x - xp).powi(2) + (y - yp).powi(2);
            if game_map.dmax < d {
                game_map.dmax = d
            }
            if d > settings.draw_max_dist {break}
            let i = x.floor() as usize;
            let j = y.floor() as usize;
            if i as i32 >= 0 && i < settings::MAPSIZE && j as i32 >= 0 && j < settings::MAPSIZE {
                if game_map.wall_array[i][j] < 255 {
                    game_map.wall_visible[i][j] = true;
                    game_map.wall_dist[i][j] = d;
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
