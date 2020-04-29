use amethyst::core::shrev::{EventChannel, ReaderId};
use amethyst::ecs::{Write, ReadStorage, System, Entities};
use amethyst::ecs::{Component, VecStorage};
use amethyst::core::transform::Transform;


use crate::physics::Collision;
use crate::room::DestRoom;
use crate::room;

impl Component for DestRoom {
    type Storage = VecStorage<Self>;
}

pub struct PerformRoomExit(pub room::DestRoom, pub (i32, i32));

#[derive(Default)]
pub struct RoomExitSystem {
    reader: Option<ReaderId<Collision>>
}

impl<'s> System<'s> for RoomExitSystem {
    type SystemData = (
        Write<'s, EventChannel<Collision>>,
        ReadStorage<'s, DestRoom>,
        Entities<'s>,
        Write<'s, Option<PerformRoomExit>>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut channel, destrooms, entities, mut perform_room_exit, transforms,): Self::SystemData,
    ) {
        if let None = self.reader {
            self.reader = Some(channel.register_reader());
        }
        if let Some(reader) = &mut self.reader {
            for collision in channel.read(reader) {
                match collision {
                    Collision::Solid(_moving_entity_id, solid_entity_id) => {
                        let solid_entity = entities.entity(*solid_entity_id);
                        if let Some(exit) = destrooms.get(solid_entity) {
                            if let Some(_transform) = transforms.get(solid_entity) {
                                let position = exit.spawn_point();
                                *perform_room_exit = Some(PerformRoomExit(*exit, position));
                                println!("Exit collision");
                            }
                        }
                    }
                }
            }
        }
    }
}