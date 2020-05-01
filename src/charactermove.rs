//! ECS to handle character movement and input from the user

use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage, LazyUpdate};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Write};
use amethyst::input::{InputHandler, StringBindings};
use specs_physics::PhysicsBody;
use specs_physics::nphysics::algebra::{Velocity3, Force3};

use crate::charactermeta::{CharacterDirection, CharacterMeta};
//use crate::swordattack::sword_attack;

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
        WriteStorage<'s, CharacterMeta>,
        WriteStorage<'s, PhysicsBody<f32>>,
        ReadStorage<'s, CharacterMove>,
        ReadStorage<'s, UserMove>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>
    );

    fn run(
        &mut self,
        (
            mut character_meta,
            mut physics_body,
            character_moves,
            user_moves,
            transforms,
            input,
            lazy_update,
        ): Self::SystemData,
    ) {
        for (character_meta, physics_body, character_move, _, transform) in (
            &mut character_meta,
            &mut physics_body,
            &character_moves,
            &user_moves,
            &transforms,
        )
            .join()
        {
            if input.axis_value("player_move_x").unwrap() > 0.0 {
                character_meta.direction = CharacterDirection::Right;
                character_meta.moving = true;
                physics_body.velocity = Velocity3::<f32>::linear(character_move.speed, 0.0, 0.0);
            } else if input.axis_value("player_move_x").unwrap() < 0.0 {
                character_meta.direction = CharacterDirection::Left;
                character_meta.moving = true;
                physics_body.velocity = Velocity3::<f32>::linear(-character_move.speed, 0.0, 0.0);
            } else if input.axis_value("player_move_y").unwrap() > 0.0 {
                character_meta.direction = CharacterDirection::Up;
                character_meta.moving = true;
                physics_body.velocity = Velocity3::linear(0.0, character_move.speed, 0.0);
            } else if input.axis_value("player_move_y").unwrap() < 0.0 {
                character_meta.direction = CharacterDirection::Down;
                character_meta.moving = true;
                physics_body.velocity = Velocity3::linear(0.0, -character_move.speed, 0.0);
            } else {
                character_meta.moving = false;
                physics_body.velocity = Velocity3::linear(0.0, 0.0, 0.0);
            }
            // if input.action_is_down("attack").unwrap() {
            //     let transform: Transform = transform.clone();
            //     let bounding_rect: BoundingRect = bounding_rect.clone();
            //     let direction: CharacterDirection = character_meta.direction.clone();
            //     lazy_update.exec_mut(move |world| {
            //         sword_attack(world, 1.0, transform, bounding_rect, direction);
            //     });
            // }
        }
    }
}
