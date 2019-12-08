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
    transform.set_translation_xyz(0.0, 0.0, WINDOWHEIGHT);
    let cam_entity = world.create_entity()
        .with(TwCamera)
        .with(Camera::standard_3d(WINDOWWIDTH, WINDOWHEIGHT))
        .with(transform)
        .build();
}
