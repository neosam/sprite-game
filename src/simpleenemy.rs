use amethyst::{
    prelude::*,
    core::{Transform},
    ecs::{Component, NullStorage, Join, ParJoin, Read, Write, ReadStorage, System, WriteStorage, Entities},
};
use crate::physics::Physics;
use crate::charactermove::CharacterMove;

#[derive(Default)]
pub struct SimpleEnemy;
impl Component for SimpleEnemy {
    type Storage = NullStorage<Self>;
}

pub struct SimpleEnemySystem;
impl<'s> System<'s> for SimpleEnemySystem {
    type SystemData = (
        ReadStorage<'s, SimpleEnemy>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Physics>,
        ReadStorage<'s, CharacterMove>,
    );

    fn run(&mut self, (simple_enemies, transforms, mut physics, charactermoves): Self::SystemData) {
        /* Identify character position */
        let mut character = None;
        for (transform, charactermove) in (&transforms, &charactermoves).join() {
            character = Some(transform);
        }
        
        /* Let the character walk. */
        for (mut )
    }
}
