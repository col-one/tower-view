use amethyst::prelude::*;
use amethyst::{utils::ortho_camera::{CameraOrtho, CameraNormalizeMode, CameraOrthoWorldCoordinates},
            core::transform::Transform,
            renderer::{Camera},
            core::{Time, Axis2},
};
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH};


pub struct TwCamera;

impl Component for TwCamera {
    type Storage = DenseVecStorage<Self>;
}


pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WINDOWWIDTH * 0.5, WINDOWHEIGHT * 0.5, 1.0);
    world.create_entity()
        .with(TwCamera)
        .with(Camera::standard_2d(WINDOWWIDTH, WINDOWHEIGHT))
        .with(transform)
        .with(CameraOrtho::new(CameraNormalizeMode::Lossy{stretch_direction: Axis2::X},
        CameraOrthoWorldCoordinates{right:WINDOWWIDTH / 2.0, left:-WINDOWWIDTH / 2.0, top:-WINDOWHEIGHT / 2.0, bottom:WINDOWHEIGHT / 2.0}))
        .build();
}
