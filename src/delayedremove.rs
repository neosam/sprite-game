//! Performs a sword attack

use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Component, DenseVecStorage, LazyUpdate, Entities},
    core::{
        timing::Time,
    },
};

pub struct DelayedRemove {
    pub current: f32,
    pub end: f32
}
impl Component for DelayedRemove {
    type Storage = DenseVecStorage<Self>;
}

pub struct DelayedRemoveSystem;
impl<'s> System<'s> for DelayedRemoveSystem {
    type SystemData = (
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
        Entities<'s>,
        WriteStorage<'s, DelayedRemove>,
    );

    fn run(&mut self, (lazy_update, time, entities, mut delayed_removes): Self::SystemData) {
        for (delayed_remove, entity) in (&mut delayed_removes, &entities).join() {
            delayed_remove.current += time.delta_seconds();
            if delayed_remove.current > delayed_remove.end {
                lazy_update.exec_mut(move |world| {
                    if let Err(err) = world.delete_entity(entity) {
                        println!("Couldn't remove entity.  Error: {}", err);
                    }
                })
            }
        }
    }
}