use amethyst::{
    prelude::*,
    ecs::world::EntityBuilder,
    core::transform::Transform,    
    renderer::{
        Transparent, SpriteRender,
    }
};
use crate::spriteanimationloader::SpriteAnimationStore;
use crate::physics::{
    BoundingRect,
    Physics,
    Solid,
};
use crate::spriteanimation::SpriteAnimation;
use crate::charactermeta::CharacterMeta;
use crate::charactermeta::CharacterDirection;
use crate::characteranimation::CharacterAnimation;
use crate::charactermove::CharacterMove;

pub fn create_character<'a>(
        entity_builder: EntityBuilder<'a>,
        animations: &SpriteAnimationStore,
        (x, y): (f32, f32),
        (left, right, bottom, top): (f32, f32, f32, f32),
        char_name: &str) -> EntityBuilder<'a> {
    println!("Create character start");
    let animation_up = format!("{}_walk_up", char_name);
    let animation_down = format!("{}_walk_down", char_name);
    let animation_left = format!("{}_walk_left", char_name);
    let animation_right = format!("{}_walk_right", char_name);

    let mut sprite_animation = SpriteAnimation::new(
        animations.animations.get(&animation_up).map(|x| x.clone()).unwrap_or(vec![0]), 0.1);
    sprite_animation.pause = true;
    let character_meta = CharacterMeta::new(
        CharacterDirection::Down,
    );
    let character_animation = CharacterAnimation {
        prev_character_meta: character_meta.clone(),
        walk_up_animation: animations.animations.get(&animation_up).map(|x| x.clone()).unwrap_or(vec![0]),
        walk_down_animation: animations.animations.get(&animation_down).map(|x| x.clone()).unwrap_or(vec![0]),
        walk_left_animation: animations.animations.get(&animation_left).map(|x| x.clone()).unwrap_or(vec![0]),
        walk_right_animation: animations.animations.get(&animation_right).map(|x| x.clone()).unwrap_or(vec![0]),
    };
    let sprite_render = SpriteRender {
        sprite_sheet: animations.sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_xyz(x, y, -y);

    println!("Create character end");

    entity_builder
        .with(sprite_render)
        .with(transform)
        .with(sprite_animation)
        .with(Transparent)
        .with(CharacterMove::new(128.0))
        .with(character_meta)
        .with(character_animation)
        .with(Physics::new())
        .with(BoundingRect::new(left, right, bottom, top))
}

pub fn create_solid<'a>(
        entity_builder: EntityBuilder<'a>,
        animations: &SpriteAnimationStore,
        (x, y): (f32, f32),
        (left, right, bottom, top): (f32, f32, f32, f32),
        name: &str) -> EntityBuilder<'a> {
    let sprite_render = SpriteRender {
        sprite_sheet: animations.sprite_sheet_handle.clone(),
        sprite_number: *animations.images.get(name).unwrap_or(&0),
    };
    let mut transform = Transform::default();
    transform.set_xyz(x, y, -y);

    entity_builder
        .with(sprite_render)
        .with(transform)
        .with(BoundingRect::new(left, right, bottom, top))
        .with(Transparent)
        .with(Solid)
}