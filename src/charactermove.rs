use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::core::timing::Time;
use amethyst::ecs::{Component, DenseVecStorage};

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
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CharacterMove>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, character_moves, input, time): Self::SystemData) {
        for (transform, character_move) in (&mut transforms, &character_moves).join() {
            transform.translate_x(
                input.axis_value("player_move_x").unwrap() as f32
                 * time.delta_seconds()
                 * character_move.speed);
            transform.translate_y(
                input.axis_value("player_move_y").unwrap() as f32
                 * time.delta_seconds()
                 * character_move.speed);
        }
    }
}