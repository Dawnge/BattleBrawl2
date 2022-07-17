use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "BattleBrawl2 Game".to_owned(),
        fullscreen: false,
        window_height: 720,
        window_width: 1280,
        window_resizable: false,
        ..Default::default()
    }
}

pub fn move_player(mut player_pos: (f32,f32), player_movement: Vec<PlayerMoveEvent>) -> (f32, f32) {
    // handle physics
    if player_movement.contains(&PlayerMoveEvent::Right) {
        player_pos.0 += 1.;
    }

    player_pos

}

#[derive(PartialEq)]
pub enum PlayerMoveEvent {
    Right,
    Left,
    Down,
    Up,
}

#[macroquad::main(window_conf, "BasicShapes")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let floor: f32 = 0.8*screen_height();
    let player_height = 15.0;
    let player_width = player_height;
    let movement_speed = 10.0;

    // Gamestate
    let mut player_movement: Vec<PlayerMoveEvent> =  vec![];
    let mut player_pos: (f32, f32) = (screen_width() / 2.0, screen_height() / 2.0);
    

    loop {
        // create background
        clear_background(DARKGRAY);
        draw_line(0., floor, screen_width(), floor, 5., BLACK);

        // handle input
        if is_key_down(KeyCode::Right) && x < screen_width() - player_width {
            // todo: move right event
            // state.player_movement.insert(PlayerMoveEvent::Right);
            x += movement_speed;
        }
        if is_key_down(KeyCode::Left) && x > 0. + player_width {
            // todo move left event
            x -= movement_speed;
        }
        if is_key_down(KeyCode::Down) && y < floor - player_height + 75. /* (thickness - 1)/2 = 2 */ {
            // todo: dash down event
            y += movement_speed;
        }
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space)) && y > 0. + player_height{
            // todo: jump event
            y -= movement_speed;
        }

        // TODO
        // move_player(player_pos, player_movement);
        // do_player_physics()

        // render
        draw_circle(x, y, player_height, YELLOW);
        

        // debug stuff
        draw_text(format!("FPS {}", get_fps()).as_str(), 0.0, 10.0, 20.0, WHITE);

        next_frame().await
    }
}