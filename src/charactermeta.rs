//! Module contains the meta informatormation for characters
//! 
//! Meta information is the direction the character looks at
//! and if the character is moving.

use amethyst::ecs::{Component, DenseVecStorage};

/// Direction on a 2D map.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CharacterDirection {
    Left, Right, Up, Down
}

/// Component which holds information for characters.
#[derive(Clone, PartialEq, Eq)]
pub struct CharacterMeta {
    pub direction: CharacterDirection,
    pub moving: bool,
}

impl CharacterMeta {
    /// Creatre a new character meta which is not warlking.
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
