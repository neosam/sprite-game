//! Handle damages

use amethyst::{
    ecs::{ReadStorage, System, WriteStorage, Component, DenseVecStorage, Entities, Write,
    ReadExpect},
};
use specs_physics::events::{ProximityEvent, ProximityEvents};
use amethyst::core::shrev::{ReaderId};




/// Destroys entities which are destroyable.
pub struct Destroyer {
    pub damage: f32
}
impl Component for Destroyer {
    type Storage = DenseVecStorage<Self>;
}

/// Will be destroyed if collides with a Destroyer.
pub struct Destroyable {
    pub health: f32
}
impl Component for Destroyable {
    type Storage = DenseVecStorage<Self>;
}

// 
pub struct DestroySystem {
    reader: Option<ReaderId<ProximityEvent>>
}
impl Default for DestroySystem {
    fn default() -> Self {
        DestroySystem {
            reader: None
        }
    }
}
impl<'s> System<'s> for DestroySystem {
    type SystemData = (
        ReadStorage<'s, Destroyer>,
        WriteStorage<'s, Destroyable>,
        Entities<'s>,
        Write<'s, ProximityEvents>,
    );

    fn run(&mut self, (destroyers, mut destroyables, entities, mut channel): Self::SystemData) {         
        if let None = self.reader {
            self.reader = Some(channel.register_reader());
        }
        
        if let Some(reader) = &mut self.reader {
            for collision in channel.read(reader) {
                
                if let (Some(destroyable), Some(destroyer)) = (destroyables.get_mut(collision.collider1), destroyers.get(collision.collider2)) {
                    let collider = collision.collider1.clone();
                    destroyable.health -= destroyer.damage;
                    if destroyable.health < 0.0 {
                        if let Err(error) = entities.delete(collider) {
                            warn!("Couldn't remove entity {} with zero health: {}",
                                collider.id(), error);
                        }
                    }
                }
                if let (Some(destroyable), Some(destroyer)) = (destroyables.get_mut(collision.collider2), destroyers.get(collision.collider1)) {
                    info!("Damage Collision");
                    let collider = collision.collider2.clone();
                    destroyable.health -= destroyer.damage;
                    if destroyable.health < 0.0 {
                        if let Err(error) = entities.delete(collider) {
                            warn!("Couldn't remove entity {} with zero health: {}",
                                collider.id(), error);
                        }
                    }
                }
            }
        }
    }
}
