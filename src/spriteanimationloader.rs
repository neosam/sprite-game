//! Load animations and sprites from a ron file.

use amethyst::{
    assets::{AssetStorage, Loader},
    config::Config,
    prelude::*,
    renderer::{ImageFormat, Sprite, SpriteSheet, /*SpriteSheetHandle,*/ Texture, /*TextureMetadata*/
        sprite::SpriteSheetHandle,
    },
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Definintion of one sprite in the RON file.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteDefinition {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub offset: Option<(f32, f32)>,
}

/// RON file definition.
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

/// Stores all animations and sprites and sprites which can be used
/// ingame.
pub struct SpriteAnimationStore {
    pub sprite_sheet_handle: SpriteSheetHandle,
    pub animations: BTreeMap<String, Vec<usize>>,
    pub images: BTreeMap<String, usize>,
}

/// Use an AnimationData and create animations and sprite images based on sprite names.
///
/// If a name ends with underscores followed by numbers it is treated as part of an animations.
/// In this case it will add it to the animations, otherwise it will simply store the index in
/// the images tree map under that name.
pub fn manually_assign_animations(animation_data: &mut AnimationData) {
    let mut animations: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut images: BTreeMap<String, usize> = BTreeMap::new();

    let ends_with_number_pattern = Regex::new(r"_\d+$").unwrap();
    for (i, sprite) in (0..).zip(&animation_data.sprites) {
        if let Some(_) = ends_with_number_pattern.find(&sprite.name) {
            let animation_name = ends_with_number_pattern.replace_all(&sprite.name, "");
            println!("Animation name: {}", animation_name);
            let entry = animations
                .entry(animation_name.to_string())
                .or_insert_with(|| Vec::new());
            entry.push(i);
        } else {
            images.insert(sprite.name.to_string(), i);
        }
    }

    animation_data.animations = animations;
    animation_data.images = images;
}

/// Load animations and images from the given ron file.
///
/// It requires a mutable reference to the world, a directory, where the assets are stored and the filename
/// of the ron file inside the directory.  The reference to the image file in the ron file is relative to the
/// directory provides as second argument.
pub fn load_sprites(
    world: &mut World,
    directory: impl ToString,
    filename: impl ToString,
) -> SpriteAnimationStore {
    // ---- Loading animations
    info!("Loading animations");
    let directory = directory.to_string();
    let filename = filename.to_string();
    let ron_path = format!("{}/{}", directory, filename);
    let mut animations = AnimationData::load(ron_path).expect("Animation data should load");
    manually_assign_animations(&mut animations);
    let texture_path = format!("{}/{}", directory, animations.texture_path);
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            texture_path,
            ImageFormat::default(),
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
            sprite.width,
            sprite.height,
            sprite.x,
            sprite.y,
            offset,
            false,
            false
        ));
    }
    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites,
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
