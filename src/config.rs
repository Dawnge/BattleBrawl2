use macroquad::prelude::*;


pub struct PlayerConfig {
    pub base_speed: f32,
    pub width: f32,
    pub height: f32,
    pub start_pos: (f32,f32),
    pub jump: f32,
    pub second_jump: f32,
    pub dive: f32,
    pub airbone_acceleleration: f32,
}
pub const PLAYER: PlayerConfig = PlayerConfig {
    base_speed: 500.,
    width: 15.,
    height: 15.,
    start_pos: (0.5f32, 0.5f32),
    jump: 1000.,
    second_jump: 700.,
    dive: 700.,
    airbone_acceleleration: 50.,
};

impl PlayerConfig {
    pub fn get_start_pos(&self) -> Box<(f32, f32)> {
        Box::new((self.start_pos.0 * screen_width(), self.start_pos.1 * screen_height()))
    }
    pub fn get_player_speed(&self) -> f32 {
        self.base_speed / get_fps() as f32
    }
    pub fn get_jump(&self) -> f32 {
        self.jump / get_fps() as f32
    }
    pub fn get_second_jump(&self) -> f32 {
        self.second_jump / get_fps() as f32
    }
    pub fn get_dive(&self) -> f32 {
        self.dive / get_fps() as f32
    }
    pub fn get_aribone_acceleration(&self) -> f32 {
        self.airbone_acceleleration / get_fps() as f32
    }
}

pub struct ArenaConfig {
    pub floor_height: f32,
    pub gravity: f32,
}
pub const ARENA: ArenaConfig = ArenaConfig {
    floor_height: 0.8,
    gravity: 30.,
};
impl ArenaConfig {
    pub fn get_floor_height(&self) -> f32 {
        self.floor_height * screen_height()
    }
    pub fn player_grounded(&self, player_pos: &(f32,f32)) -> bool {
        player_pos.1 >= self.get_floor_height()-PLAYER.height
    }
    pub fn get_gravity(&self) -> f32{
        self.gravity / get_fps()as f32
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "BattleBrawl2 Game".to_owned(),
        fullscreen: false,
        window_height: 720,
        window_width: 1280,
        window_resizable: false,
        ..Default::default()
    }
}

