use amethyst::{
        renderer::{
            camera::{ActiveCamera, Camera},
            rendy::wsi::winit::{MouseButton, Window},
        },
        prelude::*,
        input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
        core::{SystemDesc, Transform, math::{Point2, Vector2}, geometry::Plane},
        derive::SystemDesc,
        ecs::{Join, Read, System, SystemData, World, WriteStorage},
        ecs::prelude::*,
        window::ScreenDimensions,
};

use crate::twinputshandler::TwInputHandler;

#[derive(SystemDesc)]
pub struct TwMouseRaycastSystem;

impl<'s> System<'s> for TwMouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, World>,
    );

    fn run(&mut self, (
            entities,
            transforms,
            cameras,
            screen_dimensions,
            active_camera,
            input,
            mut world
        ): Self::SystemData,) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        if let Some(mouse_position) = input.mouse_position() {
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                let ray = camera.projection().screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);
                tw_input_handler.mouse_world_pos = mouse_world_position.clone();
            }
        }
    }
}
