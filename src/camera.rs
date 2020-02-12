/// camera.rs is the file creation for camera entity.
/// Every non system instruction about camera are placed here.
/// The main camera of Tower is perspective camera, it is placed at the origin (0,0,0)
/// then step back of the MAGIC_NUMBER_Z on axe Z, it's a known limitation cause this number
/// is deduce from screen hdpi to get the 100% display  of the images.

use amethyst::prelude::*;
use amethyst::{core::transform::Transform,
            renderer::{Camera},
            core::math::{Point2, Point3, Vector2},

};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH, MAGIC_NUMBER_Z};

/// TwCamera is the Tower camera component, currently it handling nothing
/// it's more here as placeholder.
pub struct TwCamera;

impl Component for TwCamera {
    type Storage = DenseVecStorage<Self>;
}

/// Init the Entity with different components : TwCamera, Camera, Transform.
/// Camera size is given by WINDOWWIDTH and WINDOWHEIGHT.
pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, MAGIC_NUMBER_Z);
    let _cam_entity = world.create_entity()
        .with(TwCamera)
        .with(Camera::standard_3d(WINDOWWIDTH, WINDOWHEIGHT))
        .with(transform)
        .build();
}

/// Give the screen coord from a world one.
/// Tips: to have a coherent result don't forget to multiply this result with the screen hdpi
pub fn world_to_screen(
    camera: &Camera,
    world_position: Point3<f32>,
    screen_diagonal: Vector2<f32>,
    camera_transform: &Transform,
) -> Point2<f32> {
    let transformation_matrix = camera_transform.global_matrix().try_inverse().unwrap();
    let screen_pos =
        (camera.as_matrix() * transformation_matrix).transform_point(&world_position);
    Point2::new(
        (screen_pos.x + 1.0) * screen_diagonal.x / 2.0,
        (screen_pos.y + 1.0) * screen_diagonal.y / 2.0,
    )
}

