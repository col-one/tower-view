use amethyst::core::{SystemDesc, Transform, math::{Point2, Vector2}, Stopwatch};
use amethyst::derive::SystemDesc;
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::renderer::{camera::{ActiveCamera, Camera, Projection},
                        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat}, };
use amethyst::assets::{AssetStorage};

use uuid::Uuid;
use std::{thread, time};

use crate::twimage::TwImage;

use crate::twinputshandler::TwInputHandler;
use crate::twutils::point_in_rect;
use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT};
use log;

#[derive(SystemDesc)]
pub struct TwImageActiveSystem;

impl<'s> System<'s> for TwImageActiveSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwImage>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       Entities<'s>);
    fn run(&mut self, (
        input,
        mut world,
        tw_images,
        transforms,
        sprites,
        sprite_sheets,
        entities
    ): Self::SystemData) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        for (sprite, transform, tw_image, entity) in (&sprites, &transforms, &tw_images, &*entities).join() {
            let mouse_world_position = tw_input_handler.mouse_world_pos;
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
            // if mouse inside sprite
            if mouse_world_position.x > min_x
                && mouse_world_position.x < max_x
                && mouse_world_position.y > min_y
                && mouse_world_position.y < max_y
            {
                if !tw_input_handler.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                    tw_input_handler.twimages_under_mouse.push((tw_image.id, tw_image.z_order));
                }
            } else {
                if tw_input_handler.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                    let index = tw_input_handler.twimages_under_mouse.iter().position(|x| x.0 == tw_image.id).unwrap();
                    tw_input_handler.twimages_under_mouse.remove(index);
                }
            }
            // set as active image the highest image z order
            tw_input_handler.twimages_under_mouse.sort_by(|a, b| b.1.cmp(&a.1));
            if tw_input_handler.twimages_under_mouse.is_empty() {
                tw_input_handler.set_twimage_active(None);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageMoveSystem;

impl<'s> System<'s> for TwImageMoveSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       Entities<'s>);
    fn run(&mut self, (
            input,
            mut world,
            tw_images,
            mut transforms,
            sprites,
            sprite_sheets,
            entities
        ): Self::SystemData) {
        let mut tw_input_handler = world.fetch_mut::<TwInputHandler>();
        for (sprite, transform, tw_image, entity) in (&sprites, &mut transforms, &tw_images, &*entities).join() {
            if input.key_is_down(VirtualKeyCode::LAlt) && input.mouse_button_is_down(MouseButton::Left) {
                // trace vector to move image
                if tw_input_handler.last_mouse_pos.is_none() {
                    let world_pos = {
                        Some((tw_input_handler.mouse_world_pos.x, tw_input_handler.mouse_world_pos.y))
                    };
                    tw_input_handler.set_last_mouse_pos(world_pos);
                }
                if !tw_input_handler.twimages_under_mouse.is_empty() && tw_input_handler.twimages_under_mouse[0].0 == tw_image.id {
                    let world_pos = {
                        Some((tw_input_handler.mouse_world_pos.x, tw_input_handler.mouse_world_pos.y))
                    };
                    let (x, y) = tw_input_handler.last_mouse_pos.unwrap();
                    let (x2, y2) = world_pos.unwrap();
                    let dist = ((x2 - x), (y2 - y));
                    let delta_x = dist.0 - tw_input_handler.last_mouse_dist.0;
                    let delta_y = dist.1 - tw_input_handler.last_mouse_dist.1;
                    tw_input_handler.last_mouse_dist = (dist.0, dist.1);
                    transform.prepend_translation_x(delta_x);
                    transform.prepend_translation_y(delta_y);
                }
            // reset of position data mouse and active image
            } else if input.key_is_down(VirtualKeyCode::LAlt) {
                tw_input_handler.set_last_mouse_pos(None);
                tw_input_handler.last_mouse_dist = (0.0, 0.0);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageLayoutSystem;

impl<'s> System<'s> for TwImageLayoutSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>, );
    fn run(&mut self, (
        input,
        tw_images,
        mut transforms,
        sprites,
        sprite_sheets
    ): Self::SystemData) {
        if input.key_is_down(VirtualKeyCode::L) {
            let twimage_count = tw_images.count() as f32;
            let xy_limit = match twimage_count.sqrt() {
                xy_limit if xy_limit < 2.0 => 2.0,
                _ => twimage_count.sqrt()
            };
            let offset = 10.0;
            let mut i = 0;
            'out: for x in 0..xy_limit as usize {
                    for y in 0..xy_limit as usize {
                        if i >= twimage_count as usize { break 'out }
                        let (tw_image, transform, sprite) = (&tw_images, &mut transforms, &sprites).join().nth(i).unwrap();
                        let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                        let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                        transform.set_translation_x((sprite.width + offset) * x as f32);
                        transform.set_translation_y((sprite.height + offset) * y as f32);
                        i += 1;
                }
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageDeleteSystem;

impl<'s> System<'s> for TwImageDeleteSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwImage>,
                       Entities<'s>);
    fn run(&mut self, (
        input,
        mut world,
        tw_images,
        entities
    ): Self::SystemData) {
        let mut tw_input_handler = world.fetch_mut::<TwInputHandler>();
        for (tw_image, entity) in (&tw_images, &*entities).join() {
            if time::Duration::from_millis(100) <= tw_input_handler.stopwatch.elapsed() {
                if input.key_is_down(VirtualKeyCode::Delete) {
                    if !tw_input_handler.twimages_under_mouse.is_empty() && tw_input_handler.twimages_under_mouse[0].0 == tw_image.id {
                        info!("TwImage is deleting, {:?}", entity);
                        let index = tw_input_handler.twimages_under_mouse.iter().position(|x| x.0 == tw_image.id).unwrap();
                        tw_input_handler.twimages_under_mouse.remove(index);
                        entities.delete(entity).unwrap();
                        tw_input_handler.stopwatch.restart();
                    }
                }
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageToFrontSystem;

impl<'s> System<'s> for TwImageToFrontSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,);
    fn run(&mut self, (
        input,
        mut world,
        tw_images,
        mut transforms,
        sprites
    ): Self::SystemData) {
        let mut tw_input_handler = world.fetch_mut::<TwInputHandler>();
        let mut images = {
            let (img) = (&tw_images).join();
            img.map(|t| t).collect::<Vec<_>>()
        };
        for (tw_image, transform, _) in (&tw_images, &mut transforms, &sprites).join() {
            if time::Duration::from_millis(100) <= tw_input_handler.stopwatch.elapsed() {
                if input.key_is_down(VirtualKeyCode::F) && input.key_is_down(VirtualKeyCode::LShift) {
                    if !tw_input_handler.twimages_under_mouse.is_empty() && tw_input_handler.twimages_under_mouse[0].0 == tw_image.id {
                        images.sort_by(|a, b| a.z_order.cmp(&b.z_order));
                        let current_index = images.iter().position(|x| x.id == tw_image.id).unwrap();
                        let pop = images.swap_remove(current_index);
                        images.push(pop);
                        info!("TwImage is bringing to front, {:?} {:?}", tw_image.id, transform);
                        transform.set_translation_z(images.iter().count() as f32);
                    } else if !tw_input_handler.twimages_under_mouse.is_empty() {
                        transform.set_translation_z(0.0);
                    }
                    tw_input_handler.stopwatch.restart();
//                    info!("{:?} {:?}", transform.translation().z, tw_image.z_order);
//                    info!("{:?}", tw_input_handler.twimages_under_mouse);
                }
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageUpdateZSystem;

impl<'s> System<'s> for TwImageUpdateZSystem {
    type SystemData = (WriteStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>);
    fn run(&mut self, (
        mut tw_images,
        mut transforms
    ): Self::SystemData) {
        for (tw_image, transform) in (&mut tw_images, &mut transforms).join() {
            tw_image.z_order = transform.translation().z as u8;
        }
    }
}
