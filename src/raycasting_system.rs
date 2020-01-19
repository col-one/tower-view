use amethyst::{
        assets::{AssetStorage},
        renderer::{
            SpriteRender, SpriteSheet,
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
use std::cmp::Ordering::Equal;

use crate::inputshandler::{TwInputHandler, TwInputsHandler};
use crate::image::{TwImage, TwActiveComponent};

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


#[derive(SystemDesc)]
pub struct TwImageActiveSystem;

impl<'s> System<'s> for TwImageActiveSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       WriteStorage<'s, TwImage>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       WriteStorage<'s, TwActiveComponent>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        mut tw_images,
        transforms,
        sprites,
        sprite_sheets,
        mut twactives,
        entities
    ): Self::SystemData) {
        let mut remove_active = false;
        for (sprite, transform, tw_image, entity) in (&sprites, &transforms, &mut tw_images, &*entities).join() {
            let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
            let sprite = &sprite_sheet.sprites[sprite.sprite_number];
            let (min_x, max_x, min_y, max_y) = {
                (
                    transform.translation().x - (sprite.width * 0.5),
                    transform.translation().x + (sprite.width * 0.5),
                    transform.translation().y - (sprite.height * 0.5),
                    transform.translation().y + (sprite.height * 0.5),
                )
            };
            if let Some(mouse_world_position) = tw_in.mouse_world_position {
                // if mouse inside sprite
                if mouse_world_position.0 > min_x
                    && mouse_world_position.0 < max_x
                    && mouse_world_position.1 > min_y
                    && mouse_world_position.1 < max_y
                {
                    if !tw_in.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                        tw_in.twimages_under_mouse.push((tw_image.id, transform.translation().z));
                    }
                } else {
                    if tw_in.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                        let index = tw_in.twimages_under_mouse.iter().position(|x| x.0 == tw_image.id).unwrap();
                        tw_in.twimages_under_mouse.remove(index);
                    }
                }
            }
            // set as active image the highest image z order
            tw_in.twimages_under_mouse.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Equal));
            if tw_in.twimages_under_mouse.is_empty() {
                tw_in.twimage_active = None;
            } else {
                if tw_in.twimages_under_mouse[0].0 == tw_image.id {
                    if !twactives.contains(entity) {
                        // add active tw_image component
                        twactives.insert(entity, TwActiveComponent).expect("Failed to add TwActiveComponent.");
                    }
                } else {
                    if twactives.remove(entity).is_some() {
                        twactives.clear();
                    }
                }
            }
            tw_image.z_order = transform.translation().z;
        }
        // prepare remove active
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Escape) && tw_in.keys_pressed.len() == 1  { remove_active = true }
        let mut entities_to_remove = Vec::new();
        for (twactive, entity) in (&mut twactives, &*entities).join() {
            if tw_in.twimages_under_mouse.is_empty() {
                entities_to_remove.push(entity);
            }
        }
        if remove_active {
            for entity in entities_to_remove {
                // remove active tw_image component
                twactives.remove(entity).expect("Failed to remove TwActiveComponent.");
            }
        }
    }
}


