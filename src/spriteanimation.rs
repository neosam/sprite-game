use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

#[derive(Default)]
pub struct SpriteAnimation {
    pub index: u32,
    pub keys: Vec<usize>,
    pub speed: f32,
    pub time: f32
}

impl SpriteAnimation {
    pub fn new(keys: Vec<usize>, speed: f32) -> Self {
        SpriteAnimation {
            index: 0,
            keys,
            speed,
            time: 0.0
        }
    }
}

impl Component for SpriteAnimation {
    type Storage = DenseVecStorage<Self>;
}


pub struct SpriteAnimationSystem;

impl<'s> System<'s> for SpriteAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SpriteAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut sprite_animations, time): Self::SystemData) {
        for (mut sprite_render, mut sprite_animation) in (&mut sprite_renders, &mut sprite_animations).join() {
            sprite_animation.time += time.delta_seconds();
            while sprite_animation.time > sprite_animation.speed {
                sprite_animation.index = (sprite_animation.index + 1) % (sprite_animation.keys.len() as u32);
                sprite_render.sprite_number = sprite_animation.keys[sprite_animation.index as usize];
                sprite_animation.time -= sprite_animation.speed;
            }
        }
    }
}