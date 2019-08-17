extern crate vector2d;
use vector2d::Vector2D;

#[derive(Clone)]
pub struct Boid {
    pub position: Vector2D<f64>,
    pub velocity: Vector2D<f64>
}