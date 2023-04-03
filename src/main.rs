use macroquad::prelude::*;
use std::process::exit;

mod circle;
use circle::Circle;

#[macroquad::main("Physics Engine")]
async fn main() {
    let width: f32 = 800.0;
    let height: f32 = 600.0;
    let friction: f32 = 3.0;
    let gravity: f32 = 1000.0;
    let restitution: f32 = 0.8;
    request_new_screen_size(width, height);
    //create blue circle
    let mut circle: Circle = Circle::new(
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        25.0,
        BLUE,
        0.0,
    );

    loop {
        let width: f32 = screen_width();
        let height: f32 = screen_height();
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
        draw_text(&format!("{}", (circle.velocity.x.powi(2) + circle.velocity.y.powi(2)).sqrt()), width - width / 30.0, height / 15.0, 20.0, WHITE);

        //update the circle
        circle.update(dt, friction, width, height, gravity, restitution);

        //draw the circle
        circle.draw();

        next_frame().await
    }
}