use amethyst::core::shrev::{ReaderId};
use amethyst::ecs::{Write, ReadStorage, System};
use amethyst::ecs::{Component, VecStorage};
use amethyst::core::transform::Transform;


use specs_physics::events::{ProximityEvent, ProximityEvents};
use crate::room::DestRoom;
use crate::room;

impl Component for DestRoom {
    type Storage = VecStorage<Self>;
}

pub struct PerformRoomExit(pub room::DestRoom, pub (i32, i32));

pub struct RoomExitSystem {
    reader: Option<ReaderId<ProximityEvent>>
}

impl Default for RoomExitSystem {
    fn default() -> Self {
        RoomExitSystem {
            reader: None
        }
    }
}

impl<'s> System<'s> for RoomExitSystem {
    type SystemData = (
        Write<'s, ProximityEvents>,
        ReadStorage<'s, DestRoom>,
        Write<'s, Option<PerformRoomExit>>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut channel, destrooms, mut perform_room_exit, transforms,): Self::SystemData,
    ) {
        if let None = self.reader {
            self.reader = Some(channel.register_reader());
        }
        if let Some(reader) = &mut self.reader {
            for collision in channel.read(reader) {
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
}