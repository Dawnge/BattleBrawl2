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
    pub dash: f32,
    pub dash_duration: f32, // seconds
    pub dash_cooldown: f32, // seconds
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
    dash: 2000.,
    dash_duration: 0.1, // seconds
    dash_cooldown: 0.3, //seconds

};

impl PlayerConfig {
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
    pub fn get_dash(&self) -> f32 {
        self.dash / get_fps() as f32
    }
    pub fn get_dash_duration(&self) -> u32 {
        (get_fps() as f32 * self.dash_duration) as u32
    }
    pub fn get_dash_cooldown(&self) -> u32 {
        (get_fps() as f32 * self.dash_cooldown) as u32
    }
}

pub struct ArenaConfig {
    pub floor_height: f32,
    pub gravity: f32,
    pub drag: f32,
}
pub const ARENA: ArenaConfig = ArenaConfig {
    floor_height: 0.8,
    gravity: 2000.,
    drag: 1000.,
};
impl ArenaConfig {
    pub fn get_floor_height(&self) -> f32 {
        self.floor_height * screen_height()
    }
    pub fn player_grounded(&self, player_pos: &(f32,f32)) -> bool {
        player_pos.1 >= self.get_floor_height() - PLAYER.height
    }
    pub fn get_gravity(&self) -> f32{
        self.gravity / (get_fps() as f32 * get_fps() as f32)
    }
    pub fn get_drag(&self) -> f32{
        self.drag / (get_fps() as f32 * get_fps() as f32)
    }
    pub fn player_1_start(&self) -> (f32,f32) {
        (0.2*screen_width(), 0.2*screen_height())
    }
    pub fn player_2_start(&self) -> (f32,f32) {
        (0.8*screen_width(), 0.2*screen_height())
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

