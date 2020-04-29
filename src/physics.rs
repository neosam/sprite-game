//! ECS system to move sprites and respect collisions

use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::core::shrev::{EventChannel, ReaderId};
use amethyst::ecs::{Component, VecStorage};
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, WriteStorage, Entities};
use amethyst::core::math::Vector3;
use na::Vector2;

/// Will be sent as an event when the physics engine detects a collision.
#[derive(Debug)]
pub enum Collision {
    Solid(u32, u32)
}

/// Component which controls the physics of an entity.
///
/// It holds the speed in units/second for the enitty.
pub struct Physics {
    pub velocity: Vector2<f32>,
}

impl Component for Physics {
    type Storage = VecStorage<Self>;
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            velocity: Vector2::new(0.0, 0.0),
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
    pub bottom: f32,
}
impl Component for BoundingRect {
    type Storage = VecStorage<Self>;
}
impl BoundingRect {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        BoundingRect {
            left,
            right,
            bottom,
            top,
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
    type Storage = VecStorage<Self>;
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
        Write<'s, EventChannel<Collision>>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (mut physics, bounding_rects, solids, mut transforms, time, mut channel, entities): Self::SystemData,
    ) {
        let mut position_corrections = Vec::new();
 
        for (physics, transform) in (&physics, &mut transforms).join() {
            transform.move_right(physics.velocity.x * time.delta_seconds());
            transform.move_up(physics.velocity.y * time.delta_seconds());
            transform.set_translation_z(-transform.translation().y);
        }
        
        (&mut physics, &transforms, &bounding_rects, &entities)
            .join()
            .for_each(|(physics, transform, bounding_rect, moving_entity)| {
                for (o_transform, o_bounding_rect, _, solid_entity) in
                    (&transforms, &bounding_rects, &solids, &entities).join()
                {
                    let x = transform.translation().x + physics.velocity.x * time.delta_seconds();
                    let y = transform.translation().y + physics.velocity.y * time.delta_seconds();
                    let min_left_x = (x + bounding_rect.left)
                        .min(o_transform.translation().x + o_bounding_rect.left);
                    let min_bottom_y = (y + bounding_rect.bottom)
                        .min(o_transform.translation().y + o_bounding_rect.bottom);
                    let max_right_x = (x + bounding_rect.right)
                        .max(o_transform.translation().x + o_bounding_rect.right);
                    let max_top_y = (y + bounding_rect.top)
                        .max(o_transform.translation().y + o_bounding_rect.top);
                    let sum_width = (bounding_rect.right - bounding_rect.left)
                        + (o_bounding_rect.right - o_bounding_rect.left);
                    let sum_height = (bounding_rect.top - bounding_rect.bottom)
                        + (o_bounding_rect.top - o_bounding_rect.bottom);
                    let intersect_x = sum_width - (max_right_x - min_left_x);
                    let intersect_y = sum_height - (max_top_y - min_bottom_y);
                    if intersect_x > 0.0 && intersect_y > 0.0 {
                        channel.single_write(Collision::Solid(moving_entity.id(), solid_entity.id()));
                        let correction_x = if physics.velocity.x > 0.0 {
                            -intersect_x
                        } else {
                            intersect_x
                        };
                        let correction_y = if physics.velocity.y > 0.0 {
                            -intersect_y
                        } else {
                            intersect_y
                        };
                        if intersect_x > intersect_y {
                            position_corrections.push((moving_entity, (0.0, correction_y)));
                        } else {
                            position_corrections.push((moving_entity, (correction_x, 0.0)));
                        }
                    }
                }
            });

        for (entity, (delta_x, delta_y)) in position_corrections {
            if let Some(transform) = transforms.get_mut(entity) {
                transform.append_translation(Vector3::new(delta_x, delta_y, 0.0));
            }
        }
    }
}

#[derive(Default)]
pub struct PhysicsStopOnCollidingSystem {
    reader: Option<ReaderId<Collision>>
}


impl<'s> System<'s> for PhysicsStopOnCollidingSystem {
    type SystemData = (
        WriteStorage<'s, Physics>,
        ReadStorage<'s, BoundingRect>,
        ReadStorage<'s, Solid>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Write<'s, EventChannel<Collision>>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (_physics, _bounding_rects, _solids, _transforms, _time, mut channel, _entities): Self::SystemData,
    ) {
        if let None = self.reader {
            self.reader = Some(channel.register_reader());
        }
        if let Some(reader) = &mut self.reader {
            for collision in channel.read(reader) {
                println!("Collision detected: {:?}", collision);
            }
        }
    }
}