use crate::boids;
use crate::components::*;
use crate::physics::PhysicsWorld;
use graphics::{Graphics, Transformed};
use legion::prelude::*;
use nalgebra::{Rotation2, Vector2};
use nphysics2d::algebra::{Force2, ForceType};
use nphysics2d::object::Body;
use opengl_graphics::GlGraphics;
use piston::input::{Event, RenderEvent, UpdateEvent};
use rand::Rng;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

pub fn create_draw_system(gl: Rc<RefCell<GlGraphics>>) -> Box<dyn Runnable> {
    SystemBuilder::new("draw_system")
        .read_resource::<Event>()
        .read_resource::<PhysicsWorld>()
        .with_query(<(Read<BodyHandle>, Read<ColliderHandle>)>::query())
        .build_thread_local(move |_commands, world, (event, physics_world), query| {
            if let Some(render_args) = event.render_args() {
                let graphics = &mut (*gl.borrow_mut());
                graphics.clear_color([1.0; 4]);

                let context = graphics.draw_begin(render_args.viewport());

                let physics_world: &PhysicsWorld = &*physics_world;

                let color = [0.0, 0.0, 0.0, 1.0];
                let polygon = boids::polygon_points();

                query.iter_immutable(&*world).for_each(|(body, collider)| {
                    let body = physics_world.bodies.rigid_body(body.0).unwrap();
                    let pos = body.position().translation.vector;
                    let rot = body.position().rotation.angle();
                    let transform = context.transform.trans(pos.x, pos.y).rot_rad(rot);
                    graphics::polygon(color, polygon.as_slice(), transform, graphics);
                });

                graphics.draw_end();
            }
        })
}

pub fn create_update_physics_world_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("update_physics_world_system")
        .read_resource::<Event>()
        .write_resource::<PhysicsWorld>()
        .with_query(<(Read<BodyHandle>, Read<WanderVelocity>)>::query())
        .build(move |_commands, world, (event, physics_world), query| {
            if let Some(update_args) = event.update_args() {
                let physics_world: &mut PhysicsWorld = &mut *physics_world;

                query.iter_immutable(&*world).for_each(|(body, wander)| {
                    let force = wander.0;

                    physics_world
                        .bodies
                        .rigid_body_mut(body.0)
                        .unwrap()
                        .apply_force(0, &Force2::linear(force), ForceType::VelocityChange, true);
                });

                physics_world.mechanical_world.set_timestep(update_args.dt);
                physics_world.mechanical_world.step(
                    &mut physics_world.geometrical_world,
                    &mut physics_world.bodies,
                    &mut physics_world.colliders,
                    &mut physics_world.joint_constraints,
                    &mut physics_world.force_generators,
                );
            }
        })
}

pub fn create_wander_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("wander_system")
        .read_resource::<Event>()
        .read_resource::<PhysicsWorld>()
        .with_query(<(Read<BodyHandle>, Write<WanderVelocity>)>::query())
        .build(move |_commands, world, (event, physics_world), query| {
            if let Some(update_args) = event.update_args() {
                let physics_world: &PhysicsWorld = &*physics_world;
                let mut rng = rand::thread_rng();

                query.iter(&mut *world).for_each(|(body, mut wander)| {
                    let body = physics_world.bodies.rigid_body(body.0).unwrap();
                    let velocity = body.velocity().linear.normalize() * 5.0;
                    wander.0 = Rotation2::new(rng.gen_range(0.0, PI * 2.0))
                        .transform_vector(&Vector2::new(2.0, 0.0));
                });
            }
        })
}
