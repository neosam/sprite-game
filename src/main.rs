extern crate amethyst;
extern crate nalgebra as na;
extern crate regex;
extern crate serde;

use amethyst::{
    input::{InputBundle, StringBindings},
    core::transform::{Transform, TransformBundle},
    prelude::*,
    renderer::{
        Camera, Transparent, RenderToWindow, RenderFlat2D, RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
};

pub mod characteranimation;
pub mod charactermeta;
pub mod charactermove;
pub mod damage;
pub mod delayedremove;
pub mod helper;
pub mod physics;
pub mod spriteanimation;
pub mod spriteanimationloader;
pub mod swordattack;
pub mod room;

struct Example;

pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 480.0;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register::<SpriteRender>();
        //world.register::<Transparent>();

        initialise_camera(world);
        initialize_test_sprite(world);
    }
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH as f32 / 2.0, ARENA_HEIGHT as f32 / 2.0, 1000.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_test_sprite(world: &mut World) {
    let sprite_animations = spriteanimationloader::load_sprites(world, "texture", "tp-export.ron");

    // Generate a room
    let tiles_x = ARENA_WIDTH as usize / 32;
    let tiles_y = ARENA_HEIGHT as usize / 32;
    let mut room_generation = room::RoomGeneration::default();
    room_generation.width = tiles_x;
    room_generation.height = tiles_y;
    let room = room_generation.generate_room(&mut rand::thread_rng());

    for (x, y, field) in room.room_field_iterator() {
        let pixel_pos = (
            x as f32 * 32.0 + 16.0,
            y as f32 * 32.0 + 16.0,
        );
        let hitbox = (-14.0, 14.0, -14.0, 14.0);
        match field {
            room::RoomField::Nothing => {},
            room::RoomField::Wall => {
                // Add a brick
                helper::create_solid(
                    world.create_entity(),
                    &sprite_animations,
                    pixel_pos,
                    hitbox,
                    "brick",
                ).build();
            },
            room::RoomField::Stone => {
                // Add a stone
                helper::create_solid(
                    world.create_entity(),
                    &sprite_animations,
                    pixel_pos,
                    hitbox,
                    "stones",
                ).build();
            },
            room::RoomField::Bush => {
                // Add a bush
                helper::create_solid(
                    world.create_entity(),
                    &sprite_animations,
                    pixel_pos,
                    hitbox,
                    "bush",
                ).with(damage::Destroyable { health: 2.0 })
                .build();
            },
            room::RoomField::Player => {
                helper::create_character(
                    world.create_entity(),
                    &sprite_animations,
                    pixel_pos,
                    hitbox,
                    "healer",
                )
                .with(charactermove::UserMove)
                // .with(damage::Destroyer { damage: 1.0})
                .build();
            }
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    //let path = format!("{}/resources/display_config.ron", root_dir);
    let binding_path = app_root.join("resources/binding_config.ron");
    let display_config_path = app_root.join("resources/display_config.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

  
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(physics::PhysicsSystem, "physics", &[])
        .with(
            spriteanimation::SpriteAnimationSystem,
            "sprite_animation",
            &[],
        )
        .with(charactermove::CharacterMoveSystem, "character_move", &[])
        .with(
            characteranimation::CharacterAnimationSystem,
            "character_animation",
            &["sprite_animation", "character_move"],
        )
        .with(damage::DestroySystem, "destroy", &["physics"])
        .with(delayedremove::DelayedRemoveSystem, "delayed_remove", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
