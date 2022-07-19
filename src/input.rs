use macroquad::prelude::*;


pub enum Input {
    Right,
    Left,
    Jump,
    Dive,
}

pub fn get_input() -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];
    // continuous
    if is_key_down(KeyCode::Right) {
        inputs.push(Input::Right);
    }
    if is_key_down(KeyCode::Left) {
        inputs.push(Input::Left);
    }
    // once per push
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) {
        inputs.push(Input::Jump);
    }
    if is_key_pressed(KeyCode::Down) {
        inputs.push(Input::Dive);
    }

    inputs
}