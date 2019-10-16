use rand::Rng;

#[derive(Copy, Clone)]
pub enum DestRoom {
    Relative(isize, isize, i32, i32,),
    Absolute(isize, isize, i32, i32,),
}

impl DestRoom {
    pub fn to_absolute_coordinates(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            DestRoom::Relative(rel_x, rel_y, _, _) => (x + *rel_x as i32, y + *rel_y as i32),
            DestRoom::Absolute(abs_x, abs_y, _, _) => (*abs_x as i32, *abs_y as i32),
        }
    }

    pub fn spawn_point(&self) -> (i32, i32) {
        match self {
            DestRoom::Relative(_, _, x, y) => (*x, *y),
            DestRoom::Absolute(_, _, x, y) => (*x, *y),
        }
    }
}

#[derive(Copy, Clone)]
pub enum RoomField {
    Nothing,
    Wall,
    Stone,
    Bush,
    Player,
    Exit(DestRoom),
}

#[derive(Clone)]
pub struct Room {
    pub width: usize,
    pub height: usize,
    pub fields: Vec<RoomField>,
}

impl Room {
    pub fn new(width: usize, height: usize) -> Room {
        let fields = (0..(width * height)).map(|_| RoomField::Nothing).collect();
        Room {
            width,
            height,
            fields,
        }
    }

    pub fn set_field(&mut self, x: usize, y: usize, field: RoomField) {
        if x < self.width && y < self.height {
            let index = x + y * self.width;
            self.fields[index] = field;
        }
    }

    pub fn get_field(&self, x: usize, y: usize) -> Option<RoomField> {
        if x < self.width && y < self.height {
            let index = x + y * self.width;
            Some(self.fields[index])
        } else {
            None
        }
    }

    pub fn room_field_iterator(&self) -> RoomFieldIterator {
        RoomFieldIterator {
            x: 0,
            y: 0,
            room: self
        }
    }
}

#[derive(Default)]
pub struct RoomGeneration {
    pub width: usize,
    pub height: usize,

    pub exit_north: bool,
    pub exit_south: bool,
    pub exit_east: bool,
    pub exit_west: bool,
}


impl RoomGeneration {
    pub fn generate_room(&self, rng: &mut impl Rng) -> Room {
        let mut room = Room::new(self.width, self.height);

        /* Draw borders */
        let wall_borders = RoomField::Wall;
        for x in 0..self.width {
            room.set_field(x, 0, wall_borders);
            room.set_field(x, self.height - 1, wall_borders);
        }
        for y in 0..self.height {
            room.set_field(0, y, wall_borders);
            room.set_field(self.width - 1, y, wall_borders);
        }

        /* Open exits */
        if self.exit_north {
            room.set_field(self.width / 2, self.height - 1,
                RoomField::Exit(DestRoom::Relative(0, -1, self.width as i32 / 2, 1)));
        }
        if self.exit_south {
            room.set_field(self.width / 2, 0,
                RoomField::Exit(DestRoom::Relative(0, 1, self.width as i32 / 2, self.height as i32 - 2)));
        }
        if self.exit_east {
            room.set_field(self.width - 1, self.height / 2, 
                RoomField::Exit(DestRoom::Relative(1, 0, 1, self.height as i32 / 2)));
        }
        if self.exit_west {
            room.set_field(0, self.height / 2,
                RoomField::Exit(DestRoom::Relative(-1, 0, self.width as i32 - 2, self.height as i32 / 2)));
        }


        /* Draw 5-7 random stones */
        for _ in 0..rng.gen_range(5, 8) {
            let x = rng.gen_range(2, self.width - 3);
            let y = rng.gen_range(2, self.height - 3);
            room.set_field(x, y, RoomField::Stone);
        }

        /* Draw 5-7 bushes */
        for _ in 0..rng.gen_range(5, 8) {
            let x = rng.gen_range(2, self.width - 3);
            let y = rng.gen_range(2, self.height - 3);
            room.set_field(x, y, RoomField::Bush);
        }

        /* Add the player somewhere */
        let x = rng.gen_range(2, self.width - 3);
        let y = rng.gen_range(2, self.height - 3);
        room.set_field(x, y, RoomField::Player);

        room
    }
}


pub struct RoomFieldIterator<'a> {
    room: &'a Room,
    x: usize,
    y: usize,
}

impl<'a> Iterator for RoomFieldIterator<'a> {
    type Item = (usize, usize, RoomField);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.room.get_field(self.x, self.y).map(|field| (self.x, self.y, field));
        self.x += 1;
        if self.x >= self.room.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.room.height {
            return None;
        }
        result
    }
}