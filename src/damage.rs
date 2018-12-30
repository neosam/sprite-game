//! Handle damages

use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Component, DenseVecStorage, LazyUpdate, Entities},
    core::Transform,
    renderer::SpriteRender,

};

use crate::physics::BoundingRect;


/// Destroys entities which are destroyable.
pub struct Destroyer;
impl Component for Destroyer {
    type Storage = DenseVecStorage<Self>;
}

/// Will be destroyed if collides with a Destroyer.
pub struct Destroyable;
impl Component for Destroyable {
    type Storage = DenseVecStorage<Self>;
}

// 
pub struct DestroySystem;
impl<'s> System<'s> for DestroySystem {
    type SystemData = (
        ReadStorage<'s, Destroyer>,
        ReadStorage<'s, Destroyable>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BoundingRect>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(&mut self, (destroyers, destroyables, transforms, bouding_rects, lazy_update, entities): Self::SystemData) {
        for (_, transform, bouding_rect) in (&destroyers, &transforms, &bouding_rects).join() {
            for (_, dest_transform, dest_bouding_rect, entity) in (&destroyables, &transforms, &bouding_rects, &entities).join() {
                let min_left = (transform.translation().x + bouding_rect.left).min(
                    dest_transform.translation().x + dest_bouding_rect.left
                );
                let min_bottom = (transform.translation().y + bouding_rect.bottom).min(
                    dest_transform.translation().y + dest_bouding_rect.bottom
                );
                let max_right = (transform.translation().x + bouding_rect.right).max(
                    dest_transform.translation().x + dest_bouding_rect.right
                );
                let max_top = (transform.translation().y + bouding_rect.top).max(
                    dest_transform.translation().y + dest_bouding_rect.top
                );
                let sum_width = bouding_rect.width() + dest_bouding_rect.width();
                let sum_height = bouding_rect.height() + dest_bouding_rect.height();
                if max_right - min_left < sum_width && max_top - min_bottom < sum_height {
                    println!("Removing");
                    lazy_update.remove::<SpriteRender>(entity);
                }
            }
        } 
    }
}
