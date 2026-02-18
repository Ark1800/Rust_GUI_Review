use macroquad::{color, prelude::*};
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::scale::use_virtual_resolution;

pub async fn run() -> String {
    // Define virtual resolution constants
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    
    let lbl_welcome = Label::new("Welcome to the game!", 150.0, 100.0, 30);
    let lbl_directions = Label::new("Click enter to start the game WASD to move", 150.0, 200.0, 30);
    let btn_enter = TextButton::new(
        150.0,
        400.0,
        200.0,
        60.0,
        "Enter Game",
        BLUE,
        GREEN,
        30
    );
    let btn_exit = TextButton::new(
        400.0,
        400.0,
        200.0,
        60.0,
        "Exit",
        BLUE,
        GREEN,
        30
    
    );
    loop {
        // Apply virtual resolution every frame
        use_virtual_resolution(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        clear_background(WHITE);
        draw_grid(50.0, color::LIGHTGRAY);
        if btn_enter.click() {
            return "screen2".to_string();
        }
        if btn_exit.click() {
            return "exit".to_string();
        }
        lbl_welcome.draw();
        lbl_directions.draw();
        next_frame().await;
    }
}