use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::messagebox::{MessageBox, MessageBoxResult};
use crate::modules::collision::check_collision;
use crate::modules::scale::use_virtual_resolution;

pub async fn run() -> String {
    // Define virtual resolution constants
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    //images
    let mut king = StillImage::new(
    "assets/king.png",
    30.0,  // width 
    30.0,  // height
    30.0,  // x position
    0.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    let maze = StillImage::new(
    "assets/maze.png",
    VIRTUAL_WIDTH,  // width
    VIRTUAL_HEIGHT,  // height
    0.0,  // x position
    0.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    let end = StillImage::new(
    "assets/end.png",
    130.0,  // width
    100.0,  // height
    60.0,  // x position
    765.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    //labels
    let btn_return = TextButton::new(
    800.0,
    700.0,
    200.0,
    60.0,
    "Return to Menu",
    BLUE,
    GREEN,
    30
    );
    // Speed of movement in pixels per second
    const MOVE_SPEED: f32 = 200.0;
    let starttime = get_time();
    //labels
    let lbl_time_str = Label::new("Time:", 900.0, 40.0, 30);
    let mut lbl_time_num = Label::new("0", 965.0, 40.0, 30);
    //message box
    let mut end_box = MessageBox::info("Level Clear!", "You Beat The Maze!");
    loop {
        // Apply virtual resolution every frame
        use_virtual_resolution(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        clear_background(DARKGRAY);
        
        // Only process input if message box is NOT visible
        if !end_box.is_visible() {
            //switching to screen 1
            if btn_return.click() {
                return "screen1".to_string();
            }
            // Direction to move in
            let mut move_dir = vec2(0.0, 0.0);

            // Keyboard input
            if is_key_down(KeyCode::D) {
                move_dir.x += 1.0;
            }
            if is_key_down(KeyCode::A) {
                move_dir.x -= 1.0;
            }
            if is_key_down(KeyCode::S) {
                move_dir.y += 1.0;
            }
            if is_key_down(KeyCode::W) {
                move_dir.y -= 1.0;
            }

            // Normalize the movement to prevent faster diagonal movement
            if move_dir.length() > 0.0 {
                move_dir = move_dir.normalize();
            }

            // Apply movement based on frame time
            let movement = move_dir * MOVE_SPEED * get_frame_time();

            // Save old position in case of collision
            let old_pos = king.pos();
            // Move X first
            if movement.x != 0.0 {
                king.set_x(king.get_x() + movement.x);
                if check_collision(&king, &maze, 1) {
                    king.set_x(old_pos.x); // Undo if collision happens
                }
                if check_collision(&king, &end, 1) {
                    end_box.show();  // Just show the message box
                }
            }

            // Move Y next
            if movement.y != 0.0 {
                king.set_y(king.get_y() + movement.y);
                if check_collision(&king, &maze, 1)  {
                    king.set_y(old_pos.y); // Undo if collision happens
                }
                if check_collision(&king, &end, 1) {
                    end_box.show();  // Just show the message box
                }
            }
            let currenttime = format!("{:.1}", get_time()-starttime);
            lbl_time_num.set_text(currenttime);
        }
        maze.draw();
        king.draw();
        end.draw();
        lbl_time_num.draw();
        lbl_time_str.draw();
        end_box.centered();  // Center the message box on screen
        //draw_grid(50.0, LIGHTGRAY);
        // message box handling
        if let Some(result) = end_box.draw() {
            match result {
                MessageBoxResult::ButtonPressed(_) | MessageBoxResult::Closed => {
                    return "screen1".to_string();
                }
            }
        }
    next_frame().await;
    }
}