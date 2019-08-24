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
    pub fn get_max_force() -> f64 {
        SETTINGS.with(|settings| settings.borrow().max_force)
    }

    pub fn set_max_force(val: f64) {
        SETTINGS.with(|settings| {
            settings.borrow_mut().max_force = val;
        });
    }

    pub fn get_max_velocity() -> f64 {
        SETTINGS.with(|settings| settings.borrow().max_velocity)
    }

    pub fn set_max_velocity(val: f64) {
        SETTINGS.with(|settings| {
           settings.borrow_mut().max_velocity = val;
        });
    }

    pub fn get_view_radius() -> f64 {
        SETTINGS.with(|settings| settings.borrow().view_radius)
    }

    pub fn set_view_radius(val: f64) {
        SETTINGS.with(|settings| {
            settings.borrow_mut().view_radius = val;
        });
    }

    pub fn get_weight() -> f64 {
        SETTINGS.with(|settings| settings.borrow().weight)
    }

    pub fn set_weight(val: f64) {
        SETTINGS.with(|settings| {
            settings.borrow_mut().weight = val;
        });
    }

    pub fn get_wander_changeable_angle() -> f64 {
        SETTINGS.with(|settings| settings.borrow().wander_changeable_angle)
    }

    pub fn set_wander_changeable_angle(val: f64) {
        SETTINGS.with(|settings| {
            settings.borrow_mut().wander_changeable_angle = val;
        });
    }

    pub fn get_wander_circle_distance() -> f64 {
        SETTINGS.with(|settings| settings.borrow().wander_circle_distance)
    }

    pub fn set_wander_circle_distance(val: f64) {
        SETTINGS.with(|settings| {
            settings.borrow_mut().wander_circle_distance = val;
        });
    }

    pub fn get_wander_circle_radius() -> f64 {
        SETTINGS.with(|settings| settings.borrow().wander_circle_radius)
    }

    pub fn set_wander_circle_radius(val: f64) {
        SETTINGS.with(|settings| {
            settings.borrow_mut().wander_circle_radius = val;
        });
    }

    pub fn set_mouse_position(x: f64, y: f64) {
        SETTINGS.with(|settings| {
           settings.borrow_mut().mouse_pos = Vector2D::new(x, y);
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


