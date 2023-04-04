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
    pub fn update(&mut self, dt: f32, friction: f32, width: f32, height: f32, gravity: f32, restitution: f32, drag: f32) {
        //append gravity to y acceleration
        self.acceleration.y += gravity;

        //add drag to the object to slow it down
        self.velocity -= self.velocity * (1.0 - drag);

        //add a spring force toward center of screen
        let spring_location = Vec2::new(width / 2.0, height / 2.0);
        let displacement = spring_location - self.position;
        let spring_force = displacement * 10.0;
        self.acceleration += spring_force;
        draw_line(
            self.position.x,
            self.position.y,
            width / 2.0,
            height / 2.0,
            2.0,
            WHITE,
        );




        //invert the velocity if there is a collision with the edge of the screen
        if self.position.x + self.radius > width {
            self.position.x = width - self.radius;
            self.velocity.x *= -1.0 * restitution;
            self.velocity.y = self.velocity.y * (1.0 - friction);
            
        } else if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x *= -1.0 * restitution;
            self.velocity.y = self.velocity.y * (1.0 - friction);
        }
        if self.position.y + self.radius > height {
            self.position.y = height - self.radius;
            self.velocity.y *= -1.0 * restitution;
            self.velocity.x = self.velocity.x * (1.0 - friction);
        } else if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y *= -1.0 * restitution;
            self.velocity.x = self.velocity.x * (1.0 - friction);
        }

        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
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