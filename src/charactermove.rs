//! ECS to handle character movement and input from the user

use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage, LazyUpdate};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::charactermeta::{CharacterDirection, CharacterMeta};
use crate::physics::{BoundingRect, Physics};
use crate::swordattack::sword_attack;

/// Ability to let the character move.
pub struct CharacterMove {
    pub speed: f32,
}

impl CharacterMove {
    /// Create a new CharacterMove which contains the given speed.
    pub fn new(speed: f32) -> Self {
        CharacterMove { speed }
    }
}

impl Component for CharacterMove {
    type Storage = DenseVecStorage<Self>;
}

/// Component which lets the user control the entity.
pub struct UserMove;
impl Component for UserMove {
    type Storage = DenseVecStorage<Self>;
}

/// System to handle user input and set the speed.
pub struct CharacterMoveSystem;

impl<'s> System<'s> for CharacterMoveSystem {
    type SystemData = (
        WriteStorage<'s, Physics>,
        WriteStorage<'s, CharacterMeta>,
        ReadStorage<'s, CharacterMove>,
        ReadStorage<'s, UserMove>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BoundingRect>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            mut physics,
            mut character_meta,
            character_moves,
            user_moves,
            transforms,
            bounding_rects,
            input,
            lazy_update,
        ): Self::SystemData,
    ) {
        for (physics, character_meta, character_move, _, transform, bounding_rect) in (
            &mut physics,
            &mut character_meta,
            &character_moves,
            &user_moves,
            &transforms,
            &bounding_rects,
        )
            .join()
        {
            physics.velocity.x =
                input.axis_value("player_move_x").unwrap() as f32 * character_move.speed;
            physics.velocity.y =
                input.axis_value("player_move_y").unwrap() as f32 * character_move.speed;

            if input.axis_value("player_move_x").unwrap() > 0.0 {
                character_meta.direction = CharacterDirection::Right;
                character_meta.moving = true;
            } else if input.axis_value("player_move_x").unwrap() < 0.0 {
                character_meta.direction = CharacterDirection::Left;
                character_meta.moving = true;
            } else if input.axis_value("player_move_y").unwrap() > 0.0 {
                character_meta.direction = CharacterDirection::Up;
                character_meta.moving = true;
            } else if input.axis_value("player_move_y").unwrap() < 0.0 {
                character_meta.direction = CharacterDirection::Down;
                character_meta.moving = true;
            } else {
                character_meta.moving = false;
            }
            if input.action_is_down("attack").unwrap() {
                let transform: Transform = transform.clone();
                let bounding_rect: BoundingRect = bounding_rect.clone();
                let direction: CharacterDirection = character_meta.direction.clone();
                lazy_update.exec_mut(move |world| {
                    sword_attack(world, 1.0, transform, bounding_rect, direction);
                });
            }
        }
    }
}
