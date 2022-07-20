use macroquad::prelude::*;


pub enum Input {
    Right,
    Left,
    Jump,
    Dive,
    DashRight,
    DashLeft,
}

pub fn get_input() -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];
    // continuous
    if is_key_down(KeyCode::D) {
        inputs.push(Input::Right);
    }
    if is_key_down(KeyCode::A) {
        inputs.push(Input::Left);
    }
    // once per push
    if is_key_pressed(KeyCode::Space) {
        inputs.push(Input::Jump);
    }
    if is_key_pressed(KeyCode::S) {
        inputs.push(Input::Dive);
    }
    if is_key_pressed(KeyCode::Z)  { // its american layout defuq
        inputs.push(Input::DashLeft);
    }
    if is_key_pressed(KeyCode::C) {
        inputs.push(Input::DashRight);
    }

    inputs
}