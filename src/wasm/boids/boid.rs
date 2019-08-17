extern crate vector2d;
use vector2d::Vector2D;

#[derive(Clone)]
pub struct Boid {
    pub position: Vector2D<f64>,
    pub velocity: Vector2D<f64>
}

impl Boid {
    pub fn set_velocity(mut self, velocity: Vector2D<f64>) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn calculate_next_position(mut self, width: f64, height: f64) -> Self {
        let mut pos = self.position + self.velocity;

        if pos.x < 0.0 {
            pos.x += width;
        } else if pos.x > width {
            pos.x -= width;
        }

        if pos.y < 0.0 {
            pos.y += height;
        } else if pos.y > height {
            pos.y -= height;
        }

        self.position = pos;

        self
    }
}