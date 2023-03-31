use macroquad::prelude::*;

//circle struct
pub struct Circle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub color: Color,
}

//circle implementation
impl Circle {
    //constructor
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, radius: f32, color: Color) -> Circle {
        Circle {
            position,
            velocity,
            acceleration,
            radius,
            color,
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
            self.velocity += self.velocity * -1.0 * friction * dt;
            self.position.x = width - self.radius;
            self.velocity.x *= -1.0 * restitution;
            
        } else if self.position.x - self.radius < 0.0 {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.velocity += self.velocity * -1.0 * friction * dt;
            self.position.x = self.radius;
            self.velocity.x *= -1.0 * restitution;
        }
        if self.position.y + self.radius > height {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.velocity += self.velocity * -1.0 * friction * dt;
            self.position.y = height - self.radius;
            self.velocity.y *= -1.0 * restitution;
        } else if self.position.y - self.radius < 0.0 {
            //add a vector to the velocity vector of opposite direction and magnitude friction
            self.velocity += self.velocity * -1.0 * friction * dt;
            self.position.y = self.radius;
            self.velocity.y *= -1.0 * restitution;
        }
        
    }

    //draw the circle
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}