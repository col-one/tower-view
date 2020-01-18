use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Sprite,
    rendy::texture::TextureBuilder, Transparent,
    rendy::hal::image::{Kind, ViewKind, Filter, WrapMode, Anisotropic, SamplerInfo, PackedColor},
    rendy::hal::format,
    types::TextureData,
    Format,
    resources::Tint,
    palette::Srgba,
    };
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity, VecStorage, };
use amethyst::prelude::*;
use amethyst::ecs::prelude::LazyUpdate;
use amethyst::assets::{AssetStorage, Loader, Handle};
use image;
use image::{GenericImageView, ImageDecoder, ImageDecoderExt, ColorType};
use std::borrow::Cow;
use uuid::Uuid;
use std::path::PathBuf;


#[derive(PartialEq, Debug)]
pub struct TwPlaceHolder {
    pub twimage_path: String,
    pub to_cache: bool,
}

impl Component for TwPlaceHolder {
    type Storage = DenseVecStorage<Self>;
}


pub fn init_sprite(world: &World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/tower-64x64.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/tower-64x64.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn sprite_twplaceholder(loader: &Loader, texture_storage: &AssetStorage<Texture>,
                            sprite_sheet_store: &AssetStorage<SpriteSheet>) -> SpriteRender {
    let texture_handle = loader.load(
        "texture/tower-64x64.png",
        ImageFormat::default(),
        (),
        &texture_storage);
    let sprite_handle = loader.load(
        "texture/tower-64x64.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_handle.clone(),
        sprite_number: 0,
    };
    sprite_render
}

pub fn create_entity_twplaceholder(world: &mut World, twimage_path: String, position: Transform, sprite_render: SpriteRender) {
    world.create_entity()
        .with(position)
        .with(sprite_render)
        .with(TwPlaceHolder{twimage_path, to_cache: true})
        .build();
}
