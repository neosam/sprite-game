extern crate amethyst;

use amethyst::{
    core::transform::{Transform, TransformBundle},
    assets::{Loader, AssetStorage},
    prelude::*,
    renderer::{Camera, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
               Projection, Texture, TextureMetadata, PngFormat, SpriteSheet, SpriteSheetFormat,
               SpriteRender, Transparent, ColorMask, ALPHA},
    utils::application_root_dir,
    input::InputBundle,
};

mod spriteanimation;
mod charactermove;
mod charactermeta;
mod characteranimation;

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
    transform.set_xyz(250.0, 250.0, 0.0);

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/healer_f.png",
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
            "texture/healer_f.ron", // Here we load the associated ron file
            SpriteSheetFormat,
            texture_handle, // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        )
    };

    

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let walk_down = vec![0, 1, 2];
    let walk_up = vec![3, 4, 5];
    let walk_right = vec![6, 7, 8];
    let walk_left = vec![9, 10, 11];
    let mut sprite_animation = spriteanimation::SpriteAnimation::new(walk_down.clone(), 0.1);
    sprite_animation.pause = true;

    let character_meta = charactermeta::CharacterMeta::new(
        charactermeta::CharacterDirection::Down,
    );

    let character_animation = characteranimation::CharacterAnimation {
        prev_character_meta: character_meta.clone(),
        walk_up_animation: walk_up.clone(),
        walk_down_animation: walk_down.clone(),
        walk_left_animation: walk_left.clone(),
        walk_right_animation: walk_right.clone(),
    };

    
    world.create_entity()
        .with(sprite_render)
        .with(transform)
        .with(sprite_animation)
        .with(Transparent)
        .with(charactermove::CharacterMove::new(128.0))
        .with(character_meta)
        .with(character_animation)
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
    ground_transform.set_xyz(60.0, 60.0, 0.0);

    world.create_entity()
        .with(ground_sprite_render)
        .with(ground_transform)
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

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat2D::new()
                .with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data =
        GameDataBuilder::default()
            .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
            .with_bundle(TransformBundle::new())?
            .with_bundle(input_bundle)?
            .with(spriteanimation::SpriteAnimationSystem, "sprite_animation", &[])
            .with(charactermove::CharacterMoveSystem, "character_move", &[])
            .with(characteranimation::CharacterAnimationSystem, "character_animation", &["sprite_animation", "character_move"]);
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
