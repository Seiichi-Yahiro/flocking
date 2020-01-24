use nalgebra::{Point2, Vector2};
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MousePosition(pub Point2<f64>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BodyHandle(pub DefaultBodyHandle);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColliderHandle(pub DefaultColliderHandle);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeekVelocity(pub Vector2<f64>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WanderVelocity(pub Vector2<f64>);
