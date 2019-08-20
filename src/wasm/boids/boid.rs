extern crate vector2d;
extern crate rand;
use vector2d::Vector2D;
use crate::utils::vector2d::Vector2DExt;
use wasm_bindgen::__rt::core::f64::consts::PI;

const MAX_FORCE: f64 = 1.0;
const MAX_VELOCITY: f64 = 3.0;
const VIEW_RADIUS: f64 = 40.0;
const WEIGHT: f64 = 10.0;

const WANDER_CHANGEABLE_ANGLE: f64 = PI / 2.0;
const WANDER_HALF_CHANGEABLE_ANGLE: f64 = WANDER_CHANGEABLE_ANGLE / 2.0;
const WANDER_CIRCLE_DISTANCE: f64 = 2.0;
const WANDER_CIRCLE_RADIUS: f64 = 7.0;

pub struct Boid {
    pub position: Vector2D<f64>,
    pub velocity: Vector2D<f64>,
    steering: Vector2D<f64>,
    wander_vector: Vector2D<f64>
}

impl Boid {
    pub fn new(position: Vector2D<f64>) -> Boid {
        Boid {
            position,
            velocity: Vector2D::new(rand::random::<f64>(), rand::random::<f64>()).normalise() * MAX_VELOCITY,
            steering: Vector2D::new(0.0, 0.0),
            wander_vector: Vector2D::new(0.0, 0.0)
        }
    }

    pub fn seek(&mut self, target: &Vector2D<f64>) {
        let desired_position = *target - self.position;
        let distance = desired_position.length();
        let mut desired_velocity = desired_position.normalise() * MAX_VELOCITY;

        // slow down the closer the boid gets
        if distance <= VIEW_RADIUS {
            desired_velocity *= distance / VIEW_RADIUS;
        }

        self.steering += desired_velocity - self.velocity;
    }

    pub fn wander(&mut self) {
        let circle_center = self.velocity.normalise() * WANDER_CIRCLE_DISTANCE;

        let angle: f64 = WANDER_CHANGEABLE_ANGLE * rand::random::<f64>() - WANDER_HALF_CHANGEABLE_ANGLE;
        let angle_cos = angle.cos();
        let angle_sin = angle.sin();

        let x = self.wander_vector.x * angle_cos - self.wander_vector.y * angle_sin;
        let y = self.wander_vector.x * angle_sin + self.wander_vector.y * angle_cos;

        let new_wander_vector = circle_center + Vector2D::new(x, y) * WANDER_CIRCLE_RADIUS;

        self.steering += new_wander_vector.limit(MAX_FORCE);
        self.wander_vector = new_wander_vector.normalise();
    }

    pub fn update(&mut self, width: &f64, height: &f64) {
        let velocity = self.velocity + self.steering.limit(MAX_FORCE) / WEIGHT;
        self.velocity = velocity.limit(MAX_VELOCITY);
        self.calculate_next_position(width, height);
        self.steering = Vector2D::new(0.0, 0.0);
    }

    fn calculate_next_position(&mut self, width: &f64, height: &f64) {
        let mut pos = self.position + self.velocity;

        if pos.x < 0.0 {
            pos.x += *width;
        } else if pos.x >= *width {
            pos.x -= *width;
        }

        if pos.y < 0.0 {
            pos.y += *height;
        } else if pos.y >= *height {
            pos.y -= *height;
        }

        self.position = pos;
    }
}