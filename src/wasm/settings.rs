use wasm_bindgen::__rt::core::f64::consts::PI;
use vector2d::Vector2D;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub struct Settings {
    pub width: f64,
    pub height: f64,

    pub mouse_pos: Vector2D<f64>,

    pub max_force: f64,
    pub max_velocity: f64,
    pub view_radius: f64,
    pub weight: f64,

    pub wander_changeable_angle: f64,
    pub wander_circle_distance: f64,
    pub wander_circle_radius: f64,
}

impl Settings {
    pub fn new(width: f64, height: f64) -> Settings {
        Settings {
            width,
            height,
            mouse_pos: Vector2D::new(0.0, 0.0),
            max_force: 1.0,
            max_velocity: 3.0,
            view_radius: 20.0,
            weight: 10.0,

            wander_changeable_angle: PI / 2.0,
            wander_circle_distance: 2.0,
            wander_circle_radius: 7.0,
        }
    }
}

#[wasm_bindgen]
impl Settings {
    pub fn get_max_velocity() -> f64 {
        SETTINGS.with(|settings| settings.borrow().max_velocity)
    }

    pub fn set_max_velocity(val: f64) {
        SETTINGS.with(|settings| {
           settings.borrow_mut().max_velocity = val;
        });
    }
}

thread_local!(
    pub static SETTINGS: RefCell<Settings> = RefCell::new(
        Settings {
            width: 800.0,
            height: 600.0,
            mouse_pos: Vector2D::new(0.0, 0.0),
            max_force: 1.0,
            max_velocity: 3.0,
            view_radius: 20.0,
            weight: 10.0,

            wander_changeable_angle: PI / 2.0,
            wander_circle_distance: 2.0,
            wander_circle_radius: 7.0,
        }
    )
);


