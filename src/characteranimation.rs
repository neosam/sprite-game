//! ECS to set required animations
//! 
//!  

use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::ecs::{Component, DenseVecStorage};

use crate::charactermeta::CharacterMeta;
use crate::charactermeta::CharacterDirection;
use crate::spriteanimation::SpriteAnimation;

/// Component which contains the sprite animations. 
pub struct CharacterAnimation {
    pub prev_character_meta: crate::charactermeta::CharacterMeta,
    pub walk_up_animation: Vec<usize>,
    pub walk_down_animation: Vec<usize>,
    pub walk_left_animation: Vec<usize>,
    pub walk_right_animation: Vec<usize>,
}

impl Component for CharacterAnimation {
    type Storage = DenseVecStorage<Self>;
}

/// System to set the animations based on the CharacterMeta 
pub struct CharacterAnimationSystem;
impl<'s> System<'s> for CharacterAnimationSystem {
    type SystemData = (
        WriteStorage<'s, CharacterAnimation>,
        ReadStorage<'s, CharacterMeta>,
        WriteStorage<'s, SpriteAnimation>,
    );

    fn run(&mut self, (mut character_animations, character_metas, mut sprite_animations): Self::SystemData) {
        for (mut character_animation, character_meta, mut sprite_animation) 
                in (&mut character_animations, &character_metas, &mut sprite_animations).join() {
            if character_animation.prev_character_meta != *character_meta {
                character_animation.prev_character_meta = character_meta.clone();
                let new_animation = match character_meta.direction {
                    CharacterDirection::Up => character_animation.walk_up_animation.clone(),
                    CharacterDirection::Down => character_animation.walk_down_animation.clone(),
                    CharacterDirection::Left => character_animation.walk_left_animation.clone(),
                    CharacterDirection::Right => character_animation.walk_right_animation.clone(),
                };
                sprite_animation.index = 0;
                sprite_animation.keys = new_animation;
                sprite_animation.pause = !character_meta.moving;
            }
        }
    }
}
