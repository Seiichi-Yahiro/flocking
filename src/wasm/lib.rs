extern crate wasm_bindgen;
extern crate vector2d;

mod utils;
mod boids;

use wasm_bindgen::prelude::*;
use vector2d::Vector2D;
use boids::boid_pool::BoidPool;

#[wasm_bindgen]
pub struct App {
    width: f64,
    height: f64,
    mouse_pos: Vector2D<f64>,
    boid_pool: BoidPool
}

#[wasm_bindgen]
impl App {
    pub fn new(width: f64, height: f64) -> App {
        App {
            width,
            height,
            mouse_pos: Vector2D::new(width / 2.0, height / 2.0),
            boid_pool: BoidPool::new()
        }
    }

    pub fn set_mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse_pos.x = x;
        self.mouse_pos.y = y;
    }

    pub fn add_boid(&mut self, x: f64, y: f64) {
        self.boid_pool.add_boid(x, y);
    }

    pub fn update(&mut self) {
        self.boid_pool.update(&self.width, &self.height, &self.mouse_pos);
    }

    pub fn render(&self) {
        self.boid_pool.render();
    }
}