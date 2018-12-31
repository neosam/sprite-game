//! ECS system to move sprites and respect collisions 

use amethyst::ecs::{Component, DenseVecStorage};
use na::{Vector2};
use amethyst::core::Transform;
use amethyst::ecs::{Join, ParJoin, Read, ReadStorage, System, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::ecs::prelude::ParallelIterator;

/// Component which controls the physics of an entity.
/// 
/// It holds the speed in units/second for the enitty.
pub struct Physics {
    pub velocity: Vector2<f32>
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            velocity: Vector2::new(0.0, 0.0)
        }
    }
}

/// Component which defines the dimension of an entity
/// 
/// The dimension of the entity is used for collision detection.
#[derive(Clone)]
pub struct BoundingRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}
impl Component for BoundingRect {
    type Storage = DenseVecStorage<Self>;
}
impl BoundingRect {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        BoundingRect {
            left, right, bottom, top
        }
    }
}

impl BoundingRect {
    /// Get the width of the rect.
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Get the height of the rect.
    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }
}

/// Component which defines that an entity is not movable
/// 
/// Items which have the Physics Component will collide with
/// Solid Components.
pub struct Solid;
impl Component for Solid {
    type Storage = DenseVecStorage<Self>;
}

/// Handles movement of Entities and respects collisions.
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Physics>,
        ReadStorage<'s, BoundingRect>,
        ReadStorage<'s, Solid>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut physics, bounding_rects, solids, mut transforms, time): Self::SystemData) {
        (&mut physics, &transforms, &bounding_rects).par_join().for_each(
                    |(physics, transform, bounding_rect)| {
                for (o_transform, o_bounding_rect, _) in (&transforms, &bounding_rects, &solids).join() {
                    let x = transform.translation().x + physics.velocity.x * time.delta_seconds();
                    let y = transform.translation().y + physics.velocity.y * time.delta_seconds();
                    let min_left_x = (x + bounding_rect.left).min(o_transform.translation().x + o_bounding_rect.left);
                    let min_bottom_y = (y + bounding_rect.bottom).min(o_transform.translation().y + o_bounding_rect.bottom);
                    let max_right_x = (x + bounding_rect.right).max(o_transform.translation().x + o_bounding_rect.right);
                    let max_top_y = (y + bounding_rect.top).max(o_transform.translation().y + o_bounding_rect.top);
                    let sum_width = (bounding_rect.right - bounding_rect.left) + (o_bounding_rect.right - o_bounding_rect.left);
                    let sum_height = (bounding_rect.top - bounding_rect.bottom) + (o_bounding_rect.top - o_bounding_rect.bottom);
                    if (max_right_x - min_left_x) < sum_width && (max_top_y - min_bottom_y) < sum_height {
                        physics.velocity.x = 0.0;
                        physics.velocity.y = 0.0;
                    }
                }
            }
        );
            
        for (physics, transform) in (&physics, &mut transforms).join() {
            transform.translate_x(
                physics.velocity.x
                * time.delta_seconds());
            transform.translate_y(
                physics.velocity.y
                * time.delta_seconds());
            transform.set_z(-transform.translation().y);
        }
    }
}