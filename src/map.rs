use std::collections::BTreeMap;
use crate::room::{Room, RoomGeneration};
use rand::prelude::*;
use rand::distributions::Standard;

pub type Coordinate = (i32, i32);

#[derive(Clone)]
pub struct Map<T> {
    rooms: BTreeMap<Coordinate, T>
}

pub type RoomMap = Map<Room>;

impl<T> Map<T> {
    pub fn new() -> Map<T> {
        Map {
            rooms: BTreeMap::new()
        }
    }

    pub fn add_room(&mut self, coordinate: Coordinate, room: T) {
        self.rooms.insert(coordinate, room);
    }

    pub fn get_room(&self, coordinate: Coordinate) -> Option<&T> {
        self.rooms.get(&coordinate)
    }

    pub fn has_room(&self, coordinate: Coordinate) -> bool {
        self.rooms.contains_key(&coordinate)
    }

    pub fn get_room_mut(&mut self, coordinate: Coordinate) -> Option<&mut T> {
        self.rooms.get_mut(&coordinate)
    }

    pub fn get_room_or_insert(&mut self, coordinate: Coordinate, f: impl FnOnce() -> T) -> &mut T {
        self.rooms.entry(coordinate).or_insert_with(f)
    }
}

#[derive(Default)]
pub struct DungeonGen {
    pub corridor_length: u32,
    pub splits: u32,
}

enum Direction {
    North, South, East, West,
}
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,

            // Not possible but the compiler doesn't know that.
            _ => Direction::North
        }
    }
}

impl Direction {
    fn relative_pos(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    fn add(&self, coordinates: (i32, i32)) -> (i32, i32) {
        let relative_pos = self.relative_pos();
        (relative_pos.0 + coordinates.0, relative_pos.1 + coordinates.1)
    }

    fn set_exit(&self, room_gen: &mut RoomGeneration) {
        match self {
            Direction::North => room_gen.exit_north = true,
            Direction::South => room_gen.exit_south = true,
            Direction::East => room_gen.exit_east = true,
            Direction::West => room_gen.exit_west = true,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl DungeonGen {
    pub fn generate(&self, rng: &mut impl Rng, width: usize, height: usize) -> Map<RoomGeneration> {
        let mut map = Map::new();
        let mut coordinate_stack = Vec::new();
        let mut coordinate = (0, 0);
        let mut room = RoomGeneration::default();
        room.width = width;
        room.height = height;
        map.add_room(coordinate, room);
        for _ in 0..self.corridor_length {
            let choice = {
                let mut i = 0;
                loop {
                    if i == 8 {
                        break None
                    }
                    i += 1;
                    let direction: Direction = rng.gen();
                    let new_coordinate = direction.add(coordinate);
                    println!("Map gen: {:?}, {:?}", coordinate, new_coordinate);
                    if !map.has_room(new_coordinate) {
                        break Some((direction, new_coordinate))
                    }
                }
            };
            if let Some((direction, new_coordinate)) = choice {
                direction.set_exit(map.get_room_mut(coordinate).unwrap());
                coordinate_stack.push(coordinate);
                coordinate = new_coordinate;
                let mut new_room = RoomGeneration::default();
                new_room.width = width;
                new_room.height = height;
                direction.reverse().set_exit(&mut new_room);
                map.add_room(coordinate, new_room);
            }
        }

        map
    }
}

impl Map<RoomGeneration> {
    pub fn generate_map(&self, rng: &mut impl Rng) -> Map<Room> {
        let mut map = Map::new();
        for key in self.rooms.keys() {
            let value = self.rooms.get(key).unwrap();
            map.add_room(*key, value.generate_room(rng));
        }
        map
    }
}
