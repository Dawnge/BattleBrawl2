use macroquad::prelude::*;


pub enum Input {
    Right,
    Left,
    Jump,
    Dive,
    DashRight,
    DashLeft,
}

pub fn get_input_player1() -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];
    // continuous
    if is_key_down(KeyCode::D) {
        inputs.push(Input::Right);
    }
    if is_key_down(KeyCode::A) {
        inputs.push(Input::Left);
    }
    // once per push
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::W) {
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

pub fn get_input_player2() -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];
    // continuous
    if is_key_down(KeyCode::Kp6) {
        inputs.push(Input::Right);
    }
    if is_key_down(KeyCode::Kp4) {
        inputs.push(Input::Left);
    }
    // once per push
    if is_key_pressed(KeyCode::Kp8) {
        inputs.push(Input::Jump);
    }
    if is_key_pressed(KeyCode::Kp5) {
        inputs.push(Input::Dive);
    }
    if is_key_pressed(KeyCode::Kp1)  { // its american layout defuq
        inputs.push(Input::DashLeft);
    }
    if is_key_pressed(KeyCode::Kp3) {
        inputs.push(Input::DashRight);
    }

    inputs
}