use amethyst::{
        renderer::{
            camera::{ActiveCamera, Camera},
            rendy::wsi::winit::{MouseButton, Window},
        },
        prelude::*,
        input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
        core::{SystemDesc, Transform, math::{Point2, Point3, Vector2}, geometry::Plane},
        derive::SystemDesc,
        ecs::{Join, Read, System, SystemData, World, WriteStorage},
        ecs::prelude::*,
        window::ScreenDimensions,
};

use crate::inputshandler::{TwInputHandler, TwInputsHandler};

pub fn screen_to_world(mouse_position: (f32, f32), camera: &Camera, transform: &Transform, screen_dimensions: &ScreenDimensions) -> Point3<f32>{
    let ray = camera.projection().screen_ray(
        Point2::new(mouse_position.0, mouse_position.1),
        Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
        transform,
    );
    let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
    ray.at_distance(distance)
}


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
        Read<'s, TwInputsHandler>
    );

    fn run(&mut self, (
            entities,
            transforms,
            cameras,
            screen_dimensions,
            active_camera,
            input,
            mut world,
            tw_in
        ): Self::SystemData,) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        if let Some(mouse_position) = tw_in.mouse_position {
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                tw_input_handler.mouse_world_pos = screen_to_world(mouse_position, camera, camera_transform, &screen_dimensions);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwInputsHandlerScreenToWorldSystem;

impl<'s> System<'s> for TwInputsHandlerScreenToWorldSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        Write<'s, TwInputsHandler>
    );

    fn run(&mut self, (
            transforms,
            cameras,
            screen_dimensions,
            mut tw_in
        ): Self::SystemData,) {
        if let Some(mouse_position) = tw_in.mouse_position {
            let (camera, transform) = (&cameras, &transforms).join().next().unwrap();
            let world_position = screen_to_world(mouse_position, camera, transform, &screen_dimensions);
            tw_in.mouse_world_position = Some((world_position.x as f32, world_position.y as f32));
        }
    }
}

