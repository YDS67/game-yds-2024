use std::cmp::Ordering;

use crate::map;
use crate::player;
use crate::settings;

// Wall faces scheme
//      -------------
//      |     2     |
//      |           |
//      |1         3|
//      |     4     |
//      -------------
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
    pub texture_bot: u8,
    pub texture_top: u8,
    pub dist: f32,
}

pub struct DepthBuffer {
    pub faces: Vec<FaceData>,
    pub len: usize,
    pub dmax: f32,
}

impl DepthBuffer {
    pub fn generate(game_map: &map::GameMap, player: &player::Player, settings: &settings::Settings) -> DepthBuffer {
        let mut faces: Vec<FaceData> = Vec::new();
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
                let xi = i as f32;
                let yj = j as f32;
                if game_map.wall_visible[i][j] {
                
                    let dist = (xi-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
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
                        texture_bot: game_map.wall_bot_array[i][j],
                        texture_top: game_map.wall_top_array[i][j],
                        dist,
                    });
                    len += 1;

                    let dist = (xi+2.0/4.0-xp).powi(2)+(yj+4.0/4.0-yp).powi(2);
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
                        texture_bot: game_map.wall_bot_array[i][j],
                        texture_top: game_map.wall_top_array[i][j],
                        dist,
                    });
                    len += 1;

                    let dist = (xi+4.0/4.0-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
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
                        texture_bot: game_map.wall_bot_array[i][j],
                        texture_top: game_map.wall_top_array[i][j],
                        dist,
                    });
                    len += 1;

                    let dist = (xi+2.0/4.0-xp).powi(2)+(yj-yp).powi(2);
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
                        texture_bot: game_map.wall_bot_array[i][j],
                        texture_top: game_map.wall_top_array[i][j],
                        dist,
                    });
                    len += 1;

                } 
                if game_map.floor_visible[i][j] {
                    let dist = (xi+2.0/4.0-xp).powi(2)+(yj+2.0/4.0-yp).powi(2);
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
                        texture_bot: game_map.floor_array[i][j],
                        texture_top: game_map.ceil_array[i][j],
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

pub fn find_visible_tiles(game_map: &mut map::GameMap, player: &player::Player, settings: &settings::Settings) {
    game_map.wall_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];
    let ip = player.position.x.floor() as usize;
    let jp = player.position.y.floor() as usize;
    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let d = (ip-i).pow(2)+(jp-j).pow(2);
            if d < settings.draw_min_dist {
                game_map.floor_visible[i][j] = true;
                game_map.floor_dist[i][j] = d as f32;
            }
        }
    }
    for k in 0..=settings.draw_rays_num {
        let phi =
            player.position.a + settings.fov_xy * (1.0 - 2.0*(k as f32) / (settings.draw_rays_num as f32));
        let cphi = phi.cos();
        let sphi = phi.sin();
        let mut xr = 0.0;
        let mut yr = 0.0;
        let mut break_soon = 0;
        for _l in 0..settings.draw_max_dist*settings.draw_steps {
            if break_soon >= 3 {break}
            xr += cphi / settings.draw_steps as f32;
            yr += sphi / settings.draw_steps as f32;
            let x = player.position.x + xr;
            let y = player.position.y + yr;
            let d = xr * xr + yr * yr;
            let i = x.floor() as usize;
            let j = y.floor() as usize;
            if i as i32 >= 0 && i < settings::MAPSIZE && j as i32 >= 0 && j < settings::MAPSIZE {
                if game_map.wall_bot_array[i][j] < 255 {
                    game_map.wall_visible[i][j] = true;
                    game_map.wall_dist[i][j] = d;
                    for ii in (i-2)..(i+3) {
                        for jj in (j-2)..(j+3) {
                            if ii as i32 > 0 && ii < settings::MAPSIZE-1 && jj as i32 > 0 && jj < settings::MAPSIZE-1 {
                            game_map.floor_visible[ii][jj] = true;
                            game_map.floor_dist[ii][jj] = d;
                            }
                        }
                    }
                    break_soon += 1;
                } else {
                    game_map.floor_visible[i][j] = true;
                    game_map.floor_dist[i][j] = d;
                    if break_soon > 0 {break_soon += 1}
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