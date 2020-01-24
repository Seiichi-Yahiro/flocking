use crate::components;
use crate::physics::PhysicsWorld;
use itertools::Itertools;
use legion::world::World;
use nalgebra::{Point2, RealField, Rotation2, Vector2};
use ncollide2d::shape::{ConvexPolygon, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::object::{BodyPartHandle, ColliderDesc, RigidBodyDesc};
use rand::Rng;

pub fn polygon_points() -> Vec<[f64; 2]> {
    vec![[0.0, 1.5], [-0.5, 0.0], [0.5, 0.0]]
        .iter()
        .map(|[x, y]| [*x * 10.0, *y * 10.0])
        .collect_vec()
}

pub struct Builder<N: RealField = f64> {
    pos: Vector2<N>,
    vel: Vector2<N>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            pos: nalgebra::zero(),
            vel: nalgebra::zero(),
        }
    }

    pub fn pos(mut self, pos: Vector2<f64>) -> Self {
        self.pos = pos;
        self
    }

    pub fn vel(mut self, vel: Vector2<f64>) -> Self {
        self.vel = vel;
        self
    }

    pub fn build(self, world: &mut World, physics_world: &mut PhysicsWorld<f64>) {
        let body_handle = {
            let body = RigidBodyDesc::new()
                .translation(self.pos)
                .velocity(Velocity2::new(self.vel, nalgebra::zero()))
                .rotation(Rotation2::rotation_between(&Vector2::y_axis(), &self.vel).angle())
                .mass(100.0)
                //.max_linear_velocity(15.0)
                //.max_angular_velocity(1.0)
                //.angular_damping(1.0)
                .gravity_enabled(false)
                .build();

            physics_world.bodies.insert(body)
        };

        let collider_handle = {
            let points = polygon_points()
                .iter()
                .map(|[x, y]| Point2::new(*x, *y))
                .collect_vec();

            let shape =
                ShapeHandle::new(ConvexPolygon::try_from_points(points.as_slice()).unwrap());

            let collider = ColliderDesc::new(shape)
                .density(1.0)
                .build(BodyPartHandle(body_handle, 0));

            physics_world.colliders.insert(collider)
        };

        world.insert(
            (),
            vec![(
                components::BodyHandle(body_handle),
                components::ColliderHandle(collider_handle),
                components::WanderVelocity(nalgebra::zero()),
                components::SeekVelocity(nalgebra::zero()),
            )],
        );
    }
}

pub fn create_random(world: &mut World, physics_world: &mut PhysicsWorld<f64>) {
    let mut rng = rand::thread_rng();
    Builder::new()
        .pos(Vector2::new(
            rng.gen_range(0.0, 500.0),
            rng.gen_range(0.0, 500.0),
        ))
        .vel(Vector2::new(
            rng.gen_range(-15.0, 15.0),
            rng.gen_range(-15.0, 15.0),
        ))
        .build(world, physics_world);
}
