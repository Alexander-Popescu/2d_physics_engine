use macroquad::prelude::*;
use std::process::exit;

mod circle;
use circle::Circle;

#[macroquad::main("Physics Engine")]
async fn main() {
    let width: f32 = 800.0;
    let height: f32 = 600.0;
    let friction: f32 = 0.1;
    let gravity: f32 = 9.81;
    let restitution: f32 = 0.75;
    let drag = 0.995;
    let render_debug_lines = false;
    request_new_screen_size(width, height);
    //create blue circle
    let mut circle: Circle = Circle::new(
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        50.0,
        BLUE,
        0.5,
    );

    let mut timer = 0.0;

    loop {
        let width: f32 = screen_width();
        let height: f32 = screen_height();
        //this frame scope variables
        let dt: f32 = get_frame_time();
        timer += dt;
        let fps: i32 = get_fps();

        //keyboard inputs
        if is_key_down(KeyCode::Escape) {
            //exit the program
            exit(0);
        }
        if is_key_down(KeyCode::R) {
            //set the velocity to random number from (-10 to 10) * 500
            circle.velocity = Vec2::new(
                rand::gen_range(-10.0, 10.0) * 500.0,
                rand::gen_range(-10.0, 10.0) * 500.0,
            );
            timer = 0.0;
        }
        if is_key_down(KeyCode::Y) {
            //set the velocity to random number from (-10 to 10) * 500
            circle.velocity = Vec2::new(
                0.0,
                0.0,
            );
            timer = 0.0;
        }

        //set velocity based on wasd keys for x and y
        circle.acceleration = Vec2::new(
            if is_key_down(KeyCode::D) {
                1000.0
            } else if is_key_down(KeyCode::A) {
                -1000.0
            } else {
                0.0
            },
            if is_key_down(KeyCode::W) {
                -1000.0
            } else if is_key_down(KeyCode::S) {
                1000.0
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
        circle.update(dt, friction, width, height, gravity, restitution, drag);

        //draw the circle
        circle.draw(render_debug_lines);

        next_frame().await
    }
}