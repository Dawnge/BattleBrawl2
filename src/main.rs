use macroquad::prelude::*;



pub mod config;
use config::*;


fn handle_input(mut player_pos: &mut Box<(f32,f32)>) {
    if is_key_down(KeyCode::Right) && player_pos.0 < screen_width() - PLAYER.width {
        player_pos.0 += PLAYER.speed;
    }
    if is_key_down(KeyCode::Left) && player_pos.0 > 0. + PLAYER.width {
        // todo move left event
        player_pos.0 -= PLAYER.speed;
    }
    if is_key_down(KeyCode::Down) && player_pos.1 < ARENA.get_floor_height() - PLAYER.height + 75.{
        // todo: dash down event
        player_pos.1 += PLAYER.speed;
    }
    if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space)) && player_pos.1 > 0. + PLAYER.height{
        // todo: jump event
        player_pos.1 -= PLAYER.speed;
    }
}
#[macroquad::main(window_conf, "BasicShapes")]
async fn main() {
        // Gamestate
    let mut player_pos: Box<(f32, f32)> = PLAYER.get_start_pos();


    loop {
        // create background
        clear_background(DARKGRAY);
        draw_line(0., ARENA.get_floor_height(), screen_width(), ARENA.get_floor_height(), 5., BLACK);

        // handle input

        handle_input(&mut player_pos);

        // TODO
        // move_player(player_pos, player_movement);
        // do_player_physics()

        // render
        draw_circle(player_pos.0, player_pos.1, PLAYER.height, YELLOW);
        

        // debug stuff
        draw_text(format!("FPS {}", get_fps()).as_str(), 0.0, 10.0, 20.0, WHITE);

        next_frame().await
    }
}