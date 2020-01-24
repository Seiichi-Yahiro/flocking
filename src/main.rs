mod boids;
mod components;
mod physics;
mod systems;

use crate::physics::PhysicsWorld;
use glutin_window::{GlutinWindow, OpenGL};
use legion::schedule::Schedule;
use legion::world::World;
use nalgebra::{Point2, Vector2};
use nphysics_testbed2d::Testbed;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::window::WindowSettings;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let opengl_version = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Flocking", [500, 500])
        .graphics_api(opengl_version)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new().max_fps(30).ups(60));

    let gl = Rc::new(RefCell::new(GlGraphics::new(opengl_version)));
    let mut world = World::new();
    let mut physics_world = PhysicsWorld::new();
    let mut schedule = Schedule::builder()
        .add_system(systems::create_update_mouse_position_system())
        //.add_system(systems::create_wander_system())
        .add_system(systems::create_seek_system())
        .add_system(systems::create_update_physics_world_system())
        .add_thread_local(systems::create_draw_system(gl.clone()))
        .build();

    for _ in 0..10 {
        boids::create_random(&mut world, &mut physics_world);
    }

    boids::Builder::new()
        .pos(Vector2::new(200.0, 200.0))
        .vel(Vector2::new(5.0, 0.0))
        .build(&mut world, &mut physics_world);
    boids::Builder::new()
        .pos(Vector2::new(250.0, 200.0))
        .vel(Vector2::new(-5.0, 0.0))
        .build(&mut world, &mut physics_world);

    world.resources.insert(physics_world);
    world
        .resources
        .insert(components::MousePosition(Point2::new(0.0, 0.0)));

    while let Some(event) = events.next(&mut window) {
        world.resources.insert(event);
        schedule.execute(&mut world);
    }

    /*let mut testbed = Testbed::new(
        physics_world.mechanical_world,
        physics_world.geometrical_world,
        physics_world.bodies,
        physics_world.colliders,
        physics_world.joint_constraints,
        physics_world.force_generators,
    );

    testbed.look_at(Point2::new(200.0, 200.0), 1.0);
    testbed.run();*/
}
