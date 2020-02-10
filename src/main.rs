#[macro_use]
extern crate log;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        palette,
        plugins::{RenderFlat2D, RenderToWindow, RenderDebugLines, RenderSkybox},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
};
use amethyst_imgui::RenderImgui;

mod args_cli;
mod image;
mod image_system;
mod camera;
mod camera_system;
mod inputshandler;
mod tower;
mod utils;
mod scene_system;
mod raycasting_system;
mod ui_system;
mod placeholder;
mod placeholder_system;


use crate::tower::{Tower, BACKGROUNDCOLOR, BACKGROUNDCOLOR2};
use crate::camera_system::{CameraTranslateNavigationSystem, CameraKeepRatioSystem, CameraZoomNavigationSystem, CameraFitNavigationSystem, CameraCenterSystem, CameraOrignalScaleSystem};
use crate::image_system::{TwImageMoveSystem, TwImageLayoutSystem, TwImageDeleteSystem,
                          TwImageToFrontSystem, TwImageApplyBlendingSystem, TwImageLoadFromCacheSystem,
                          TwImageNextSystem};
use crate::raycasting_system::{TwImageActiveSystem, TwInputsHandlerScreenToWorldSystem};
use crate::scene_system::{SceneBoundingBox};
use crate::ui_system::{SliderChannelsSystem};
use crate::placeholder_system::{TwPlaceHolderLoadTwImageSystem, TwPlaceHolderCacheSystem, TwImageDroppedSystem};


fn main() -> amethyst::Result<()> {
    Builder::new()
    .format(|buf, record| {
        writeln!(buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .init();



    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_bundle = InputBundle::<StringBindings>::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        // Active image system
        .with(TwImageActiveSystem::default(), "image_active_system", &["input_system"])
        // debug
//        .with(DebugLinesSystem, "ex", &["input_system"])
        // Camera system
        .with(CameraTranslateNavigationSystem::default(), "camera_translate_system", &["input_system"])
        .with(CameraKeepRatioSystem, "camera_ratio_system", &["input_system", "image_active_system"])
        .with(CameraZoomNavigationSystem::default(), "camera_zoom_system", &["input_system", "image_active_system"])
        .with(CameraFitNavigationSystem, "camera_fit_system", &["input_system", "image_active_system"])
        .with(CameraCenterSystem::default(), "camera_center_system", &["input_system", "image_active_system"])
        .with(CameraOrignalScaleSystem, "camera_original_system", &["input_system", "image_active_system"])
        // Image system
        .with(TwImageLayoutSystem::default(), "image_layout_system", &["image_active_system"])
        .with(TwImageDeleteSystem, "image_delete_system", &["input_system", "image_active_system"])
        .with(SceneBoundingBox::default(), "scene_bounding_system", &["input_system", "image_active_system"])
        .with(TwImageToFrontSystem, "image_tofront_system", &["input_system", "image_active_system"])
        .with(TwImageApplyBlendingSystem, "image_apply_blending_system", &["input_system", "image_active_system"])
        .with(TwImageMoveSystem::default(), "image_move_system", &["input_system", "image_active_system"])
        .with(TwPlaceHolderLoadTwImageSystem, "place_holder_system", &["input_system"])
        .with(TwImageLoadFromCacheSystem, "image_load_from_cache", &["input_system", "place_holder_system"])
        .with(TwImageNextSystem, "image_next_cache", &["input_system"])
        .with(TwPlaceHolderCacheSystem, "images_to_cache", &["input_system"])
        .with(TwImageDroppedSystem, "dropped_images", &["input_system"])
        .with(TwInputsHandlerScreenToWorldSystem, "convert_screen_to_world", &[])
        // UI
        .with(SliderChannelsSystem{open: false}, "slider_alpha_system", &["input_system", "image_active_system"])
        // bundle + plugins
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path)
                             .with_clear(BACKGROUNDCOLOR),)
            .with_plugin(RenderImgui::<StringBindings>::default())
            .with_plugin(RenderDebugLines::default())
            .with_plugin(RenderSkybox::with_colors(
                palette::Srgb::new(BACKGROUNDCOLOR[0], BACKGROUNDCOLOR[1], BACKGROUNDCOLOR[2]),
                palette::Srgb::new(BACKGROUNDCOLOR2[0], BACKGROUNDCOLOR2[1], BACKGROUNDCOLOR2[2])))
            .with_plugin(RenderFlat2D::default()))?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Tower::default(), game_data)?;
    game.run();
    Ok(())
}
