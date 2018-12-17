extern crate amethyst;

use amethyst::{
    core::transform::{Transform, TransformBundle},
    assets::{Loader, AssetStorage},
    prelude::*,
    renderer::{Camera, DisplayConfig, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage,
               Projection, Texture, TextureMetadata, PngFormat, SpriteSheet, SpriteSheetFormat,
               SpriteRender},
    utils::application_root_dir,
};

mod spriteanimation;

struct Example;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register::<SpriteRender>();

        initialise_camera(world);
        initialize_test_sprite(world);
    }
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);

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
    transform.set_xyz(10.0, 10.0, 0.0);

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/pong_spritesheet.ron", // Here we load the associated ron file
            SpriteSheetFormat,
            texture_handle, // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        )
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let indices = vec![0,1];
    let sprite_animation = spriteanimation::SpriteAnimation::new(indices, 0.5);

    world.create_entity()
        .with(sprite_render)
        .with(transform)
        .with(sprite_animation)
        .build();
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data =
        GameDataBuilder::default()
            .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
            .with_bundle(TransformBundle::new())?
            .with(spriteanimation::SpriteAnimationSystem, "sprite_animation", &[]);
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
