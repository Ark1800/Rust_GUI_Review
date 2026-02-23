use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::messagebox::{MessageBox, MessageBoxResult};
use crate::modules::collision::check_collision;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::player::Player;

pub async fn run() -> String {
    // Define virtual resolution constants
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    //images
    let mut player = Player::new("assets/king.png", 30.0, 30.0).await;
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
    let wall = StillImage::new(
    "assets/end.png",
    130.0,  // width
    100.0,  // height
    0.0,  // x position
    -100.0,   // y position
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
            player.move_player();
            let old_pos = vec2(player.view_player().get_x(), player.view_player().get_y());
            //x collision
            if check_collision(player.view_player(), &maze, 1) {
                println!("Collided with maze");
                player.set_oldpos(old_pos.x, old_pos.y); // Undo if collision happens
            }
            if check_collision(player.view_player(), &wall, 1) {
                player.set_oldpos(old_pos.x, old_pos.y); // Undo if collision happens
            }
            if check_collision(player.view_player(), &end, 1) {
                end_box.show();  // Just show the message box
            }
        }
        if btn_return.click() {
            return "screen1".to_string();
        }
        let currenttime = format!("{:.1}", get_time()-starttime);
        lbl_time_num.set_text(currenttime);
        maze.draw();
        player.draw();
        end.draw();
        wall.draw();
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