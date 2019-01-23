//! Handle damages

use amethyst::{
    core::Transform,
    ecs::{
        Component, DenseVecStorage, Entities, Join, LazyUpdate, Read, ReadStorage, System,
        WriteStorage,
    },
};

use crate::physics::BoundingRect;

/// Destroys entities which are destroyable.
pub struct Destroyer {
    pub damage: f32,
}
impl Component for Destroyer {
    type Storage = DenseVecStorage<Self>;
}

/// Will be destroyed if collides with a Destroyer.
pub struct Destroyable {
    pub health: f32,
}
impl Component for Destroyable {
    type Storage = DenseVecStorage<Self>;
}

//
pub struct DestroySystem;
impl<'s> System<'s> for DestroySystem {
    type SystemData = (
        ReadStorage<'s, Destroyer>,
        WriteStorage<'s, Destroyable>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BoundingRect>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (destroyers, mut destroyables, transforms, bouding_rects, lazy_update, entities): Self::SystemData,
    ) {
        for (destroyer, transform, bouding_rect) in
            (&destroyers, &transforms, &bouding_rects).join()
        {
            for (mut destroyable, dest_transform, dest_bouding_rect, entity) in
                (&mut destroyables, &transforms, &bouding_rects, &entities).join()
            {
                let min_left = (transform.translation().x + bouding_rect.left)
                    .min(dest_transform.translation().x + dest_bouding_rect.left);
                let min_bottom = (transform.translation().y + bouding_rect.bottom)
                    .min(dest_transform.translation().y + dest_bouding_rect.bottom);
                let max_right = (transform.translation().x + bouding_rect.right)
                    .max(dest_transform.translation().x + dest_bouding_rect.right);
                let max_top = (transform.translation().y + bouding_rect.top)
                    .max(dest_transform.translation().y + dest_bouding_rect.top);
                let sum_width = bouding_rect.width() + dest_bouding_rect.width();
                let sum_height = bouding_rect.height() + dest_bouding_rect.height();
                if max_right - min_left < sum_width && max_top - min_bottom < sum_height {
                    destroyable.health -= destroyer.damage;
                    if destroyable.health < 0.0 {
                        println!("Removing");
                        lazy_update.exec_mut(move |world| {
                            if let Err(err) = world.delete_entity(entity) {
                                println!("Couldn't remove entity.  Error: {}", err);
                            }
                        });
                    }
                }
            }
        }
    }
}
