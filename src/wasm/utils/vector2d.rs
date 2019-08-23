use vector2d::Vector2D;

pub trait Vector2DExt<T> {
    fn limit(&self, val: T) -> Self;
    fn rotate(&self, angle: f64) -> Self;
}

impl Vector2DExt<f64> for Vector2D<f64> {
    fn limit(&self, val: f64) -> Self {
        if self.length_squared() > val.powi(2) {
            self.normalise() * val
        } else {
            *self
        }
    }

    fn rotate(&self, angle: f64) -> Self {
        let angle_cos = angle.cos();
        let angle_sin = angle.sin();

        let x = self.x * angle_cos - self.y * angle_sin;
        let y = self.x * angle_sin + self.y * angle_cos;

        Self { x, y }
    }
}
