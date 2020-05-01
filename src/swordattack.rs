use crate::{
    charactermeta::CharacterDirection, damage::Destroyer, delayedremove::DelayedRemove,
};
use specs_physics::{PhysicsBodyBuilder, PhysicsBody,
    nphysics::object::BodyStatus,
    nalgebra::{Vector3},
    PhysicsColliderBuilder,
    PhysicsCollider,
    colliders::Shape,
};
use amethyst::{core::Transform, ecs::world::World, prelude::*};

pub fn sword_attack(
    world: &mut World,
    strength: f32,
    transform: Transform,
    direciton: CharacterDirection,
) {
    let translation = transform.translation();
    let (x, y) = match direciton {
        CharacterDirection::Up => (translation.x, translation.y + 32.0),
        CharacterDirection::Down => (translation.x, translation.y -32.0),
        CharacterDirection::Left => (translation.x -32.0, translation.y),
        CharacterDirection::Right => (translation.x + 32.0, translation.y),
    };
    let mut damage_transform = Transform::default();
    damage_transform.set_translation_xyz(x, y, -y);
        let physics_body: PhysicsBody<f32> = PhysicsBodyBuilder::from(BodyStatus::Dynamic)
        .build();
    let physics_collider: PhysicsCollider<f32> =
        PhysicsColliderBuilder::from(Shape::Cuboid {
            half_extents: Vector3::new(16.0, 16.0, 300.0)
        })
        .sensor(true)
        .build();
    world
        .create_entity()
        .with(damage_transform)
        .with(DelayedRemove::new(0.2))
        .with(Destroyer { damage: strength })
        .with(physics_body)
        .with(physics_collider)
        .build();
}
