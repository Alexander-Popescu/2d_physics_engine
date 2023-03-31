use macroquad::prelude::*;
use std::process::exit;

mod circle;
use circle::Circle;

#[macroquad::main("Physics Engine")]
async fn main() {
    let width: f32 = 800.0;
    let height: f32 = 600.0;
    let friction: f32 = 0.9;
    request_new_screen_size(width, height);
    //create blue circle
    let mut circle: Circle = Circle::new(
        Vec2::new(width / 2.0, height / 2.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        50.0,
        BLUE,
    );

    loop {
        //this frame scope variables
        let dt: f32 = get_frame_time();
        let fps: i32 = get_fps();

        //keyboard inputs

        if is_key_down(KeyCode::Escape) {
            //exit the program
            exit(0);
        }

        //set velocity based on wasd keys for x and y
        circle.acceleration = Vec2::new(
            if is_key_down(KeyCode::D) {
                500.0
            } else if is_key_down(KeyCode::A) {
                -500.0
            } else {
                0.0
            },
            if is_key_down(KeyCode::W) {
                -500.0
            } else if is_key_down(KeyCode::S) {
                500.0
            } else {
                0.0
            },
        );

        //clear the screen to black background
        clear_background(BLACK);

        //render fps counter
        draw_text(&format!("{}", fps), width - width / 30.0, height / 30.0, 20.0, WHITE);
        
        //update the circle
        circle.update(dt, &friction);

        //draw the circle
        circle.draw();

        next_frame().await
    }
}
