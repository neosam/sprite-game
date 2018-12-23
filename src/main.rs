extern crate amethyst;
extern crate nalgebra as na;
extern crate serde;
extern crate regex;

use amethyst::{
    core::transform::{Transform, TransformBundle},
    assets::{Loader, AssetStorage},
    prelude::*,
    renderer::{Camera, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, Sprite,
               Projection, Texture, TextureMetadata, PngFormat, SpriteSheet, SpriteSheetFormat,
               SpriteRender, Transparent, ColorMask, ALPHA, DepthMode, TargetBuilder},
    utils::application_root_dir,
    input::InputBundle,
    config::Config,
};

mod spriteanimation;
mod charactermove;
mod charactermeta;
mod characteranimation;
mod physics;
mod spriteanimationloader;
mod character;

struct Example;

pub const ARENA_WIDTH: f32 = 500.0;
pub const ARENA_HEIGHT: f32 = 500.0;



impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register::<SpriteRender>();
        world.register::<Transparent>();

        initialise_camera(world);
        initialize_test_sprite(world);
    }
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1000.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialize_test_sprite(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_animations = spriteanimationloader::load_sprites(world, "texture", "tp-export.ron");

    character::create_character(
            world.create_entity(),
            &sprite_animations,
            transform,
            physics::BoundingRect::new(-16.0, 16.0, -18.0, 18.0),
            "healer")
        .build();

    // Add a brick
    let mut ground_transform = Transform::default();
    ground_transform.set_xyz(100.0, 100.0, -100.0);

    character::create_solid(
            world.create_entity(), 
            &sprite_animations, 
            ground_transform,
            physics::BoundingRect::new(-8.0, 8.0, -8.0, 8.0),
            "brick")
        .build();
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let binding_path = format!(
        "{}/resources/binding_config.ron",
        application_root_dir()
    );



    let config = DisplayConfig::load(&path);





    //println!("Loaded animations: {:?}", animations);

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_target(
        TargetBuilder::new("asdf").with_depth_buf(true)
    ).with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat2D::new()
                .with_transparency(ColorMask::all(), ALPHA, Some(DepthMode::LessEqualTest)))
    );

    let game_data =
        GameDataBuilder::default()
            .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
            .with_bundle(TransformBundle::new())?
            .with_bundle(input_bundle)?
            .with(physics::PhysicsSystem, "physics", &[])
            .with(spriteanimation::SpriteAnimationSystem, "sprite_animation", &[])
            .with(charactermove::CharacterMoveSystem, "character_move", &[])
            .with(characteranimation::CharacterAnimationSystem, "character_animation", &["sprite_animation", "character_move"]);
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
