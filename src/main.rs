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

mod twimage;
mod twimage_system;
mod twcamera;
mod twcamera_system;
mod twinputshandler;
mod tower;

use crate::tower::{Tower, BACKGROUNDCOLOR};
use crate::twcamera_system::CameraTranslateNavigationSystem;
use crate::twimage_system::TwImageMoveSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_bundle = InputBundle::<StringBindings>::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
                        .with_clear(BACKGROUNDCOLOR),)
                .with_plugin(RenderFlat2D::default())
        )?
        .with(ortho_camera::CameraOrthoSystem, "camera_ortho_system", &["input_system"])
        .with(CameraTranslateNavigationSystem, "camera_translate_system", &["input_system"])
        .with(TwImageMoveSystem, "image_move_system", &["input_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Tower::default(), game_data)?;
    game.run();
    Ok(())
}
