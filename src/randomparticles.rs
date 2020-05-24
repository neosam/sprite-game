use amethyst::{
    core::Transform,
    core::timing::Time,
    ecs::{Entities, System, WriteStorage, Read, ReadExpect},
    renderer::SpriteRender,
};
use specs_physics::{PhysicsBody, PhysicsBodyBuilder, nphysics::object::BodyStatus};
use crate::spriteanimationloader::SpriteAnimationStore;
use crate::delayedremove::DelayedRemove;
use rand;
use rand::Rng;

pub struct SpawnParticleSystem {
    pub average_part_spawn: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub lifespan: f32,
}

impl<'s> System<'s> for SpawnParticleSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, PhysicsBody<f32>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, DelayedRemove>,
        ReadExpect<'s, SpriteAnimationStore>,
        Entities<'s>,
    );

    fn run(&mut self, (
                time, 
                mut physics_bodies, 
                mut transforms, 
                mut sprite_render, 
                mut delayed_removes,
                sprite_animation_store, 
                entities): Self::SystemData) {
        let delta = time.delta_seconds();
        let mut rng = rand::prelude::thread_rng();
        let random_number: f32 = rng.gen();
        let probability_to_spawn = delta / self.average_part_spawn;
        if random_number < probability_to_spawn {
            let entity = entities.create();

            let mut transform = Transform::default();
            let x_pos = rng.gen::<f32>() * (self.max_x - self.min_x) + self.min_x;
            let y_pos = rng.gen::<f32>() * (self.max_y - self.min_y) + self.min_y;
            transform.set_translation_xyz(x_pos, y_pos, 0.0);
            transforms.insert(entity, transform).unwrap();

            let physics_body: PhysicsBody<f32> = PhysicsBodyBuilder::from(BodyStatus::Dynamic)
                .lock_rotations(true)
                .build();
            physics_bodies.insert(entity, physics_body).unwrap();

            let sprite = sprite_animation_store.get_sprite_render("particle").unwrap();
            sprite_render.insert(entity, sprite).unwrap();

            let delayed_remove = DelayedRemove::new(self.lifespan);
            delayed_removes.insert(entity, delayed_remove).unwrap();
        }
    }
}