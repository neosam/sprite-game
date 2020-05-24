use amethyst::core::shrev::{ReaderId};
use amethyst::ecs::{Write, ReadStorage, System, Read};
use amethyst::ecs::{Component, VecStorage};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::ecs::SystemData;


use specs_physics::events::{ProximityEvent, ProximityEvents};
use crate::room::DestRoom;
use crate::room;

impl Component for DestRoom {
    type Storage = VecStorage<Self>;
}

pub struct PerformRoomExit(pub room::DestRoom, pub (i32, i32));

pub struct RoomExitSystem {
    reader: ReaderId<ProximityEvent>
}

/*impl Default for RoomExitSystem {
    fn default() -> Self {
        RoomExitSystem {
            reader: None
        }
    }
}*/
impl RoomExitSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader = world.fetch_mut::<ProximityEvents>().register_reader();
        RoomExitSystem {
            reader
        }
    }
}

impl<'s> System<'s> for RoomExitSystem {
    type SystemData = (
        Read<'s, ProximityEvents>,
        ReadStorage<'s, DestRoom>,
        Write<'s, Option<PerformRoomExit>>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (channel, destrooms, mut perform_room_exit, transforms,): Self::SystemData,
    ) {
        for collision in channel.read(&mut self.reader) {
            let solid_entity = collision.collider1;
            if let Some(exit) = destrooms.get(solid_entity) {
                if let Some(_transform) = transforms.get(solid_entity) {
                    let position = exit.spawn_point();
                    *perform_room_exit = Some(PerformRoomExit(*exit, position));
                }
            }
            let solid_entity = collision.collider2;
            if let Some(exit) = destrooms.get(solid_entity) {
                if let Some(_transform) = transforms.get(solid_entity) {
                    let position = exit.spawn_point();
                    *perform_room_exit = Some(PerformRoomExit(*exit, position));
                }
            }
        }
    }
}