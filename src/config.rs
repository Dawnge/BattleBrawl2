use macroquad::prelude::*;

/// Ich will das in der Documentation sehen!
pub struct PlayerConfig {
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub start_pos: (f32,f32),
}
pub const PLAYER: PlayerConfig = PlayerConfig {
    speed: 10.0f32,
    width: 15.,
    height: 15.,
    start_pos: (0.5f32, 0.5f32),
};
impl PlayerConfig {
    pub fn get_start_pos(&self) -> Box<(f32, f32)> {
        Box::new((self.start_pos.0 * screen_width(), self.start_pos.1 * screen_height()))
    }
}

pub struct ArenaConfig {
    pub floor_height: f32,
}
pub const ARENA: ArenaConfig = ArenaConfig {
    floor_height: 0.8,
};
impl ArenaConfig {
    pub fn get_floor_height(&self) -> f32 {
        self.floor_height * screen_height()
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