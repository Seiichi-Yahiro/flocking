extern crate wasm_bindgen;
extern crate vector2d;
extern crate rand;

mod utils;
mod settings;
mod boids;

use wasm_bindgen::prelude::*;
use boids::boid_pool::BoidPool;

#[wasm_bindgen]
pub struct App {
    boid_pool: BoidPool
}

#[wasm_bindgen]
impl App {
    pub fn new() -> App {
        App {
            boid_pool: BoidPool::new()
        }
    }

    pub fn set_mouse_pos(&mut self, x: f64, y: f64) {

    }

    pub fn add_boid(&mut self, x: f64, y: f64) {
        self.boid_pool.add_boid(x, y);
    }

    pub fn update(&mut self) {
        self.boid_pool.update();
    }

    pub fn render(&self) {
        self.boid_pool.render();
    }
}