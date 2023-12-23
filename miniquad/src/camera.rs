use crate::player;
use crate::settings;
use crate::map;

pub fn find_visible_tiles(game_map: &mut map::GameMap, player: &player::Player) {
    game_map.wall_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    game_map.floor_visible = [[false; settings::MAPSIZE]; settings::MAPSIZE];
    let w = settings::WIDTH as usize;
    for k in 0..w {
        let phi = player.position.a + settings::FOVXY * (0.5 - (k as f32)/settings::WIDTHF);
        let cphi = phi.cos();
        let sphi = phi.sin();
        for l in 0..settings::MAXDRAWDIST {
            let x = player.position.x + (l as f32) * cphi;
            let y = player.position.y + (l as f32) * sphi;
            let i = x.floor() as i32;
            let j = y.floor() as i32;
            if i >= 0 && i < settings::MAPSIZE as i32 && j >= 0 && j < settings::MAPSIZE as i32 {
                if game_map.wall_array[i as usize][j as usize] < 255 {
                    game_map.wall_visible[i as usize][j as usize] = true;
                    break
                } else {
                    game_map.floor_visible[i as usize][j as usize] = true;
                }
            }
        }
    }
}