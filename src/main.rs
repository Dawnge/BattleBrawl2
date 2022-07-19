
use macroquad::prelude::*;

pub mod config;
use config::*;

pub mod input;
use input::*;

pub mod player;
use player::*;


#[macroquad::main(window_conf, "BasicShapes")]
async fn main() {
    // Gamestate

    let mut player: Player = Player::new();


    loop {
        // input
        let inputs: Vec<Input> = get_input();

        // logic
        player.update(&inputs);
        

        // render
        // create background
        clear_background(DARKGRAY);
        draw_line(0., ARENA.get_floor_height() - 80., screen_width(), ARENA.get_floor_height() - 80., 5., BLACK);
        // draw player
        draw_circle(player.position.0, player.position.1, PLAYER.height, YELLOW);
        match player.orientation {
            Orientation::Right => {
                draw_line(player.position.0, player.position.1, player.position.0 + PLAYER.width, player.position.1, 3., RED)
            },
            Orientation::Left => {
                draw_line(player.position.0, player.position.1, player.position.0 - PLAYER.width, player.position.1, 3., RED)

            }
        }

        // debug stuff
        draw_text(format!("FPS {}", get_fps()).as_str(), 0.0, 10.0, 20.0, WHITE);
        draw_text(format!("Player.p=(x={:.2}, y={:.2})", player.position.0, player.position.1).as_str(), 0.0, 30.0, 20.0, WHITE);
        draw_text(format!("Player.v=(x={:.2}, y={:.2})", player.velocity.0 * get_fps() as f32, player.velocity.1 * get_fps() as f32).as_str(), 0.0, 50.0, 20.0, WHITE);



        next_frame().await
    }
}