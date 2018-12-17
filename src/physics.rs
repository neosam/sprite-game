use amethyst::ecs::{Component, DenseVecStorage};
use na::{Vector2};
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::timing::Time;

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


pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'s, Physics>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (physics, mut transforms, time): Self::SystemData) {
        for (physics, transform) in (&physics, &mut transforms).join() {
            transform.translate_x(
                physics.velocity.x
                 * time.delta_seconds());
            transform.translate_y(
                physics.velocity.y
                 * time.delta_seconds());
        }
    }
}