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
        rendy::wsi::winit::dpi::LogicalSize,
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
use crate::camera_system::{CameraTranslateNavigationSystem, CameraKeepRatioSystem, CameraZoomNavigationSystem, CameraFitNavigationSystem, CameraCenterSystem, CameraOriginalScaleSystem};
use crate::image_system::{TwImageMoveSystem, TwImageLayoutSystem, TwImageDeleteSystem,
                          TwImageToFrontSystem, TwImageApplyBlendingSystem, TwImageLoadFromCacheSystem,
                          TwImageNextSystem};
use crate::raycasting_system::{TwImageActiveSystem, TwInputsHandlerScreenToWorldSystem};
use crate::scene_system::{SceneBoundingBox};
use crate::ui_system::{SliderChannelsSystem};
use crate::placeholder_system::{TwCachingImages, TwImageDroppedSystem};


/// Entry point of tower program.
/// Create the logger of tower and one for amethyst engine.
/// Init the main loop of the program.
/// Manage the display from display.ron
/// Add all the tower systems to the GameDataBuilder
/// And finally run the loop application
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
    // the display size present in the ron is not changeable, Z MAGICNUMBER is dependent of this size.
    let display_config_path = config_dir.join("display.ron");
    let tower_data = GameDataBuilder::default()
        // Active image system
        .with(TwImageActiveSystem::default(), "image_active_system", &[])
        // debug
//        .with(DebugLinesSystem, "ex", &[])
        // Camera system
        .with(CameraTranslateNavigationSystem::default(), "camera_translate_system", &[])
        .with(CameraKeepRatioSystem{previous_size: LogicalSize{width: 0.0, height: 0.0}}, "camera_ratio_system", &[])
        .with(CameraZoomNavigationSystem::default(), "camera_zoom_system", &["image_active_system"])
        .with(CameraFitNavigationSystem, "camera_fit_system", &["image_active_system"])
        .with(CameraCenterSystem::default(), "camera_center_system", &["image_active_system"])
        .with(CameraOriginalScaleSystem, "camera_original_system", &["image_active_system"])
        // Image system
        .with(TwImageLayoutSystem::default(), "image_layout_system", &["image_active_system"])
        .with(TwImageDeleteSystem, "image_delete_system", &["image_active_system"])
        .with(SceneBoundingBox::default(), "scene_bounding_system", &["image_active_system"])
        .with(TwImageToFrontSystem, "image_tofront_system", &["image_active_system"])
        .with(TwImageApplyBlendingSystem, "image_apply_blending_system", &["image_active_system"])
        .with(TwImageMoveSystem::default(), "image_move_system", &["image_active_system"])
        .with(TwImageDroppedSystem, "dropped_images", &[])
        .with(TwCachingImages::default(), "caching_image_system", &["dropped_images"])
        .with(TwImageLoadFromCacheSystem, "image_load_from_cache", &["caching_image_system"])
        .with(TwImageNextSystem, "image_next_cache", &[])
        .with(TwInputsHandlerScreenToWorldSystem, "convert_screen_to_world", &[])
        // UI
        .with(SliderChannelsSystem{open: false}, "slider_alpha_system", &["image_active_system"])
        // bundle + plugins
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path))
            .with_plugin(RenderImgui::<StringBindings>::default())
            .with_plugin(RenderDebugLines::default())
            .with_plugin(RenderSkybox::with_colors(
                palette::Srgb::new(BACKGROUNDCOLOR[0], BACKGROUNDCOLOR[1], BACKGROUNDCOLOR[2]),
                palette::Srgb::new(BACKGROUNDCOLOR2[0], BACKGROUNDCOLOR2[1], BACKGROUNDCOLOR2[2])))
            .with_plugin(RenderFlat2D::default()))?;

    let assets_dir = app_root.join("assets");
    let mut tower = Application::new(assets_dir, Tower::default(), tower_data)?;
    tower.run();
    Ok(())
}
