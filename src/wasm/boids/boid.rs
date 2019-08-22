extern crate vector2d;
extern crate rand;
use vector2d::Vector2D;
use crate::utils::vector2d::Vector2DExt;
use wasm_bindgen::__rt::core::f64::consts::PI;

pub const MAX_FORCE: f64 = 1.0;
pub const MAX_VELOCITY: f64 = 3.0;
pub const VIEW_RADIUS: f64 = 40.0;
pub const WEIGHT: f64 = 10.0;

const WANDER_CHANGEABLE_ANGLE: f64 = PI / 2.0;
const WANDER_HALF_CHANGEABLE_ANGLE: f64 = WANDER_CHANGEABLE_ANGLE / 2.0;
const WANDER_CIRCLE_DISTANCE: f64 = 2.0;
const WANDER_CIRCLE_RADIUS: f64 = 7.0;

#[derive(Clone)]
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
            velocity: Vector2D::new(rand::random::<f64>() * 10.0 - 5.0, rand::random::<f64>() * 10.0 - 5.0).normalise() * MAX_VELOCITY,
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

        self.steering += (desired_velocity - self.velocity).limit(MAX_FORCE);
    }

    pub fn wander(&mut self) {
        let circle_center = self.velocity.normalise() * WANDER_CIRCLE_DISTANCE;
        let angle: f64 = WANDER_CHANGEABLE_ANGLE * rand::random::<f64>() - WANDER_HALF_CHANGEABLE_ANGLE;
        let circle = self.wander_vector.rotate(angle) * WANDER_CIRCLE_RADIUS;
        let new_wander_vector = circle_center + circle;

        self.steering += new_wander_vector.limit(MAX_FORCE);
        self.wander_vector = new_wander_vector.normalise();
    }

    pub fn align(&mut self, boids: &Vec<Boid>)  {
        let mut steering = Vector2D::new(0.0, 0.0);

        for boid in boids {
            steering += boid.velocity;
        }

        if boids.len() > 0 {
            steering /= boids.len() as f64;
            steering = steering.normalise() * MAX_VELOCITY;
            self.steering += steering - self.velocity;
        }
    }

    pub fn cohesion(&mut self, boids: &Vec<Boid>) {
        let mut steering = Vector2D::new(0.0, 0.0);

        for boid in boids {
            steering += boid.position;
        }

        if boids.len() > 0 {
            steering /= boids.len() as f64;
            steering = (steering - self.position).normalise() * MAX_VELOCITY;
            steering -= self.velocity;
            steering = steering.limit(MAX_FORCE);
            self.steering += steering;
        }
    }

    pub fn separation(&mut self, boids: &Vec<Boid>) {
        let mut steering = Vector2D::new(0.0, 0.0);

        for boid in boids {
            steering += (self.position - boid.position).normalise();
        }

        if boids.len() > 0 {
            steering /= boids.len() as f64;
            steering = steering.normalise() * MAX_VELOCITY;
            steering -= self.velocity;
            steering = steering.limit(MAX_FORCE);
            self.steering += steering;
        }
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