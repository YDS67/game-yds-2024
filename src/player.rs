use crate::map;
use crate::settings;
use crate::input;

pub struct PlayerPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
    pub b: f32,
    pub ax: f32,
    pub ay: f32,
    pub bxy: f32,
    pub bz: f32,
}

pub struct CollisionState {
    pub cxp: bool,
    pub cyp: bool,
    pub cxm: bool,
    pub cym: bool,
    pub cxl: bool,
    pub cyl: bool,
    pub cxr: bool,
    pub cyr: bool,
}

pub struct Direction {
    pub f: bool,  // forward
    pub b: bool,  // backward
    pub l: bool,  // left
    pub r: bool,  // right
    pub u: bool,  // up
    pub d: bool,  // down
    pub lt: bool, // left turn
    pub rt: bool, // right turn
    pub ut: bool, // up turn
    pub dt: bool, // down turn
}

impl Direction {
    pub fn _erase(&mut self) {
        self.f = false;
        self.b = false;
        self.l = false;
        self.r = false;
        self.u = false;
        self.d = false;
        self.lt = false;
        self.rt = false;
        self.ut = false;
        self.dt = false;
    }
}

pub struct MovementState {
    pub moving: bool,
    pub mouse: bool,
    pub dir: Direction,
}

impl MovementState {
    pub fn check(&mut self) {
        self.moving = self.dir.f
            || self.dir.b
            || self.dir.l
            || self.dir.r
            || self.dir.u
            || self.dir.d
            || self.dir.lt
            || self.dir.rt
            || self.dir.ut
            || self.dir.dt
    }
}

pub struct Player {
    pub position: PlayerPos,
    pub collision: CollisionState,
    pub movement: MovementState,
    pub radius: f32,
}

impl Player {
    pub fn new(settings: &settings::Settings) -> Player {
        let a = settings.player_a0;
        let b = settings.player_b0;
        Player {
            position: PlayerPos {
                x: settings.player_x0,
                y: settings.player_y0,
                z: settings.player_height,
                a,
                b,
                ax: a.cos(),
                ay: a.sin(),
                bxy: b.cos(),
                bz: b.sin(),
            },
            collision: CollisionState {
                cxp: false,
                cyp: false,
                cxm: false,
                cym: false,
                cxl: false,
                cyl: false,
                cxr: false,
                cyr: false,
            },
            movement: MovementState {
                moving: false,
                mouse: false,
                dir: Direction {
                    f: false,
                    b: false,
                    l: false,
                    r: false,
                    u: false,
                    d: false,
                    lt: false,
                    rt: false,
                    ut: false,
                    dt: false,
                },
            },
            radius: settings.player_radius,
        }
    }

    fn coll_check(&mut self, game_map: &map::GameMap) {
        let i = (self.position.x).floor() as usize;
        let j = (self.position.y).floor() as usize;
        let ip = (self.position.x + self.radius * self.position.ax).floor() as usize;
        let jp = (self.position.y + self.radius * self.position.ay).floor() as usize;
        let im = (self.position.x - self.radius * self.position.ax).floor() as usize;
        let jm = (self.position.y - self.radius * self.position.ay).floor() as usize;
        let il = (self.position.x - self.radius * self.position.ay).floor() as usize;
        let jl = (self.position.y + self.radius * self.position.ax).floor() as usize;
        let ir = (self.position.x + self.radius * self.position.ay).floor() as usize;
        let jr = (self.position.y - self.radius * self.position.ax).floor() as usize;

        self.collision.cxp = false;
        self.collision.cxm = false;
        self.collision.cyp = false;
        self.collision.cym = false;
        self.collision.cxl = false;
        self.collision.cxr = false;
        self.collision.cyl = false;
        self.collision.cyr = false;

        if game_map.wall_bot_array[ip][jp] < 255
            || game_map.wall_bot_array[im][jm] < 255
            || game_map.wall_bot_array[il][jl] < 255
            || game_map.wall_bot_array[ir][jr] < 255
        {
            self.collision.cxp = true;
            self.collision.cxm = true;
            self.collision.cyp = true;
            self.collision.cym = true;
            self.collision.cxl = true;
            self.collision.cxr = true;
            self.collision.cyl = true;
            self.collision.cyr = true;
        }

        if game_map.wall_bot_array[ip][j] == 255 {
            self.collision.cxp = false;
        }

        if game_map.wall_bot_array[i][jp] == 255 {
            self.collision.cyp = false;
        }

        if game_map.wall_bot_array[im][j] == 255 {
            self.collision.cxm = false;
        }

        if game_map.wall_bot_array[i][jm] == 255 {
            self.collision.cym = false;
        }

        if game_map.wall_bot_array[il][j] == 255 {
            self.collision.cxl = false;
        }

        if game_map.wall_bot_array[ir][j] == 255 {
            self.collision.cxr = false;
        }

        if game_map.wall_bot_array[i][jl] == 255 {
            self.collision.cyl = false;
        }

        if game_map.wall_bot_array[i][jr] == 255 {
            self.collision.cyr = false;
        }
    }

