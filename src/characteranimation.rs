use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::core::timing::Time;
use amethyst::ecs::{Component, DenseVecStorage};

use crate::charactermeta::CharacterMeta;
use crate::charactermeta::CharacterDirection;
use crate::spriteanimation::SpriteAnimation;

pub struct CharacterAnimation {
    pub prev_direction: crate::charactermeta::CharacterDirection,
    pub walk_up_animation: Vec<usize>,
    pub walk_down_animation: Vec<usize>,
    pub walk_left_animation: Vec<usize>,
    pub walk_right_animation: Vec<usize>,
}

impl Component for CharacterAnimation {
    type Storage = DenseVecStorage<Self>;
}


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
            if character_animation.prev_direction != character_meta.direction {
                character_animation.prev_direction = character_meta.direction;
                let new_animation = match character_meta.direction {
                    CharacterDirection::Up => character_animation.walk_up_animation.clone(),
                    CharacterDirection::Down => character_animation.walk_down_animation.clone(),
                    CharacterDirection::Left => character_animation.walk_left_animation.clone(),
                    CharacterDirection::Right => character_animation.walk_right_animation.clone(),
                };
                sprite_animation.index = 0;
                sprite_animation.keys = new_animation;
            }
        }
    }
}
