use macroquad::prelude::*;
use crate::modules::still_image::StillImage;

pub struct Player {
    view: StillImage,
    move_speed: f32,
}

impl Player {
    pub async fn new(image_path: &str, x: f32, y: f32) -> Self {
        let view = StillImage::new(
            image_path,
            40.0,  // width 
            40.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;

        Player {
            view,
            move_speed: 200.0, // Movement speed in pixels per second
        }
    }
    // Direction to move in
    pub fn move_player(&mut self) {
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
        let movement = move_dir * self.move_speed * get_frame_time();
        self.view.set_x(self.view.get_x() + movement.x);
        self.view.set_y(self.view.get_y() + movement.y);
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }

    pub fn set_oldpos(&mut self, x: f32, y: f32) {
        self.view.set_x(x);
        self.view.set_y(y);
    }
}