use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub async fn run() -> String {
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
        1024.0,  // width
        768.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
        ).await;
        // Speed of movement in pixels per second
        const MOVE_SPEED: f32 = 200.0;
    loop {
        clear_background(DARKGRAY);
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);
        //switching to screen 1
        if is_key_pressed(KeyCode::Space) {
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
        }

        // Move Y next
        if movement.y != 0.0 {
            king.set_y(king.get_y() + movement.y);
            if check_collision(&king, &maze, 1)  {
                king.set_y(old_pos.y); // Undo if collision happens
            }
        }
        maze.draw();
        king.draw();
        println!("King position: ({:.2}, {:.2})", king.get_x(), king.get_y());
    next_frame().await;
    }
}