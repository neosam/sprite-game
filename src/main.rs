extern crate amethyst;
extern crate nalgebra as na;
extern crate serde;

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

    let sprite_animations = spriteanimationloader::load_sprites(world, "texture", "animations.ron");

    character::create_character(
            world.create_entity(),
            &sprite_animations,
            transform,
            physics::BoundingRect::new(-16.0, 16.0, -18.0, 0.0),
            "healer")
        .build();

    // Add a bush
    let ground_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/Ground0.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let ground_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/Ground0.ron", // Here we load the associated ron file
            SpriteSheetFormat,
            ground_texture_handle, // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        )
    };
    let ground_sprite_render = SpriteRender {
        sprite_sheet: ground_sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    let mut ground_transform = Transform::default();
    ground_transform.set_xyz(100.0, 100.0, -100.0);

    world.create_entity()
        .with(ground_sprite_render)
        .with(ground_transform)
        .with(physics::BoundingRect::new(-8.0, 8.0, -8.0, 8.0))
        .with(Transparent)
        .with(physics::Solid)
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
