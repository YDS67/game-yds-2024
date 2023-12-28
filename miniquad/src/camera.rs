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

        for i in 0..settings::MAPSIZE {
            for j in 0..settings::MAPSIZE {
                let xi = i as f32;
                let yj = j as f32;
                if game_map.wall_visible[i][j] {
                    let dist = (xi - xp).powi(2) + (yj + 2.0 / 4.0 - yp).powi(2);
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
    let ip = f32::trunc(xp) as usize;
    let jp = f32::trunc(yp) as usize;
    game_map.floor_visible[ip][jp] = true;
    let phi1 = player::angle_round(player.position.a - 0.5*settings.fov_xy - settings.tolerance);
    let phi2 = player::angle_round(player.position.a + 0.5*settings.fov_xy + settings.tolerance);
    let cond = phi1 < phi2;

    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let di = i as f32 + 0.5 - xp;
            let dj = j as f32 + 0.5 - yp;
            let d = (di.powi(2)+dj.powi(2)).sqrt();
            let phi = player::angle_round(dj.atan2(di));

            if (((cond && (phi1 < phi && phi < phi2)) || (!cond && (phi1 < phi || phi < phi2))) && d < settings.draw_max_dist) || d < settings.draw_min_dist {
                game_map.floor_visible[i][j] = true;
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