    pub fn read_key(&mut self, input: &input::InputState) {
        self.movement.dir.f = input.keys.w;
        self.movement.dir.b = input.keys.s;
        self.movement.dir.l = input.keys.a;
        self.movement.dir.r = input.keys.d;
        self.movement.dir.lt = input.keys.left;
        self.movement.dir.rt = input.keys.right;
        self.movement.dir.ut = input.keys.up;
        self.movement.dir.dt = input.keys.down;

        if input.keys.space && !self.movement.dir.u && !self.movement.dir.d {
            self.movement.dir.u = true
        }
    }

    pub fn walk(
        &mut self,
        game_map: &map::GameMap,
        settings: &settings::Settings,
        mouse_dx: f32,
        mouse_dy: f32,
        mouse_moving: bool,
    ) {
        self.coll_check(game_map);
        self.movement.check();

        let mut up_movement = mouse_moving;
        let mut down_movement = mouse_moving;

        if angle_round(self.position.b)+settings.fov_z > settings::PI/2.0 {
            up_movement = false;
        }

        if angle_round(self.position.b)-settings.fov_z < -settings::PI/2.0 {
            down_movement = false;
        }

        if mouse_dx.abs() < settings::TOLERANCE && mouse_dy.abs() < settings::TOLERANCE {
            self.movement.mouse = false;
        }

        if self.movement.dir.u {
            if self.position.z >= 1.5 {
                self.movement.dir.u = false;
                self.movement.dir.d = true;
            } else {
                self.position.z = self.position.z + 0.2 * settings.player_speed;
            }
        }

        if self.movement.dir.d {
            if self.position.z <= 0.5 {
                self.movement.dir.d = false;
            } else {
                self.position.z = self.position.z - 0.25 * settings.player_speed;
            }
        }

        if self.movement.dir.f {
            if !self.collision.cxp {
                self.position.x = self.position.x + settings.player_speed * self.position.ax;
            }
            if !self.collision.cyp {
                self.position.y = self.position.y + settings.player_speed * self.position.ay;
            }
        }

        if self.movement.dir.b {
            if !self.collision.cxm {
                self.position.x = self.position.x - settings.player_speed * self.position.ax;
            }
            if !self.collision.cym {
                self.position.y = self.position.y - settings.player_speed * self.position.ay;
            }
        }

        if self.movement.dir.l {
            if !self.collision.cxl {
                self.position.x = self.position.x - settings.player_speed * self.position.ay;
            }
            if !self.collision.cyl {
                self.position.y = self.position.y + settings.player_speed * self.position.ax;
            }
        }

        if self.movement.dir.r {
            if !self.collision.cxr {
                self.position.x = self.position.x + settings.player_speed * self.position.ay;
            }
            if !self.collision.cyr {
                self.position.y = self.position.y - settings.player_speed * self.position.ax;
            }
        }

        if self.movement.dir.lt {
            self.position.a = angle_round(self.position.a + 0.2 * settings.player_speed);
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if self.movement.dir.rt {
            self.position.a = angle_round(self.position.a - 0.2 * settings.player_speed);
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
        }

        if mouse_moving {
            self.position.a =
                self.position.a - settings::PI * settings.mouse_sensitivity * mouse_dx;
            self.position.ax = self.position.a.cos();
            self.position.ay = self.position.a.sin();
            self.movement.mouse = true;
        }

        if self.movement.dir.dt && self.position.b + settings.fov_z < settings::PI / 2.0 {
            self.position.b = angle_round(self.position.b + 0.2 * settings.player_speed);
            self.position.bxy = self.position.b.cos();
            self.position.bz = self.position.b.sin();
        }

        if self.movement.dir.ut && self.position.b - settings.fov_z > -settings::PI / 2.0 {
            self.position.b = angle_round(self.position.b - 0.2 * settings.player_speed);
            self.position.bxy = self.position.b.cos();
            self.position.bz = self.position.b.sin();
        }

        if (mouse_dy > 0.0 && down_movement) || (mouse_dy < 0.0 && up_movement)
        {
            self.position.b = angle_round(self.position.b - settings::PI * settings.mouse_sensitivity * mouse_dy);
            self.position.bxy = self.position.b.cos();
            self.position.bz = self.position.b.sin();
            self.movement.mouse = true;
        }
    }
}

pub fn angle_round(angle: f32) -> f32 {
    let mut in_degrees = angle * 180.0 / settings::PI;
    while in_degrees < -90.0 {
        in_degrees = 360.0 + in_degrees
    }
    while in_degrees > 89.9 {
        in_degrees = -360.0 + in_degrees
    }
    in_degrees * settings::PI / 180.0
}
