use macroquad::{prelude::*};



pub mod config;
use config::*;


/// player_pos = Position of player (x,y)
/// player_vel = Velocity of player (x pixels per second, y pixesl per second)
/// 
/// if Arrow Right key is pressed: x velocity equals base speed (moving to the right)
///     if player out of boarder: Player position = next to boarder + velocity canceled
/// if Arrow Left key is pressed: x velocity equals minus base speed (moving to the left)
///     if player out of boarder: Player position = next to boarder + velocity canceled
/// none of these two pressed: x velocity = 0 (not moving)
/// 
/// NOTE: no differenciation between grounded and airborne yet
/// TODO: Implement something to track if certain air movement is possible
/// TODO: Implement air left/air right
/// TODO: Implement dive
/// TODO: Implement double Jump
/// TODO: Implement air dash
fn handle_input(player_pos: &mut Box<(f32,f32)>, player_vel: &mut Box<(f32,f32)>) {
    // GroundMovement
    //Move right: is alright
    if is_key_down(KeyCode::Right) && ARENA.player_grounded(player_pos){
        if player_pos.0 < screen_width() - PLAYER.width {
            player_vel.0 = PLAYER.get_player_speed();
        } else {
            player_vel.0 = 0.;
            player_pos.0 = screen_width() - PLAYER.width;
        }
    }
    // Move left: is alright
    if is_key_down(KeyCode::Left) && ARENA.player_grounded(player_pos){
        if player_pos.0 > PLAYER.width {
            player_vel.0 = -PLAYER.get_player_speed();
        } else {
            player_vel.0 = 0.;
            player_pos.0 = PLAYER.width;
        }
    } 
    if !is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
        player_vel.0 = 0.;
    }
    //Jump (TODO: clean up)
    if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space)) && ARENA.player_grounded(player_pos){
        player_vel.1 -= PLAYER.get_jump();
    }
    //TODO: implement DashOnGround

    // Air movement

    //TODO: Implement something to track if certain air movement is possible
    //TODO: Implement air left/air right
    //Move right: is alright
    if is_key_down(KeyCode::Right) && !ARENA.player_grounded(player_pos) && player_vel.0 <= PLAYER.get_player_speed(){
            player_vel.0 += PLAYER.get_aribone_acceleration();
    }
    // Move left: is alright
    if is_key_down(KeyCode::Left) && !ARENA.player_grounded(player_pos) && player_vel.0 >= -PLAYER.get_player_speed(){
            player_vel.0 -= PLAYER.get_aribone_acceleration();
    } 
    //TODO: Implement dive
    if is_key_pressed(KeyCode::Down) && !ARENA.player_grounded(player_pos){
        if player_pos.1 < ARENA.get_floor_height() - PLAYER.height {
            player_vel.1 = PLAYER.get_dive();
        } else {
            player_vel.1 = 0.;
            player_pos.1 = ARENA.get_floor_height() - PLAYER.height;
        }
    }
    //TODO: Implement double Jump
    if is_key_pressed(KeyCode::Up) && !ARENA.player_grounded(player_pos) {
        player_vel.1 = -PLAYER.get_second_jump();
    }
    //TODO: Implement air dash
    
    
}
/// checks if the player is out of screen to the left or the right and if so:
/// sets Player position to the position of the corresponding edge of the screen and sets Player velocity to 0
/// Function is called before apply_velocity to player is called.
fn check_boarders(player_pos: &mut Box<(f32,f32)>, player_vel: &mut Box<(f32,f32)>) {
    if player_pos.0 < PLAYER.width {
        player_vel.0 = 0.;
        player_pos.0 = PLAYER.width;
    }
    if player_pos.0 > screen_width() - PLAYER.width {
        player_vel.0 = 0.;
        player_pos.0 = screen_width() - PLAYER.width;
    }
}

fn handle_gravity(player_vel: &mut Box<(f32,f32)>) {
    player_vel.1 += ARENA.get_gravity();
}

fn apply_velocity(player_vel: &mut Box<(f32,f32)>, player_pos: &mut Box<(f32,f32)>) {
    player_pos.1 += player_vel.1;
    player_pos.0 += player_vel.0;
    if ARENA.player_grounded(player_pos) {
        player_pos.1 = ARENA.get_floor_height() - PLAYER.height;
        player_vel.1 = 0.;
    }
}
#[macroquad::main(window_conf, "BasicShapes")]
async fn main() {
        // Gamestate
    let mut player_pos: Box<(f32, f32)> = PLAYER.get_start_pos();
    let mut player_vel: Box<(f32, f32)> = Box::new((0.,0.));

    loop {
        // create background
        clear_background(DARKGRAY);
        draw_line(0., ARENA.get_floor_height() - 80., screen_width(), ARENA.get_floor_height() - 80., 5., BLACK);

        // handle/process input

        handle_input(&mut player_pos, &mut player_vel);
        check_boarders(&mut player_pos, &mut player_vel);
        apply_velocity(&mut player_vel, &mut player_pos);

        //update
        handle_gravity(&mut player_vel);
        
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