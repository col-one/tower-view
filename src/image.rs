use amethyst::renderer::{
    rendy::texture::TextureBuilder,
    rendy::hal::image::{Kind, ViewKind, Filter, WrapMode, Anisotropic, SamplerInfo, PackedColor},
    rendy::hal::format,
    types::TextureData,
    Format,
    };
use amethyst::ecs::prelude::{Component, DenseVecStorage, VecStorage, };
use image;
use image::{GenericImageView, ColorType};
use std::borrow::Cow;
use uuid::Uuid;

use crate::utils::{premultiply_by_alpha, add_alpha_channel};


// input path
pub struct InputComponent {
    pub path: String,
}

impl Component for InputComponent {
    type Storage = VecStorage<Self>;
}


// active ui component
pub struct TwActiveUiComponent;

impl Component for TwActiveUiComponent {
    type Storage = VecStorage<Self>;
}

// active component
pub struct TwActiveComponent;

impl Component for TwActiveComponent {
    type Storage = VecStorage<Self>;
}


// Component Image
#[derive(PartialEq, Debug, Clone)]
pub struct TwImage {
    pub id: Uuid,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
    pub ratio: f32,
    pub z_order: f32,
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub mouse_offset: Option<(f32, f32)>
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
            z_order: 0.0,
            id,
            alpha: 1.0,
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            mouse_offset: None
        }
    }
}

impl Component for TwImage {
    type Storage = DenseVecStorage<Self>;
}

// Flag component to track texture loading







pub fn get_color_type(color: &ColorType) -> (Format, format::Swizzle) {
    match color {
        ColorType::RGB(8) => (Format::Rgba8Srgb, format::Swizzle(format::Component::R, format::Component::G, format::Component::B, format::Component::A)),
        ColorType::RGBA(8) => (Format::Rgba8Srgb, format::Swizzle(format::Component::R, format::Component::G, format::Component::B, format::Component::A)),
        ColorType::Gray(8) => (Format::R8Unorm, format::Swizzle(format::Component::R, format::Component::R, format::Component::R, format::Component::A)),
        _ => (Format::Rgb8Unorm, format::Swizzle(format::Component::R, format::Component::G, format::Component::B, format::Component::A))
    }
}

pub fn load_texture_from_file (name: &str) ->  (TwImage, TextureData) {
    let img = image::open(name).unwrap();
    let dimensions = img.dimensions();
    let (color_type, swizzle) = get_color_type(&img.color());
    let pixels = match &img.color() {
        ColorType::RGBA(8) => premultiply_by_alpha(&img.raw_pixels()),
        ColorType::RGB(8) => add_alpha_channel(&img.raw_pixels()),
        _ => img.raw_pixels()};
    let texture_builder = TextureBuilder::new()
        .with_data_width(dimensions.0)
        .with_data_height(dimensions.1)
        .with_kind(Kind::D2(dimensions.0, dimensions.1, 1, 1))
        .with_view_kind(ViewKind::D2)
        .with_sampler_info(SamplerInfo {
            min_filter: Filter::Nearest,
            mag_filter: Filter::Nearest,
            mip_filter: Filter::Nearest,
            wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
            lod_bias: 0.0.into(),
            lod_range: std::ops::Range {
                start: 0.0.into(),
                end: 1.0.into(),
            },
            comparison: None,
            border: PackedColor(0),
            anisotropic: Anisotropic::On(8),
            })
        .with_raw_data(Cow::Owned(pixels), color_type)
        .with_swizzle(swizzle);
    (TwImage::new(dimensions.0, dimensions.1, name), TextureData(texture_builder))
}





