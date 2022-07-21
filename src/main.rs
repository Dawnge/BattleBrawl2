
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

    let mut player1_move: PlayerMove = PlayerMove::new(ARENA.player_1_start());
    let mut player2_move: PlayerMove = PlayerMove::new(ARENA.player_2_start());


    loop {
        // input
        let inputs_player_1: Vec<Input> = get_input_player1();
        let inputs_player_2: Vec<Input> = get_input_player2();

        // logic
        player1_move.update(&inputs_player_1);
        player2_move.update(&inputs_player_2);


        // render
        // create background
        clear_background(DARKGRAY);
        draw_line(0., ARENA.get_floor_height() - 80., screen_width(), ARENA.get_floor_height() - 80., 5., BLACK);
        
        // render player
        player1_move.render(YELLOW);
        player2_move.render(GREEN);

        // debug stuff
        draw_text(format!("FPS {}", get_fps()).as_str(), 0.0, 10.0, 20.0, WHITE);
        draw_text(format!("Player.p=(x={:.2}, y={:.2})", player1_move.position.0, player1_move.position.1).as_str(), 0.0, 30.0, 20.0, WHITE);
        draw_text(format!("Player.v=(x={:.2}, y={:.2})", player1_move.velocity.0 * get_fps() as f32, player1_move.velocity.1 * get_fps() as f32).as_str(), 0.0, 50.0, 20.0, WHITE);



        next_frame().await
    }
}