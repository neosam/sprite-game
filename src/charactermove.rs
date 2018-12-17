use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::ecs::{Component, DenseVecStorage};


use crate::charactermeta::{CharacterMeta, CharacterDirection};
use crate::physics::Physics;

pub struct CharacterMove {
    pub speed: f32
}

impl CharacterMove {
    pub fn new(speed: f32) -> Self {
        CharacterMove { speed }
    }
}

impl Component for CharacterMove {
    type Storage = DenseVecStorage<Self>;
}

pub struct CharacterMoveSystem;

impl<'s> System<'s> for CharacterMoveSystem {
    type SystemData = (
        WriteStorage<'s, Physics>,
        WriteStorage<'s, CharacterMeta>,
        ReadStorage<'s, CharacterMove>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut physics, mut character_meta, character_moves, input): Self::SystemData) {
        for (physics, character_meta, character_move) in (&mut physics, &mut character_meta, &character_moves).join() {
            physics.velocity.x =
                input.axis_value("player_move_x").unwrap() as f32
                 * character_move.speed;
            physics.velocity.y =
                input.axis_value("player_move_y").unwrap() as f32
                 * character_move.speed;
                
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
        }
    }
}