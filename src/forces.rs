use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, Join, System, WriteStorage, ReadStorage},
};
use specs_physics::PhysicsBody;
use nalgebra::{Point3};
use nalgebra::distance;
use specs_physics::nphysics::algebra::Force3;

pub struct RadialForceField {
    strength: f32,
}

impl Component for RadialForceField {
    type Storage = DenseVecStorage<Self>;
}

impl RadialForceField {
    pub fn new(strength: f32) -> Self {
        RadialForceField { strength }
    }
}

pub struct ForceSystem;

impl<'s> System<'s> for ForceSystem {
    type SystemData = (
        WriteStorage<'s, PhysicsBody<f32>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, RadialForceField>,
    );

    fn run(&mut self, (mut physics_bodies, transforms, radial_force_fields): Self::SystemData) {
        for (force_transform, radial_force_field) in (&transforms, &radial_force_fields).join() {
            for (body_transform, physics) in (&transforms, &mut physics_bodies).join() {
                let body_position = 
                    Point3::from(*(body_transform.translation()));
                let force_position = 
                    Point3::from(*(force_transform.translation()));
                let dist : f32 = distance(&force_position, &body_position);
                // Do not apply force to yourself
                if dist > 0.1 {
                    let applied_force_abs = radial_force_field.strength / dist / dist;
                    let force_vector = (body_position - force_position)
                        .normalize() * applied_force_abs; 
                    let force = Force3::linear(force_vector);
                    physics.apply_external_force(&force);
                }
            }
        }
    }
}
