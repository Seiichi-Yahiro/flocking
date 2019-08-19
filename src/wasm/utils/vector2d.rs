extern crate vector2d;

use vector2d::Vector2D;

pub trait Vector2DExt<T> {
    fn limit(&self, val: T) -> Self;
}

impl Vector2DExt<f64> for Vector2D<f64> {
    fn limit(&self, val: f64) -> Self {
        if self.length_squared() > val.powi(2) {
            self.normalise() * val
        } else {
            *self
        }
    }
}