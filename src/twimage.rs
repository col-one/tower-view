use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Sprite,
    rendy::texture::TextureBuilder, Transparent,
    rendy::hal::image::{Kind, ViewKind, Filter, WrapMode, Anisotropic, SamplerInfo, PackedColor},
    types::TextureData,
    Format,
    };
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity, VecStorage};
use amethyst::prelude::*;
use amethyst::assets::{AssetStorage, Loader, Handle};
use image;
use image::{GenericImageView, ImageDecoder, ImageDecoderExt, ColorType};
use std::borrow::Cow;
use uuid::Uuid;
use std::path::PathBuf;


use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT, TowerData};
use crate::twutils::premultiply_by_alpha;
use crate::twargs_cli::Opt;


pub struct InputComponent {
    pub path: String,
}

impl Component for InputComponent {
    type Storage = VecStorage<Self>;
}

impl  InputComponent {
    fn new(path: String) -> Self {
        Self {path}
    }
}

// Component Image
#[derive(PartialEq, Debug)]
pub struct TwImage {
    pub id: Uuid,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
    pub ratio: f32,
    pub z_order: u8,
}

impl  TwImage {
    pub fn new(width: u32, height: u32, file_name: &str) -> Self {
        let ratio = width as f32 / height as f32;
        let id = Uuid::new_v4();
        Self {
            width,
            height,
            file_name: file_name.to_owned(),
            ratio,
            z_order: 0,
            id,
        }
    }
}

impl Component for TwImage {
    type Storage = DenseVecStorage<Self>;
}

// Flag component to track texture loading







pub fn get_color_type(color: &ColorType) -> Format {
    match color {
        ColorType::RGB(8) => Format::Rgb8Srgb,
        ColorType::RGBA(8) => Format::Rgba8Srgb,
        _ => Format::Rgb8Unorm
    }
}

pub fn load_texture_from_file (name: &str) ->  (TwImage, TextureData) {
    let img = image::open(name).unwrap();
    let dimensions = img.dimensions();
    let pixels = match get_color_type(&img.color()) {
        Format::Rgba8Srgb => premultiply_by_alpha(&img.raw_pixels()),
        _ => img.raw_pixels()};
    let texture_builder = TextureBuilder::new()
        .with_data_width(dimensions.0)
        .with_data_height(dimensions.1)
        .with_kind(Kind::D2(dimensions.0, dimensions.1, 1, 1))
        .with_view_kind(ViewKind::D2)
        .with_sampler_info(SamplerInfo {
            min_filter: Filter::Linear,
            mag_filter: Filter::Nearest,
            mip_filter: Filter::Linear,
            wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
            lod_bias: 0.0.into(),
            lod_range: std::ops::Range {
                start: 0.0.into(),
                end: 100.0.into(),
            },
            comparison: None,
            border: PackedColor(0),
            anisotropic: Anisotropic::Off,
            })
        .with_raw_data(Cow::Owned(pixels), get_color_type(&img.color()));
    (TwImage::new(dimensions.0, dimensions.1, name), TextureData(texture_builder))
}

pub fn create_sprite_sheet(world: &mut World, texture_data: TextureData, tw_image: &TwImage) -> Handle<SpriteSheet> {
    let texture_storage = &world.fetch_mut::<AssetStorage<Texture>>();
    let mut sprites = Vec::with_capacity(1);
    let loader = &world.fetch_mut::<Loader>();
    let texture = loader.load_from_data(texture_data, (), &texture_storage);
    let sprite = Sprite::from_pixel_values(
            tw_image.width, tw_image.height, tw_image.width,
            tw_image.height, 0, 0, [0.0, 0.0],
            false, false,
        );
    sprites.push(sprite);
    let sprite_sheet = SpriteSheet {
        texture,
        sprites,
    };
    loader.load_from_data(
        sprite_sheet,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

pub fn create_entity_twimage(world: &mut World, tw_image: TwImage, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_x( 0.0);
    transform.set_translation_y( 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world.create_entity()
        .with(transform)
        .with(sprite_render)
        .with(tw_image)
        .with(Transparent)
        .build();
}

pub fn load_image_from_inputs_arg(world: &mut World) {
    let mut z_count = {
        let mut td = world.fetch_mut::<TowerData>();
        td.twimage_count
    };
    let inputs = {
        let opt = world.fetch::<Opt>();
        opt.inputs.iter().map(|input| InputComponent::new(input.to_owned()))
            .collect::<Vec<_>>()
    };
    for path in inputs {
        let (mut tw_image, texture_data) = load_texture_from_file(&path.path);
        let sprite_sheet = create_sprite_sheet(world, texture_data, &tw_image);
        tw_image.z_order = z_count;
        create_entity_twimage(world, tw_image, sprite_sheet);
        z_count += 1;
    }
        let mut td = world.fetch_mut::<TowerData>();
        td.twimage_count = z_count;
}

pub fn load_image_from_path(world: &mut World, path: &str) {
    let mut z_count = {
        let mut td = world.fetch_mut::<TowerData>();
        td.twimage_count
    };
    let (mut tw_image, texture_data) = load_texture_from_file(path);
    let sprite_sheet = create_sprite_sheet(world, texture_data, &tw_image);
    tw_image.z_order = z_count;
    create_entity_twimage(world, tw_image, sprite_sheet);
    z_count += 1;
    let mut td = world.fetch_mut::<TowerData>();
    td.twimage_count = z_count;
}
