use macroquad::prelude::*;

pub async fn run() -> String {
    loop {
        clear_background(DARKGRAY);
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();
        }

        next_frame().await;
    }
}