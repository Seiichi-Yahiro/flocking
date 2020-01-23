use nalgebra::RealField;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub struct PhysicsWorld<N: RealField = f64> {
    pub mechanical_world: DefaultMechanicalWorld<N>,
    pub geometrical_world: DefaultGeometricalWorld<N>,
    pub bodies: DefaultBodySet<N>,
    pub colliders: DefaultColliderSet<N>,
    pub joint_constraints: DefaultJointConstraintSet<N>,
    pub force_generators: DefaultForceGeneratorSet<N>,
}

impl<N: RealField> PhysicsWorld<N> {
    pub fn new() -> PhysicsWorld<N> {
        PhysicsWorld {
            mechanical_world: DefaultMechanicalWorld::new(nalgebra::zero()),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
        }
    }
}
