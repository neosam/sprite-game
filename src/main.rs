extern crate amethyst;
extern crate nalgebra as na;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate log;
extern crate specs_physics;

use amethyst::{
    input::{InputBundle, StringBindings},
    core::transform::{Transform, TransformBundle},
    prelude::*,
    renderer::{
        Camera, RenderToWindow, RenderFlat2D, RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
    ecs,
    core::bundle::SystemBundle,
    core::ArcThreadPool,
};
use specs_physics:: {
    systems::*,
};



pub mod characteranimation;
pub mod charactermeta;
pub mod charactermove;
pub mod damage;
pub mod delayedremove;
pub mod helper;
pub mod spriteanimation;
pub mod spriteanimationloader;
pub mod swordattack;
pub mod room;
pub mod map;
pub mod roomexit;
pub mod forces;
pub mod randomparticles;
// pub mod simpleenemy;

struct Example<'a, 'b> {
    map: map::Map<room::Room>,
    room_coordinate: map::Coordinate,
    spawn_player: Option<(i32, i32)>,

    dispatcher: Option<ecs::Dispatcher<'a, 'b>>,
}

pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 480.0;

impl<'a, 'b> SimpleState for Example<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register::<SpriteRender>();
        //world.register::<Transparent>();

        let app_root = application_root_dir().unwrap();
        //let path = format!("{}/resources/display_config.ron", root_dir);
        let binding_path = app_root.join("resources/binding_config.ron");
        let display_config_path = app_root.join("resources/display_config.ron");
        let input_bundle =
            InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path).unwrap();

        let mut dispatcher_builder = ecs::DispatcherBuilder::new();
        input_bundle.build(world, &mut dispatcher_builder).unwrap();
        //.with_bundle(input_bundle)?
        //.with(physics::PhysicsSystem, "physics", &[])
        let mut dispatcher_builder =
                dispatcher_builder.with(delayedremove::DelayedRemoveSystem, "delayed_remove", &[])
            .with(
                spriteanimation::SpriteAnimationSystem,
                "sprite_animation",
                &[],
            )
            .with(randomparticles::SpawnParticleSystem {
                average_part_spawn: 0.01,
                min_x: 0.0,
                max_x: 640.0,
                min_y: 0.0,
                max_y: 480.0,
                lifespan: 5.0,
            }, "spawn_particle_system", &[])
            .with(forces::ForceSystem, "force_system", &[])
            .with(charactermove::CharacterMoveSystem::default(), "character_move", &[])
            .with(
                characteranimation::CharacterAnimationSystem,
                "character_animation",
                &["sprite_animation", "character_move"],
            )
            
            .with(SyncBodiesToPhysicsSystem::<f32, Transform>::default(),
                "sync_bodies_to_physics_system",
                &["character_move", "delayed_remove"],
            )
            .with(SyncCollidersToPhysicsSystem::<f32, Transform>::default(),
                "sync_colliders_to_physics_system",
                &["sync_bodies_to_physics_system"],
            )
            .with(SyncParametersToPhysicsSystem::<f32>::default(),
                "sync_gravity_to_physics_system",
                &[],
            )
            .with(PhysicsStepperSystem::<f32>::default(),
                "physics_stepper_system",
                &[
                    "sync_bodies_to_physics_system",
                    "sync_colliders_to_physics_system",
                    "sync_gravity_to_physics_system",
                ],
            )
            .with(SyncBodiesFromPhysicsSystem::<f32, Transform>::default(),
                "sync_bodies_from_physics_system",
                &["physics_stepper_system"],
            )
            .with(roomexit::RoomExitSystem::new(world), "roomexit", &["sync_bodies_from_physics_system"])
            .with(damage::DestroySystem::default(), "destroy", &["sync_bodies_from_physics_system"]);
        RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path).unwrap()
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .build(world, &mut dispatcher_builder).unwrap();
        
        let mut dispatcher = dispatcher_builder
                .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
                .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        info!("Initialize camera");
        initialise_camera(world);
        info!("Initialize sprites");
        initialize_test_sprite(self, world);
    }

    fn update(&mut self, game_state: &mut StateData<GameData>) -> SimpleTrans {
        use crate::roomexit::PerformRoomExit;

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&game_state.world);
        }


        let reset_all = {
            let mut perform_room_exits = game_state.world.fetch_mut::<Option<roomexit::PerformRoomExit>>();
            if let Some(PerformRoomExit(dest_room, spawn_coordinates)) = &*perform_room_exits {
                let room_coordinate = dest_room.to_absolute_coordinates(self.room_coordinate);
                println!("New coordinate: {:?}", room_coordinate);
                self.room_coordinate = room_coordinate;
                self.spawn_player = Some(*spawn_coordinates);
                *perform_room_exits = None;
                
                true
            } else {
                false
            }
        };
        if reset_all {
            game_state.world.delete_all();
            initialise_camera(game_state.world);
            initialize_test_sprite(self, game_state.world);
        }
        SimpleTrans::None
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
    info!("Loading sprites");
    let sprite_animations = spriteanimationloader::load_sprites(world, "texture", "tp-export.ron");

    // Generate a room
    println!("Getting room: {:?}", scene.room_coordinate);
    let room = scene.map.get_room(scene.room_coordinate).unwrap();

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
                    "brick",
                ).build();
            },
            room::RoomField::Stone => {
                // Add a stone
                helper::create_solid(
                    world.create_entity(),
                    &sprite_animations,
                    pixel_pos,
                    "stones",
                ).build();
            },
            room::RoomField::Bush => {
                // Add a bush
                helper::create_solid(
                    world.create_entity(),
                    &sprite_animations,
                    pixel_pos,
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
                )
                .with(direction)
                // .with(damage::Destroyer { damage: 1.0})
                .build();
            },
        }
    }
    if let Some(player_coordinate) = scene.spawn_player {
        info!("Setting player coordinates");
        let pixel_pos = (
            player_coordinate.0 as f32 * 32.0 + 16.0,
            player_coordinate.1 as f32 * 32.0 + 16.0,
        );
        helper::create_character(
            world.create_entity(),
            &sprite_animations,
            pixel_pos,
            "healer",
        )
        .with(charactermove::UserMove)
        // .with(damage::Destroyer { damage: 1.0})
        .build();
    }
    world.insert(sprite_animations);
    info!("Room setup complete");
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    info!("starting up");
  
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?;


    info!("Generate map");
    let tiles_x = ARENA_WIDTH as usize / 32;
    let tiles_y = ARENA_HEIGHT as usize / 32;
    let scene = Example {
        map: build_map(tiles_x, tiles_y),
        room_coordinate: (0, 0),
        spawn_player: None,
        dispatcher: None,
    };

    info!("Create game");
    let mut game = Application::new("./", scene, game_data)?;

    info!("Run game");
    game.run();

    Ok(())
}

fn build_map(width: usize, height: usize) -> map::Map<room::Room> {
    /*let mut map = map::Map::new();

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
    map.add_room((1, 0), room2);*/
    let mut map_gen = map::DungeonGen::default();
    map_gen.corridor_length = 5;
    map_gen.splits = 4;
    let map = map_gen.generate(&mut rand::thread_rng(), width, height).generate_map(&mut rand::thread_rng());

    map
}
