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

pub struct Ray {
    pub angle: f32,
    pub distance: f32,
    pub x: f32,
    pub y: f32,
    pub i: usize,
    pub j: usize,
}

impl Ray {
    pub fn new (angle: f32) -> Ray {
        Ray {
            angle,
            distance: 0.0,
            x: 0.0,
            y: 0.0,
            i: 0,
            j: 0,
        }
    }
}

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
    pub faces_dist: Vec<FaceData>,
    pub len: usize,
}

impl DepthBuffer {
    pub fn generate(game_map: &map::GameMap, player: &player::Player, settings: &settings::Settings) -> DepthBuffer {
        let mut faces_dist: Vec<FaceData> = Vec::new();
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
                    faces_dist.push(FaceData { // face 1
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
                    faces_dist.push(FaceData { // face 2
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
                    faces_dist.push(FaceData { // face 3
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
                    faces_dist.push(FaceData { // face 4
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
                    faces_dist.push(FaceData {
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

        faces_dist.sort_by(cmp_dist);

        DepthBuffer {
            faces_dist,
            len,
        }
    }
}

pub fn ray_cast(game_map: &mut map::GameMap, player: &player::Player, settings: &settings::Settings) -> Vec<Ray> {
    let mut rays: Vec<Ray> = Vec::new();

    game_map.wall_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = vec![vec![false; settings::MAPSIZE]; settings::MAPSIZE];

    let screen_dist: f32 = settings.screen_width_f / 2.0 / settings.fov_xy.tan();
    let scale = settings.screen_width_f / (settings.draw_rays_num as f32);

    let xp = player.position.x;
    let yp = player.position.y;
    let ip = xp.floor() as usize;
    let jp = yp.floor() as usize;

    for i in 0..settings::MAPSIZE {
        for j in 0..settings::MAPSIZE {
            let d = (ip-i).pow(2)+(jp-j).pow(2);
            if d < settings.draw_min_dist {
                game_map.floor_visible[i][j] = true;
            }
        }
    }

    let mut a = player.position.a - settings.fov_xy + settings::TOLERANCE;

    for k in 0..settings.draw_rays_num {
        let cos_a = a.cos();
        let sin_a = a.sin();

        let mut ray = Ray::new(a);

        // horizontals
        let (mut y_hor, dy) = if sin_a > 0.0 {
            (jp as f32 + 1.0, 1.0)
        } else {
            (jp as f32 - settings::TOLERANCE, -1.0)
        };

        let mut dist_hor = (y_hor - yp) / sin_a;
        let mut x_hor = xp + dist_hor * cos_a;

        let d_dist = dy / sin_a;
        let dx = d_dist * cos_a;

        let mut check = false;

        let (mut i_hor, mut j_hor): (usize, usize) = (0, 0);
        let (mut i_vert, mut j_vert): (usize, usize) = (0, 0);

        while dist_hor < settings.draw_max_dist && !check {
            (i_hor, j_hor) = (x_hor.floor() as usize, y_hor.floor() as usize);

            if check_ij(i_hor, j_hor) {
                if game_map.wall_bot_array[i_hor][j_hor] < 255 {
                    check = true;
                } else {
                    game_map.floor_visible[i_hor][j_hor] = true;
                    x_hor += dx;
                    y_hor += dy;
                    dist_hor += d_dist
                }
            } else {
                if !check {
                    x_hor += dx;
                    y_hor += dy;
                    dist_hor += d_dist
                }
            }
        }

        // verticals
        let (mut x_vert, dx) = if cos_a > 0.0 {
            (ip as f32 + 1.0, 1.0)
        } else {
            (ip as f32 - settings::TOLERANCE, -1.0)
        };

        let mut dist_vert = (x_vert - xp) / cos_a;
        let mut y_vert = yp + dist_vert * sin_a;

        let d_dist = dx / cos_a;
        let dy = d_dist * sin_a;

        let mut check = false;

        while dist_vert < settings.draw_max_dist && !check {
            (i_vert, j_vert) = (x_vert.floor() as usize, y_vert.floor() as usize);

            if check_ij(i_vert, j_vert) {
                if game_map.wall_bot_array[i_vert][j_vert] < 255 {
                    check = true;
                } else {
                    game_map.floor_visible[i_vert][j_vert] = true;
                    x_vert += dx;
                    y_vert += dy;
                    dist_vert += d_dist
                }
            } else {
                if !check {
                    x_vert += dx;
                    y_vert += dy;
                    dist_vert += d_dist
                }
            }
        }

        let delta_a: f32 = player.position.a - a;

        // compare distances
        if dist_vert > dist_hor {
            ray.distance = dist_hor;
            if check_ij(i_hor, j_hor) && game_map.wall_bot_array[i_hor][j_hor] < 255 {
                game_map.wall_visible[i_hor][j_hor] = true;
            }
            ray.i = i_hor;
            ray.j = j_hor;
            ray.x = xp + (dist_hor+1.0)*cos_a;
            ray.y = yp + (dist_hor+1.0)*sin_a;
        } 
        else {
            ray.distance = dist_vert;
            if check_ij(i_vert, j_vert) && game_map.wall_bot_array[i_vert][j_vert] < 255 {
                game_map.wall_visible[i_vert][j_vert] = true;
            }
            ray.i = i_vert;
            ray.j = j_vert;
            ray.x = xp + (dist_vert+1.0)*cos_a;
            ray.y = yp + (dist_vert+1.0)*sin_a;
        };

        rays.push(ray);

        let q = scale / screen_dist;

        let da1 = q * delta_a.cos().powf(2.0);
        let da2 = da1 / (1.0 + q * (2.0 * delta_a).sin().abs());

        if k < settings.draw_rays_num / 2 {
            a += da1
        } else {
            a += da2
        }
        
    }


    rays
    
}

fn cmp_dist(a: &FaceData, b: &FaceData) -> Ordering {
    if a.dist < b.dist {
        return Ordering::Greater;
    } else if a.dist > b.dist {
        return Ordering::Less;
    }
    return Ordering::Equal;
}

fn check_ij(i: usize, j: usize) -> bool {
    i as i32 >= 0 && i < settings::MAPSIZE && j as i32 >= 0 && j < settings::MAPSIZE
}