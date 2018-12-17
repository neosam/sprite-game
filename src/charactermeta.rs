use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CharacterDirection {
    Left, Right, Up, Down
}

#[derive(Clone, PartialEq, Eq)]
pub struct CharacterMeta {
    pub direction: CharacterDirection,
    pub moving: bool,
}

impl CharacterMeta {
    pub fn new(direction: CharacterDirection) -> Self {
        CharacterMeta {
            direction,
            moving: false
        }
    }
}


impl Component for CharacterMeta {
    type Storage = DenseVecStorage<Self>;
}
