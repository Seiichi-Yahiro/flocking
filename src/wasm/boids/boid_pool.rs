extern crate vector2d;

use vector2d::Vector2D;
use crate::utils::vector2d::Vector2DExt;
use super::boid::{Boid, VIEW_RADIUS};

#[link(wasm_import_module = "../src/canvas.js")]
extern {
    fn clear_canvas();
    fn draw_boid(x: f64, y: f64, angle: f64);
}

pub struct BoidPool {
    boids: Vec<Boid>,
}

impl BoidPool {
    pub fn new() -> BoidPool {
        BoidPool {
            boids: vec![]
        }
    }

    pub fn add_boid(&mut self, x: f64, y: f64) {
        self.boids.push(Boid::new(Vector2D::new(x, y)));
    }

    pub fn update(&mut self, width: &f64, height: &f64, mouse_pos: &Vector2D<f64>) {
        let boids_clone = self.boids.clone();

        for boid in &mut self.boids {
            let close_boids: Vec<Boid> = boids_clone.clone().into_iter()
                .filter(|other| {
                    let distance = (other.position - boid.position).length();
                    distance <= VIEW_RADIUS && distance != 0.0
                })
                .collect();
            boid.align(&close_boids);
            boid.cohesion(&close_boids);
            boid.separation(&close_boids);
            boid.wander();
            //boid.seek(mouse_pos);
            boid.update(width, height);
        }
    }

    pub fn render(&self) {
        unsafe {
            clear_canvas();

            for boid in &self.boids {
                draw_boid(boid.position.x, boid.position.y, boid.velocity.angle());
            }
        }
    }
}
