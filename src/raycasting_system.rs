/// contains functions and system related to "raycasting" mouse image intersection.
use amethyst::{
        assets::{AssetStorage},
        renderer::{
            SpriteRender, SpriteSheet,
            camera::{Camera},
        },
        input::{VirtualKeyCode},
        core::{SystemDesc, Transform, math::{Point2, Point3, Vector2}, geometry::Plane},
        derive::SystemDesc,
        ecs::{Join, Read, System, SystemData, World, WriteStorage},
        ecs::prelude::*,
        window::ScreenDimensions,
};

use std::cmp::Ordering::Equal;

use crate::inputshandler::{TwInputsHandler};
use crate::image::{TwImage, TwActiveUiComponent, TwActiveComponent};

/// Utility function which convert screen position to world position coord with a 0 z intersect plane
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
pub struct TwInputsHandlerScreenToWorldSystem;
/// system which convert screen to world TowerData.mouse_position and set the member the return to
/// TowerData.mouse_world_position
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


#[derive(SystemDesc, Default)]
pub struct TwImageActiveSystem {
    ui_active: bool
}
/// system which manage that TwImages are active or not, it also sort in a TwInputsHandler the images
/// by z position to get their orders and set their z transform value
/// An active TwImage get an new TwActiveComponent and a TwActiveUIComponent
/// TwActiveUIComponent is deleted when Escape is pressed while TwActiveComponent is deleted when
/// mouse is outside of the image.
impl<'s> System<'s> for TwImageActiveSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       WriteStorage<'s, TwImage>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       WriteStorage<'s, TwActiveUiComponent>,
                       WriteStorage<'s, TwActiveComponent>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        mut tw_images,
        transforms,
        sprites,
        sprite_sheets,
        mut tw_ui_actives,
        mut tw_actives,
        entities
    ): Self::SystemData) {
        let mut remove_active = false;
        // as there is no TwActiveComponent, ui_active switch to false
        if tw_ui_actives.is_empty() {
            self.ui_active = false;
        }
        for (sprite, transform, _tw_image, entity) in (&sprites, &transforms, &mut tw_images, &*entities).join() {
            if let Some(sprite_sheet) = sprite_sheets.get(&sprite.sprite_sheet) {
                let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                let is_present = tw_in.z_ordered_entities.iter().any(|e| e == &entity);
                if !is_present {
                    tw_in.z_ordered_entities.push(entity);
                }
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
                        // if active is busy (used by action) or actives empty
                        if !tw_in.active_busy || tw_in.active_entities.is_empty() {
                            tw_actives.insert(entity, TwActiveComponent).expect("Failed to add TwActiveComponent.");
                            if !self.ui_active {
                                tw_ui_actives.insert(entity, TwActiveUiComponent).expect("Failed to add TwActiveComponent.");
                                self.ui_active = true;
                            }
                            if tw_in.active_entities.len() >= 2 {
                                tw_in.active_entities.remove(0);
                                tw_in.active_entities.push(entity);
                            } else {
                                tw_in.active_entities.push(entity);
                            }
                        }
                    } else {
                        if let Some(_ok) = tw_actives.remove(entity) { debug!("Mouse cursor is outside the image, Remove TwActiveComponent") };
                        if let Some(id) = tw_in.active_entities.iter().position(|i| i == &entity) {
                            tw_in.active_entities.remove(id);
                        }
                    }
                }
            }
        }
        // prepare remove active
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Escape) && tw_in.keys_pressed.len() == 1  { remove_active = true }
        let mut entities_to_remove = Vec::new();
        for (_tw_ui_active, entity) in (&mut tw_ui_actives, &*entities).join() {
            entities_to_remove.push(entity);
        }
        if remove_active {
            for entity in entities_to_remove {
                // remove active tw_image component
                tw_ui_actives.remove(entity).expect("Failed to remove TwActiveUiComponent.");
                self.ui_active = false;
            }
        }
        // sort active by z if active is not busy, need new active order
        if !tw_in.active_busy {
            if tw_in.active_entities.len() >= 2 {
                tw_in.active_entities.sort_by(|e1, e2|
                    transforms.get(*e1).unwrap().translation().z.partial_cmp(
                    &transforms.get(*e2).unwrap().translation().z).unwrap_or(Equal));
            }
        }
        tw_in.z_ordered_entities.sort_by(|e1, e2|
            transforms.get(*e1).unwrap().translation().z.partial_cmp(
            &transforms.get(*e2).unwrap().translation().z).unwrap_or(Equal));
    }
}

