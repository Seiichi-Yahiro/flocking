use super::boid::Boid;
use super::lattice::Lattice;
use vector2d::Vector2D;

pub struct BoidPool {
    boids: Vec<Boid>,
}

impl BoidPool {
    pub fn new() -> BoidPool {
        BoidPool { boids: vec![] }
    }

    pub fn add_boid(&mut self, x: f64, y: f64) {
        self.boids.push(Boid::new(Vector2D::new(x, y)));
    }

    pub fn update(&mut self, width: &f64, height: &f64, mouse_pos: &Vector2D<f64>) {
        let lattice = Lattice::new(&self.boids, width, height);

        for boid in &mut self.boids {
            let close_boids = lattice.get_neighbors(boid);

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
