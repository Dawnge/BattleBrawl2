use macroquad::prelude::*;

use crate::config::*;
use crate::input::*;

pub enum Orientation {
    Right,
    Left
}

pub struct Player {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub orientation: Orientation,
    pub grounded: bool,
    pub can_jump: bool, // player can still jump
    pub can_dive: bool,
    pub dash_duration: u32,
}

impl Player {
    pub fn new(position: (f32, f32)) -> Self {
        Self {
            position, 
            velocity: (0., 0.),
            orientation: Orientation::Right, 
            grounded: false, 
            can_jump: true, 
            can_dive: true,
            dash_duration: 0,
        }
    }

    pub fn update(&mut self, input: &Vec<Input>) {
        // apply basic movement directly and update velocity.
        for i in input {
            match *i {
                Input::Right => {
                    self.position.0 += PLAYER.get_player_speed();
                    self.orientation = Orientation::Right;
                },
                Input::Left => {
                    self.position.0 -= PLAYER.get_player_speed();
                    self.orientation = Orientation::Left;
                },
                Input::Jump => {
                    if self.grounded {
                        self.velocity.1 = -PLAYER.get_jump();
                        self.grounded = false; // we jump from the ground -> we are no longer grounded.
                        self.can_dive = true; // we are no longer grounded -> we can dive.
                    } else if self.can_jump {
                        self.velocity.1 = -PLAYER.get_second_jump();
                        self.can_jump = false; // can only jump once when airbone.
                    }
                },
                Input::Dive => {
                    if self.can_dive {
                        self.velocity.1 = PLAYER.get_dive();
                        self.can_dive = false; // can only dive once, resets when grounded.
                    }
                },
                Input::DashRight => {
                    if self.dash_duration == 0 { // we are not dashing -> we can dash
                        self.dash_duration = PLAYER.get_dash_duration();
                        self.velocity.0 = PLAYER.get_dash();
                    }
                },
                Input::DashLeft => {
                    if self.dash_duration == 0 { // we are not dashing -> we can dash
                        self.dash_duration = PLAYER.get_dash_duration();
                        self.velocity.0 = -PLAYER.get_dash();
                    }
                    
                },

            }
        }

        // Apply Physics: Drag, Gravity
        // Horizontal axis, DRAG
        if self.velocity.0 > 0. {
            self.velocity.0 = (self.velocity.0 - ARENA.get_drag()).max(0.);
        } else if self.velocity.0 < 0. {
            self.velocity.0 = (self.velocity.0 + ARENA.get_drag()).min(0.)
        }
        // Vertical axis, GRAVITY
        self.velocity.1 += ARENA.get_gravity();


        // Apply velocity to position + boundary checks.
        let mut x = self.position.0 + self.velocity.0;
        let mut y = self.position.1 + self.velocity.1;
        if x < PLAYER.width {
            x = PLAYER.width;
        } else if x > screen_width() - PLAYER.width {
            x = screen_width() - PLAYER.width;
        }
        if y > ARENA.get_floor_height() - PLAYER.height {
            y = ARENA.get_floor_height() - PLAYER.height;
        } else if y < PLAYER.height {
            y =  PLAYER.height;
        }
        self.position.0 = x;
        self.position.1 = y;

        // Update Player state, check if we are grounded after applying the velocity and update state.
        if ARENA.player_grounded(&self.position) {
            self.grounded = true;
            self.can_jump = true;
            self.can_dive = false;
            self.velocity.1 = 0.; // reset all vertical velocity
        }
        // decrement dash_duration if dashing and reset vertical velocity if no longer dashing
        if self.dash_duration > 0 {
            self.dash_duration -= 1;
            if self.dash_duration == 0 {
                self.velocity.0 = 0.;
            }
        }
    }

    pub fn render(&self, color: Color) {
        draw_circle(self.position.0, self.position.1, PLAYER.height, color);
        match self.orientation {
            Orientation::Right => {
                draw_line(self.position.0, self.position.1, self.position.0 + PLAYER.width, self.position.1, 3., RED)
            },
            Orientation::Left => {
                draw_line(self.position.0, self.position.1, self.position.0 - PLAYER.width, self.position.1, 3., RED)

            }
        }
    }
}
