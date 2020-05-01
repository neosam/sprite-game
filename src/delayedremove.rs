//! Performs a sword attack

use amethyst::{
    core::timing::Time,
    ecs::{Component, DenseVecStorage, Entities, Join, System, WriteStorage, Read},
};

pub struct DelayedRemove {
    pub current: f32,
    pub end: f32,
}
impl Component for DelayedRemove {
    type Storage = DenseVecStorage<Self>;
}
impl DelayedRemove {
    pub fn new(end: f32) -> Self {
        DelayedRemove { current: 0.0, end }
    }
}

pub struct DelayedRemoveSystem;
impl<'s> System<'s> for DelayedRemoveSystem {
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        WriteStorage<'s, DelayedRemove>,
    );

    fn run(&mut self, (time, entities, mut delayed_removes): Self::SystemData) {
        for (delayed_remove, entity) in (&mut delayed_removes, &entities).join() {
            delayed_remove.current += time.delta_seconds();
            if delayed_remove.current > delayed_remove.end {
                info!("Delayed remove of {}", entity.id());
                if let Err(error) = entities.delete(entity) {
                    warn!("Delayed remove of {} failed: {}", entity.id(), error);
                }
            }
        }
    }
}
