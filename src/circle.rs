use macroquad::prelude::*;

//circle struct
pub struct Circle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub color: Color,
    pub timer: f32,
}

//circle implementation
impl Circle {
    //constructor
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, radius: f32, color: Color, timer: f32) -> Circle {
        Circle {
            position,
            velocity,
            acceleration,
            radius,
            color,
            timer,
        }
    }


    //update the circle
    pub fn update(&mut self, dt: f32, friction: f32, width: f32, height: f32, gravity: f32, restitution: f32) {
        //append gravity to y acceleration
        self.acceleration.y += gravity;
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;

        //invert the velocity if there is a collision with the edge of the screen
        if self.position.x + self.radius > width {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.position.x = width - self.radius;
            self.velocity.x *= -1.0 * restitution;
            self.velocity.y *= restitution;
            
        } else if self.position.x - self.radius < 0.0 {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.position.x = self.radius;
            self.velocity.x *= -1.0 * restitution;
            self.velocity.y *= restitution;
        }
        if self.position.y + self.radius > height {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.position.y = height - self.radius;
            self.velocity.y *= -1.0 * restitution;
            self.velocity.x *= restitution;
        } else if self.position.y - self.radius < 0.0 {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.position.y = self.radius;
            self.velocity.y *= -1.0 * restitution;
            self.velocity.x *= restitution;
        }

        self.timer += dt;
        if self.timer > 0.2 && (self.velocity.x.powi(2) + self.velocity.y.powi(2)).sqrt() < 10.0 {
            //set the velocity to random number from (-10 to 10) * 1000
            self.velocity = Vec2::new(
                rand::gen_range(-10.0, 10.0) * 1000.0,
                rand::gen_range(-10.0, 10.0) * 1000.0,
            );
            self.timer = 0.0;
        }

        
    }

    //draw the circle
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
        //draw line from center of circle to velocity vector of magnitude 100
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.velocity.x / 10.0,
            self.position.y + self.velocity.y / 10.0,
            2.0,
            RED,
        );
    }
}