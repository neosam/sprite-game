extern crate amethyst;
extern crate nalgebra as na;
extern crate regex;
extern crate serde;

use amethyst::{
    input::{InputBundle, StringBindings},
    core::transform::{Transform, TransformBundle},
    core::shrev::{EventChannel, ReaderId},
    ecs::{ReadStorage, WriteStorage},
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
pub mod map;
pub mod roomexit;

struct Example {
    map: map::Map,
    room_coordinate: map::Coordinate,
    spawn_player: Option<(i32, i32)>,
}

pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 480.0;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register::<SpriteRender>();
        //world.register::<Transparent>();

        initialise_camera(world);
        initialize_test_sprite(self, world);
    }

    fn update(&mut self, game_state: &mut StateData<GameData>) -> SimpleTrans {
        use crate::roomexit::PerformRoomExit;
        let mut trans = SimpleTrans::None;
        {
            let mut perform_room_exits = game_state.world.fetch_mut::<Option<roomexit::PerformRoomExit>>();
            if let Some(PerformRoomExit(dest_room, spawn_coordinates)) = &*perform_room_exits {
                let room_coordinate = dest_room.to_absolute_coordinates(self.room_coordinate);
                println!("New coordinate: {:?}", room_coordinate);
                let new_state = Example {
                    map: self.map.clone(),
                    room_coordinate,
                    spawn_player: Some(*spawn_coordinates),
                };
                *perform_room_exits = None;
                trans = SimpleTrans::Push(Box::new(new_state))
            }
        }
        if let SimpleTrans::None = trans {
            trans
        } else {
            game_state.world.delete_all();
            trans
        }
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

fn initialize_test_sprite(scene: &Example, world: &mut World) {
    let sprite_animations = spriteanimationloader::load_sprites(world, "texture", "tp-export.ron");

    // Generate a room
    let tiles_x = ARENA_WIDTH as usize / 32;
    let tiles_y = ARENA_HEIGHT as usize / 32;
    println!("Getting room: {:?}", scene.room_coordinate);
    let room = scene.map.get_room((scene.room_coordinate)).unwrap();
    let hitbox = (-14.0, 14.0, -14.0, 14.0);

    for (x, y, field) in room.room_field_iterator() {
        let pixel_pos = (
            x as f32 * 32.0 + 16.0,
            y as f32 * 32.0 + 16.0,
        );
        
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
                if let None = scene.spawn_player {
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
            },
            room::RoomField::Exit(direction) => {
                helper::create_walkable_solid(
                    world.create_entity(),
                    pixel_pos,
                    hitbox,
                )
                .with(direction)
                // .with(damage::Destroyer { damage: 1.0})
                .build();
            },
        }
    }
    if let Some(player_coordinate) = scene.spawn_player {
        let pixel_pos = (
            player_coordinate.0 as f32 * 32.0 + 16.0,
            player_coordinate.1 as f32 * 32.0 + 16.0,
        );
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
        .with(roomexit::RoomExitSystem::default(), "roomexit", &["physics"])
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

    let tiles_x = ARENA_WIDTH as usize / 32;
    let tiles_y = ARENA_HEIGHT as usize / 32;
    let scene = Example {
        map: build_map(tiles_x, tiles_y),
        room_coordinate: (0, 0),
        spawn_player: None,
    };

    let mut game = Application::new("./", scene, game_data)?;

    game.run();

    Ok(())
}

fn build_map(width: usize, height: usize) -> map::Map {
    let mut map = map::Map::new();

    let mut room_generation1 = room::RoomGeneration::default();
    room_generation1.width = width;
    room_generation1.height = height;
    room_generation1.exit_east = true;
    let room1 = room_generation1.generate_room(&mut rand::thread_rng());

    let mut room_generation2 = room::RoomGeneration::default();
    room_generation2.width = width;
    room_generation2.height = height;
    room_generation2.exit_west = true;
    let room2 = room_generation2.generate_room(&mut rand::thread_rng());

    map.add_room((0, 0), room1);
    map.add_room((1, 0), room2);

    map
}
