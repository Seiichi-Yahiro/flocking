use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BodyHandle(pub DefaultBodyHandle);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColliderHandle(pub DefaultColliderHandle);

pub struct WanderVelocity(pub Vector2<f64>);
