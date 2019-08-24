use vector2d::Vector2D;
use crate::utils::vector2d::Vector2DExt;
use crate::settings::SETTINGS;

#[derive(Clone)]
pub struct Boid {
    pub position: Vector2D<f64>,
    pub velocity: Vector2D<f64>,
    steering: Vector2D<f64>,
    wander_vector: Vector2D<f64>
}

impl Boid {
    pub fn new(position: Vector2D<f64>) -> Boid {
        SETTINGS.with(|settings| {
            Boid {
                position,
                velocity: Vector2D::new(rand::random::<f64>() * 10.0 - 5.0, rand::random::<f64>() * 10.0 - 5.0).normalise() * settings.borrow().max_velocity,
                steering: Vector2D::new(0.0, 0.0),
                wander_vector: Vector2D::new(0.0, 0.0)
            }
        })
    }

    pub fn seek(&mut self, target: &Vector2D<f64>) {
        SETTINGS.with(|settings| {
            let desired_position = *target - self.position;
            let distance = desired_position.length();
            let mut desired_velocity = desired_position.normalise() * settings.borrow().max_velocity;

            // slow down the closer the boid gets
            if distance <= settings.borrow().view_radius {
                desired_velocity *= distance / settings.borrow().view_radius;
            }

            self.steering += (desired_velocity - self.velocity).limit(settings.borrow().max_force);
        })
    }

    pub fn wander(&mut self) {
        SETTINGS.with(|settings| {
            let circle_center = self.velocity.normalise() * settings.borrow().wander_circle_distance;
            let angle: f64 = settings.borrow().wander_changeable_angle * rand::random::<f64>() - settings.borrow().wander_changeable_angle / 2.0;
            let circle = self.wander_vector.rotate(angle) * settings.borrow().wander_circle_radius;
            let new_wander_vector = circle_center + circle;

            self.steering += new_wander_vector.limit(settings.borrow().max_force);
            self.wander_vector = new_wander_vector.normalise();
        })
    }

    pub fn align(&mut self, boids: &Vec<&Boid>)  {
        let mut steering = Vector2D::new(0.0, 0.0);

        for boid in boids {
            steering += boid.velocity;
        }

        if boids.len() > 0 {
            SETTINGS.with(|settings| {
                steering /= boids.len() as f64;
                steering = steering.normalise() * settings.borrow().max_velocity;
                self.steering += steering - self.velocity;
            });
        }
    }

    pub fn cohesion(&mut self, boids: &Vec<&Boid>) {
        let mut steering = Vector2D::new(0.0, 0.0);

        for boid in boids {
            steering += boid.position;
        }

        if boids.len() > 0 {
            SETTINGS.with(|settings| {
                steering /= boids.len() as f64;
                steering = (steering - self.position).normalise() * settings.borrow().max_velocity;
                steering -= self.velocity;
                steering = steering.limit(settings.borrow().max_force);
                self.steering += steering;
            });
        }
    }

    pub fn separation(&mut self, boids: &Vec<&Boid>) {
        let mut steering = Vector2D::new(0.0, 0.0);

        for boid in boids {
            steering += (self.position - boid.position).normalise();
        }

        if boids.len() > 0 {
            SETTINGS.with(|settings| {
                steering /= boids.len() as f64;
                steering = steering.normalise() * settings.borrow().max_velocity;
                steering -= self.velocity;
                steering = steering.limit(settings.borrow().max_force);
                self.steering += steering;
            });
        }
    }

    pub fn update(&mut self) {
        SETTINGS.with(|settings| {
            let velocity = self.velocity + self.steering.limit(settings.borrow().max_force) / settings.borrow().weight;
            self.velocity = velocity.limit(settings.borrow().max_velocity);
            self.calculate_next_position();
            self.steering = Vector2D::new(0.0, 0.0);
        });
    }

    fn calculate_next_position(&mut self) {
        let mut pos = self.position + self.velocity;

        SETTINGS.with(|settings| {
            if pos.x < 0.0 {
                pos.x += settings.borrow().width;
            } else if pos.x >= settings.borrow().width {
                pos.x -= settings.borrow().width;
            }

            if pos.y < 0.0 {
                pos.y += settings.borrow().height;
            } else if pos.y >= settings.borrow().height {
                pos.y -= settings.borrow().height;
            }
        });

        self.position = pos;
    }
}