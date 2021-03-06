//! Contains helper functions

use crate::characteranimation::CharacterAnimation;
use crate::charactermeta::CharacterDirection;
use crate::charactermeta::CharacterMeta;
use crate::charactermove::CharacterMove;
use crate::spriteanimation::SpriteAnimation;
use crate::spriteanimationloader::SpriteAnimationStore;
use crate::forces::RadialForceField;
use amethyst::{
    prelude::*,
    core::transform::Transform,
    ecs::world::EntityBuilder,
    renderer::{SpriteRender},
};
use specs_physics::{PhysicsBodyBuilder, PhysicsBody,
    nphysics::object::BodyStatus,
    nalgebra::{Vector3},
    PhysicsColliderBuilder,
    PhysicsCollider,
    colliders::Shape,
};
/// Assembles a character on the map
///
/// Assigns the components to the EntityBuilder which are required
/// to have a moving character on the screen.
///
/// For the animations, it requires to have animation names following
/// this pattern:
/// * (name)_walk_up
/// * (name)_walk_down
/// * (name)_walk_left
/// * (name)_walk_right
///
/// ## Examples
/// ```
/// use helper::create_character;
///
/// create_character(
///         world.create_entity(),
///         &animations,
///         (300.0, 300.0),
///         (-16.0, 16.0, -16.0, 16.0),
///         "hero"
/// ).build();
/// ```
pub fn create_character<'a>(
    entity_builder: EntityBuilder<'a>,
    animations: &SpriteAnimationStore,
    (x, y): (f32, f32),
    char_name: &str,
) -> EntityBuilder<'a> {
    println!("Create character start");
    let animation_up = format!("{}_walk_up", char_name);
    let animation_down = format!("{}_walk_down", char_name);
    let animation_left = format!("{}_walk_left", char_name);
    let animation_right = format!("{}_walk_right", char_name);

    let mut sprite_animation = SpriteAnimation::new(
        animations
            .animations
            .get(&animation_up)
            .map(|x| x.clone())
            .unwrap_or(vec![0]),
        0.1,
    );
    sprite_animation.pause = true;
    let character_meta = CharacterMeta::new(CharacterDirection::Down);
    let character_animation = CharacterAnimation {
        prev_character_meta: character_meta.clone(),
        walk_up_animation: animations
            .animations
            .get(&animation_up)
            .map(|x| x.clone())
            .unwrap_or(vec![0]),
        walk_down_animation: animations
            .animations
            .get(&animation_down)
            .map(|x| x.clone())
            .unwrap_or(vec![0]),
        walk_left_animation: animations
            .animations
            .get(&animation_left)
            .map(|x| x.clone())
            .unwrap_or(vec![0]),
        walk_right_animation: animations
            .animations
            .get(&animation_right)
            .map(|x| x.clone())
            .unwrap_or(vec![0]),
    };
    let sprite_render = SpriteRender {
        sprite_sheet: animations.sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, -y);

    let physics_body: PhysicsBody<f32> = PhysicsBodyBuilder::from(BodyStatus::Dynamic)
        .lock_rotations(true)
        .build();
    let physics_collider: PhysicsCollider<f32> =
        PhysicsColliderBuilder::from(Shape::Cuboid {
            half_extents: Vector3::new(13.0, 13.0, 300.0)
        })
        .angular_prediction(0.0)
        .build();

    println!("Create character end");

    entity_builder
        .with(sprite_render)
        .with(transform)
        .with(sprite_animation)
        //.with(Transparent)
        .with(CharacterMove::new(128.0))
        .with(character_meta)
        .with(character_animation)
    //    .with(Physics::new())
    //    .with(BoundingRect::new(left, right, bottom, top))
        .with(physics_body)
        .with(physics_collider)
        .with(RadialForceField::new(20000000.0))
    
}

/// Assebles a solid entity
///
/// Assigns the components to the EntityBuilder which are required
/// to have a solid enity.
///
/// The name must match the sprite name in.
///
/// ## Examples
/// ```
/// use helper::create_solid;
///
/// create_solid(
///         world.create_entity(),
///         &animations,
///         (300.0, 300.0),
///         (-16.0, 16.0, -16.0, 16.0),
///         "hero"
/// ).build();
/// ```
pub fn create_solid<'a>(
    entity_builder: EntityBuilder<'a>,
    animations: &SpriteAnimationStore,
    (x, y): (f32, f32),
    name: &str,
) -> EntityBuilder<'a> {
    let sprite_render = animations.get_sprite_render(name).unwrap();
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, -y);
    let physics_body: PhysicsBody<f32> = PhysicsBodyBuilder::from(BodyStatus::Static)
        .build();
    let physics_collider: PhysicsCollider<f32> =
        PhysicsColliderBuilder::from(Shape::Cuboid {
            half_extents: Vector3::new(16.0, 16.0, 300.0)
        })
        .build();

    entity_builder
        .with(sprite_render)
        .with(transform)
        .with(physics_body)
        .with(physics_collider)
    //    .with(BoundingRect::new(left, right, bottom, top))
     //   .with(Transparent)
     //   .with(Solid)
}

pub fn create_walkable_solid<'a>(
    entity_builder: EntityBuilder<'a>,
    (x, y): (f32, f32),
) -> EntityBuilder<'a> {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, -y);
    let physics_body: PhysicsBody<f32> = PhysicsBodyBuilder::from(BodyStatus::Static)
        .build();
    let physics_collider: PhysicsCollider<f32> =
        PhysicsColliderBuilder::from(Shape::Cuboid {
            half_extents: Vector3::new(16.0, 16.0, 300.0)
        })
        .sensor(true)
        .build();

    entity_builder
        .with(transform)
        .with(physics_body)
        .with(physics_collider)
     //   .with(BoundingRect::new(left, right, bottom, top))
     //   .with(Transparent)
     //   .with(Solid)
}

/// Assembles a walkable entity
///
/// Assigns the components to the EntityBuilder which are required
/// to have a solid entity.
///
/// The name must match the sprite name in.
///
/// ## Examples
/// ```
/// use helper::create_solid;
///
/// create_solid(
///         world.create_entity(),
///         &animations,
///         (300.0, 300.0),
///         (-16.0, 16.0, -16.0, 16.0),
///         "hero"
/// ).build();
/// ```
pub fn create_walkable<'a>(
    entity_builder: EntityBuilder<'a>,
    animations: &SpriteAnimationStore,
    (x, y): (f32, f32),
    name: &str,
) -> EntityBuilder<'a> {
    let sprite_render = SpriteRender {
        sprite_sheet: animations.sprite_sheet_handle.clone(),
        sprite_number: *animations.images.get(name).unwrap_or(&0),
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, -y);

    entity_builder
        .with(sprite_render)
        .with(transform)
    //    .with(BoundingRect::new(left, right, bottom, top))
    //    .with(Transparent)
}
