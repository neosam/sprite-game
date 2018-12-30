use amethyst::{
    prelude::*,
    ecs::world::World,
    core::Transform,
};
use crate::{
    physics::BoundingRect,
    delayedremove::DelayedRemove,
    charactermeta::{
        CharacterDirection,
    },
    damage::Destroyer,
};

pub fn sword_attack(world: &mut World,
                    strength: f32,
                    transform: Transform,
                    bounding_rect: BoundingRect,
                    direciton: CharacterDirection) {
    let translation = transform.translation();
    let (x, y) = match direciton {
        CharacterDirection::Up => (translation.x, translation.y + bounding_rect.top),
        CharacterDirection::Down => (translation.x, translation.y + bounding_rect.bottom),
        CharacterDirection::Left => (translation.x + bounding_rect.left, translation.y),
        CharacterDirection::Right => (translation.x + bounding_rect.right, translation.y)
    };
    let mut damage_transform = Transform::default();
    damage_transform.set_xyz(x, y, -y);
    world.create_entity()
        .with(damage_transform)
        .with(DelayedRemove::new(0.2))
        .with(bounding_rect.clone())
        .with(Destroyer { damage: strength })
        .build();
}