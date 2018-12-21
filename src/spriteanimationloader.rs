use amethyst::{
    core::timing::Time,
    ecs::{Component, DenseVecStorage},
    ecs::prelude::{Join, Read, System, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
    config::Config,
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub offset: Option<(f32, f32)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationData {
    pub texture_path: String,
    pub texture_width: u32,
    pub texture_height: u32,
    pub sprites: Vec<Sprite>,
    pub animations: BTreeMap<String, Vec<usize>>
}

impl Default for AnimationData {
    fn default() -> Self {
        AnimationData {
            texture_path: String::new(),
            texture_width: 0,
            texture_height: 0,
            sprites: Vec::new(),
            animations: BTreeMap::new()
        }
    }
}
