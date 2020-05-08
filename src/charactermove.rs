//! ECS to handle character movement and input from the user

use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage, LazyUpdate};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, ReadExpect};
use amethyst::input::{InputHandler, StringBindings};
use specs_physics::PhysicsBody;
use specs_physics::nphysics::algebra::Velocity3;

use crate::charactermeta::{CharacterDirection, CharacterMeta};
use crate::swordattack::sword_attack;
use crate::spriteanimationloader::SpriteAnimationStore;

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
pub struct CharacterMoveSystem {
    attack_released: bool
}
impl Default for CharacterMoveSystem {
    fn default() -> Self {
        CharacterMoveSystem {
            attack_released: true
        }
    }
}

impl<'s> System<'s> for CharacterMoveSystem {
    type SystemData = (
        WriteStorage<'s, CharacterMeta>,
        WriteStorage<'s, PhysicsBody<f32>>,
        ReadStorage<'s, CharacterMove>,
        ReadStorage<'s, UserMove>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, SpriteAnimationStore>,
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
            sprite_animation_store,
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
            let mut velocity_x = 0.0;
            let mut velocity_y = 0.0;
            let mut movement = false;
            if input.axis_value("player_move_x").unwrap() > 0.0 {
                character_meta.direction = CharacterDirection::Right;
                character_meta.moving = true;
                velocity_x += character_move.speed;
                movement = true;
            }
            if input.axis_value("player_move_x").unwrap() < 0.0 {
                character_meta.direction = CharacterDirection::Left;
                character_meta.moving = true;
                velocity_x -= character_move.speed;
                movement = true;
            }
            if input.axis_value("player_move_y").unwrap() > 0.0 {
                character_meta.direction = CharacterDirection::Up;
                character_meta.moving = true;
                velocity_y += character_move.speed;
                movement = true;
            }
            if input.axis_value("player_move_y").unwrap() < 0.0 {
                character_meta.direction = CharacterDirection::Down;
                character_meta.moving = true;
                velocity_y -= character_move.speed;
                movement = true;
            } 
            if !movement {
                character_meta.moving = false;
            }
            physics_body.velocity = Velocity3::linear(velocity_x, velocity_y, 0.0);
            if input.action_is_down("attack").unwrap() {
                if self.attack_released {
                    self.attack_released = false;
                    let transform: Transform = transform.clone();
                    let direction: CharacterDirection = character_meta.direction.clone();
                    let sprite_name = format!("sword-attack-{}", direction.as_str());
                    let sprite = sprite_animation_store.get_sprite_render(&sprite_name).unwrap();
                    lazy_update.exec_mut(move |world| {
                        sword_attack(world, 1.0, transform, direction, sprite);
                    });
                }
            } else {
                self.attack_released = true;
            }
        }
    }
}
