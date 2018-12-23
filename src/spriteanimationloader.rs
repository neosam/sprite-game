use amethyst::{
    prelude::*,
    assets::{Loader, AssetStorage},
    renderer::{Sprite, SpriteSheet, SpriteSheetHandle, PngFormat, Texture, TextureMetadata},
    config::Config,
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use regex::Regex;


#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteDefinition {
    pub name: String,
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
    pub animations: BTreeMap<String, Vec<usize>>,
    pub images: BTreeMap<String, usize>,
}

impl Default for AnimationData {
    fn default() -> Self {
        AnimationData {
            texture_path: String::new(),
            texture_width: 0,
            texture_height: 0,
            sprites: Vec::new(),
            animations: BTreeMap::new(),
            images: BTreeMap::new(),
        }
    }
}

pub struct SpriteAnimationStore {
    pub sprite_sheet_handle: SpriteSheetHandle,
    pub animations: BTreeMap<String, Vec<usize>>,
    pub images: BTreeMap<String, usize>
}

pub fn manually_assign_animations(animation_data: &mut AnimationData) {
    let mut animations : BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut images : BTreeMap<String, usize> = BTreeMap::new();

    let ends_with_number_pattern = Regex::new(r"_\d+$").unwrap();
    for (i, sprite) in (0..).zip(&animation_data.sprites) {
        if let Some(_) = ends_with_number_pattern.find(&sprite.name) {
            let animation_name = ends_with_number_pattern.replace_all(&sprite.name, "");
            println!("Animation name: {}", animation_name);
            let entry = animations.entry(animation_name.to_string()).or_insert_with(|| Vec::new());
            entry.push(i);
        } else {
            images.insert(sprite.name.to_string(), i);
        }
    }

    animation_data.animations = animations;
    animation_data.images = images;
}

pub fn load_sprites<S: ToString, 
                    T: ToString>(world: &mut World, directory: S, filename: T) -> SpriteAnimationStore {
    // ---- Loading animations
    let directory = directory.to_string();
    let filename = filename.to_string();
    let ron_path = format!("{}/{}", directory, filename);
    let mut animations = AnimationData::load(ron_path);
    manually_assign_animations(&mut animations);
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
        images: animations.images.clone(),
    }
}
