extern crate wasm_bindgen;
extern crate vector2d;
extern crate rand;

mod utils;
mod settings;
mod lattice;
mod boid;

use wasm_bindgen::prelude::*;
use boid::Boid;
use lattice::Lattice;
use vector2d::Vector2D;

#[link(wasm_import_module = "../src/canvas.js")]
extern {
    fn clear_canvas();
    fn draw_boid(x: f64, y: f64, angle: f64);
}

#[wasm_bindgen]
pub struct Simulation {
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            boids: vec![]
        }
    }

    pub fn add_boid(&mut self, x: f64, y: f64) {
        self.boids.push(Boid::new(Vector2D::new(x, y)));
    }

    pub fn tick(&mut self) {
        self.update();
        self.render();
    }

    fn update(&mut self) {
        let lattice = Lattice::new(&self.boids);

        for boid in &mut self.boids {
            let close_boids = lattice.get_neighbors(boid);

            boid.align(&close_boids);
            boid.cohesion(&close_boids);
            boid.separation(&close_boids);
            boid.wander();
            //boid.seek(mouse_pos);
            boid.update();
        }
    }

    fn render(&self) {
        unsafe {
            clear_canvas();

            for boid in &self.boids {
                draw_boid(boid.position.x, boid.position.y, boid.velocity.angle());
            }
        }
    }
}