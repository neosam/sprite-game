use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CharacterDirection {
    Left, Right, Up, Down
}

pub struct CharacterMeta {
    pub direction: CharacterDirection
}


impl Component for CharacterMeta {
    type Storage = DenseVecStorage<Self>;
}
