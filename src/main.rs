#[macro_use]
extern crate log;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use structopt::StructOpt;
use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, SystemData},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::ortho_camera,
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
};

mod twargs_cli;
mod twimage;
mod twimage_system;
mod twcamera;
mod twcamera_system;
mod twinputshandler;
mod tower;
mod twutils;
mod twscene_system;
mod twraycasting_system;

use crate::twargs_cli::Opt;
use crate::tower::{Tower, BACKGROUNDCOLOR};
use crate::twcamera_system::{CameraTranslateNavigationSystem, CameraKeepRatioSystem,
                             CameraZoomNavigationSystem, CameraFitNavigationSystem};
use crate::twimage_system::{TwImageMoveSystem, TwImageLayoutSystem, TwImageActiveSystem, TwImageDeleteSystem,
                            TwImageToFrontSystem, TwImageChangeAlphaSystem, TwImageApplyBlendingSystem};
use crate::twraycasting_system::TwMouseRaycastSystem;
use crate::twscene_system::{SceneBoundingBox};



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
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path)
                             .with_clear(BACKGROUNDCOLOR),)
            .with_plugin(RenderFlat2D::default()))?
        .with(CameraTranslateNavigationSystem, "camera_translate_system", &["input_system"])
        .with(TwImageActiveSystem, "image_active_system", &["input_system"])
        .with(CameraKeepRatioSystem, "camera_ratio_system", &["input_system", "image_active_system"])
        .with(CameraZoomNavigationSystem, "camera_zoom_system", &["input_system", "image_active_system"])
        .with(CameraFitNavigationSystem, "camera_fit_system", &["input_system", "image_active_system"])
        .with(TwMouseRaycastSystem, "mouse_raycasting_system", &["input_system"])
        .with(TwImageLayoutSystem, "image_layout_system", &["input_system"])
        .with(TwImageDeleteSystem, "image_delete_system", &["input_system", "image_active_system"])
        .with(SceneBoundingBox, "scene_bounding_system", &["input_system", "image_active_system"])
        .with(TwImageToFrontSystem, "image_tofront_system", &["input_system", "image_active_system"])
        .with(TwImageChangeAlphaSystem, "image_change_alpha_system", &["input_system", "image_active_system"])
        .with(TwImageApplyBlendingSystem, "image_apply_blending_system", &["input_system", "image_active_system"])
        .with(TwImageMoveSystem, "image_move_system", &["input_system", "image_active_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Tower::default(), game_data)?;
    game.run();
    Ok(())
}
