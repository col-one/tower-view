use amethyst::prelude::*;
use amethyst::{core::transform::Transform,
            renderer::{Camera},
            core::math::{Point2, Point3, Vector2},

};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH, MAGIC_NUMBER_Z};


pub struct TwCamera;

impl Component for TwCamera {
    type Storage = DenseVecStorage<Self>;
}


pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, MAGIC_NUMBER_Z);
    let _cam_entity = world.create_entity()
        .with(TwCamera)
        .with(Camera::standard_3d(WINDOWWIDTH, WINDOWHEIGHT))
        .with(transform)
        .build();
}


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

