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
mod helper;

struct Example;

pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 480.0;



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
    let sprite_animations = spriteanimationloader::load_sprites(world, "texture", "tp-export.ron");

    helper::create_character(
            world.create_entity(),
            &sprite_animations,
            (ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0),
            (-16.0, 16.0, -16.0, 16.0),
            "healer")
        .build();

    // Add a brick
    helper::create_solid(
            world.create_entity(), 
            &sprite_animations, 
            (100.0, 100.0),
            (-16.0, 16.0, -16.0, 16.0),
            "brick")
        .build();
    generate_surrounding_walls(world, &sprite_animations);
}

fn generate_surrounding_walls(world: &mut World, animations: &spriteanimationloader::SpriteAnimationStore) {
    let tiles_x = ARENA_WIDTH as i32 / 32;
    let tiles_y = ARENA_HEIGHT as i32 / 32;
    let size = (-16.0, 16.0, -16.0, 16.0);

    for x in 0..tiles_x {
        helper::create_solid(
            world.create_entity(),
            &animations,
            (x as f32 * 32.0, 0.0),
            size,
            "brick"
        ).build();
        helper::create_solid(
            world.create_entity(),
            &animations,
            (x as f32 * 32.0, tiles_y as f32 * 32.0),
            size,
            "brick"
        ).build();
    }

    for y in 0..tiles_y {
        helper::create_solid(
            world.create_entity(),
            &animations,
            (0.0, y as f32 * 32.0),
            size,
            "brick"
        ).build();
        helper::create_solid(
            world.create_entity(),
            &animations,
            (tiles_x as f32 * 32.0, y as f32 * 32.0),
            size,
            "brick"
        ).build();
    }
    helper::create_solid(
        world.create_entity(),
        &animations,
        (tiles_x as f32 * 32.0, tiles_y as f32 * 32.0),
        size,
        "brick"
    ).build();
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
