use amethyst::{
    prelude::*,
    core::timing::Time,
    assets::{Loader, AssetStorage},
    ecs::{Component, DenseVecStorage},
    ecs::prelude::{Join, Read, System, WriteStorage},
    renderer::{Sprite, SpriteRender, SpriteSheet, SpriteSheetHandle, PngFormat, Texture, TextureMetadata},
    config::Config,
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteDefinition {
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
    pub sprites: Vec<SpriteDefinition>,
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

pub struct SpriteAnimationStore {
    pub sprite_sheet_handle: SpriteSheetHandle,
    pub animations: BTreeMap<String, Vec<usize>>,
}

pub fn load_sprites<S: ToString, 
                    T: ToString>(world: &mut World, directory: S, filename: T) -> SpriteAnimationStore {
    // ---- Loading animations
    let directory = directory.to_string();
    let filename = filename.to_string();
    let ron_path = format!("{}/{}", directory, filename);
    let animations = AnimationData::load(ron_path);
    let texture_path = format!("{}/{}", directory, animations.texture_path);
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            texture_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let mut sprites = Vec::with_capacity(animations.sprites.len());
    for sprite in animations.sprites {
        let offset = if let Some((offset_x, offset_y)) = sprite.offset {
            [offset_x, offset_y]
        } else {
            [0.5; 2]
        };
        sprites.push(Sprite::from_pixel_values(
            animations.texture_width,
            animations.texture_height,
            sprite.width, sprite.height, sprite.x, sprite.y, offset));
    }
    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites
    };

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprite_sheet,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };

    SpriteAnimationStore {
        sprite_sheet_handle,
        animations: animations.animations.clone(),
    }
}
