use amethyst::core::{SystemDesc, Transform, math::{Point2, Vector2}, Stopwatch};
use amethyst::derive::SystemDesc;
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::renderer::{camera::{ActiveCamera, Camera, Projection},
                        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, Sprite},
                        resources::Tint,
                        palette::Srgba,
                        Texture, Transparent
};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::is_mouse_button_down;

use uuid::Uuid;
use std::{thread, time};

use crate::image::{TwImage, TwActiveComponent};

use crate::inputshandler::{MouseState, TwInputHandler};
use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT, TowerData};
use log;
use std::cmp::Ordering::Equal;
use std::sync::Arc;


#[derive(SystemDesc)]
pub struct TwImageActiveSystem;

impl<'s> System<'s> for TwImageActiveSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       WriteStorage<'s, TwImage>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       WriteStorage<'s, TwActiveComponent>,
                       Entities<'s>);
    fn run(&mut self, (
        input,
        mut world,
        mut tw_images,
        transforms,
        sprites,
        sprite_sheets,
        mut twactives,
        entities
    ): Self::SystemData) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        let mut remove_active = false;
        for (sprite, transform, tw_image, entity) in (&sprites, &transforms, &mut tw_images, &*entities).join() {
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
                    tw_input_handler.twimages_under_mouse.push((tw_image.id, transform.translation().z));
                }
            } else {
                if tw_input_handler.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                    let index = tw_input_handler.twimages_under_mouse.iter().position(|x| x.0 == tw_image.id).unwrap();
                    tw_input_handler.twimages_under_mouse.remove(index);
                }
            }
            // set as active image the highest image z order
            tw_input_handler.twimages_under_mouse.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Equal));
            if tw_input_handler.twimages_under_mouse.is_empty() {
                tw_input_handler.set_twimage_active(None);
            } else {
                if tw_input_handler.twimages_under_mouse[0].0 == tw_image.id {
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
        if input.key_is_down(VirtualKeyCode::Escape) { remove_active = true }
        let mut entities_to_remove = Vec::new();
        for (twactive, entity) in (&mut twactives, &*entities).join() {
            if tw_input_handler.twimages_under_mouse.is_empty() {
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
            let mut images = img.map(|t| t).collect::<Vec<_>>();
            images.sort_by(|a, b| a.z_order.partial_cmp(&b.z_order).unwrap_or(Equal));
            images
        };
        for (tw_image, transform, _) in (&tw_images, &mut transforms, &sprites).join() {
            let mut current_index = tw_image.z_order as usize;
            if input.key_is_down(VirtualKeyCode::F) && input.key_is_down(VirtualKeyCode::LShift) {
                if time::Duration::from_millis(500) <= tw_input_handler.stopwatch.elapsed() {
                    if !tw_input_handler.twimages_under_mouse.is_empty() && tw_input_handler.twimages_under_mouse[0].0 == tw_image.id {
                        let i = images.iter().position(|x| x.id == tw_image.id).unwrap();
                        let pop = images.swap_remove(i);
                        images.push(pop);
                        current_index = images.iter().position(|x| x.id == tw_image.id).unwrap();
                        tw_input_handler.stopwatch.restart();
                    }
                }
            }
            current_index = images.iter().position(|x| x.id == tw_image.id).unwrap();
            transform.set_translation_z(current_index as f32);
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageApplyBlendingSystem;

impl<'s> System<'s> for TwImageApplyBlendingSystem {
    type SystemData = (ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Tint>,);
    fn run(&mut self, (
        tw_images,
        mut tints
    ): Self::SystemData) {
        for (tw_image, tint) in (&tw_images, &mut tints).join() {
            *tint = Tint(Srgba::new((tw_image.red).powf(1.0 / 2.2),
                                    (tw_image.green).powf(1.0 / 2.2),
                                    (tw_image.blue).powf(1.0 / 2.2),
                                    (tw_image.alpha).powf(1.0 / 2.2)));
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageLoadFromCacheSystem;

impl<'s> System<'s> for TwImageLoadFromCacheSystem {
    type SystemData = (WriteStorage<'s, TwImage>,
                       Write<'s, TowerData>,
                       Entities<'s>,
                       Write<'s, AssetStorage<Texture>>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       WriteExpect<'s, Loader>,
                       Write<'s, LazyUpdate>);
    fn run(&mut self, (
        mut tw_images,
        mut td,
        entities,
        mut asset_texture,
        asset_sprite,
        mut loader,
        mut world
    ): Self::SystemData) {
        let arc_cache = Arc::clone(&td.cache);
        let cache_res = match arc_cache.try_lock() {
            Ok(cache) => Some(cache),
            Err(e) => None
        };
        if !cache_res.is_none() {
            let mut cache = cache_res.unwrap();
            if !cache.is_empty() {
                let (tw_image, texture_data) = cache.pop().unwrap();
                // create entity
                let texture_storage = &mut asset_texture;
                let mut sprites = Vec::with_capacity(1);
                let loader = &mut loader;
                let texture = loader.load_from_data(texture_data, (), &texture_storage);
                let sprite = Sprite::from_pixel_values(
                    tw_image.width, tw_image.height, tw_image.width,
                    tw_image.height, 0, 0, [0.0, 0.0],
                    false, false,
                );
                sprites.push(sprite);
                let sprite_sheet = SpriteSheet {
                    texture,
                    sprites,
                };
                let sprite_handle = loader.load_from_data(
                    sprite_sheet,
                    (),
                    &asset_sprite,
                );
                let mut transform = Transform::default();
                transform.set_translation_x(0.0);
                transform.set_translation_y(0.0);
                transform.set_translation_z(td.twimage_count);
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_handle.clone(),
                    sprite_number: 0,
                };
                let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));
                world.create_entity(&*entities)
                    .with(transform)
                    .with(sprite_render)
                    .with(tw_image)
                    .with(Transparent)
                    .with(tint)
                    .build();
                td.twimage_count = td.twimage_count + 1.0;
            }
        }
    }
}