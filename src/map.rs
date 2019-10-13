use std::collections::BTreeMap;
use crate::room::Room;

pub type Coordinate = (i32, i32);

pub struct Map {
    rooms: BTreeMap<Coordinate, Room>
}

impl Map {
    pub fn new() -> Map {
        Map {
            rooms: BTreeMap::new()
        }
    }

    pub fn add_room(&mut self, coordinate: Coordinate, room: Room) {
        self.rooms.insert(coordinate, room);
    }

    pub fn get_room(&self, coordinate: Coordinate) -> Option<&Room> {
        self.rooms.get(&coordinate)
    }

    pub fn get_room_mut(&mut self, coordinate: Coordinate) -> Option<&mut Room> {
        self.rooms.get_mut(&coordinate)
    }

    pub fn get_room_or_insert(&mut self, coordinate: Coordinate, f: impl FnOnce() -> Room) -> &mut Room {
        self.rooms.entry(coordinate).or_insert_with(f)
    }
}