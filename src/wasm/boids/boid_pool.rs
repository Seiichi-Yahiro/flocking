extern crate wasm_bindgen;
extern crate vector2d;

use wasm_bindgen::prelude::*;
use vector2d::Vector2D;
use super::boid::Boid;

#[link(wasm_import_module = "../src/canvas.js")]
extern {
    fn clear_canvas();
    fn draw_boid(x: f64, y: f64, angle: f64);
}

#[wasm_bindgen]
pub struct BoidPool {
    boids: Vec<Boid>,
    width: f64,
    height: f64,
    mouse_pos: Vector2D<f64>
}

#[wasm_bindgen]
impl BoidPool {
    pub fn new(width: f64, height: f64) -> BoidPool {
        BoidPool {
            boids: vec![],
            width,
            height,
            mouse_pos: Vector2D::new(width / 2.0, height / 2.0)
        }
    }

    pub fn add_boid(&mut self, x: f64, y: f64) {
        self.boids.push(Boid::new(Vector2D::new(x, y)));
    }

    pub fn set_mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse_pos = Vector2D::new(x, y);
    }

    pub fn update(&mut self) {
        for boid in &mut self.boids {
            //boid.seek(&self.mouse_pos);
            boid.wander();
            boid.update(self.width, self.height);
        }
    }

    /*pub fn update(&mut self) {
        let mut new_boids: Vec<Boid> = vec![];

        for boid in &self.boids {
            let mut number_of_neighbors = 0;

            let mut alignment_velocity = boid.velocity.clone();
            let mut cohesion_velocity = boid.velocity.clone();
            let mut separation_velocity = boid.velocity.clone();

            for other in &self.boids {
                if boid as *const Boid == other as *const Boid {
                    continue;
                }

                let distance = (other.position - boid.position).length();
                if distance > 100.0 {
                    continue;
                }

                number_of_neighbors += 1;

                alignment_velocity += other.velocity;
                cohesion_velocity += other.position;
                separation_velocity += Vector2D::new(distance, distance);
            }

            if number_of_neighbors == 0 {
                new_boids.push(boid.clone().calculate_next_position(self.width, self.height));
                continue;
            }

            alignment_velocity = (alignment_velocity / number_of_neighbors as f64).normalise();
            cohesion_velocity = (cohesion_velocity / number_of_neighbors as f64 - boid.position).normalise();
            separation_velocity = (separation_velocity / number_of_neighbors as f64 * -1.0).normalise();

            let new_velocity = (alignment_velocity + cohesion_velocity + separation_velocity).normalise();
            let new_boid = boid
                .clone()
                .set_velocity(new_velocity)
                .calculate_next_position(self.width, self.height);

            new_boids.push(new_boid);
        }

        self.boids = new_boids;
    }*/

    pub fn render(&self) {
        unsafe {
            clear_canvas();

            for boid in &self.boids {
                draw_boid(boid.position.x, boid.position.y, boid.velocity.angle());
            }
        }
    }
}
