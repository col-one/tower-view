use amethyst::core::{SystemDesc, Transform, math::{Point2, Vector2}, Stopwatch};
use amethyst::derive::SystemDesc;
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::renderer::rendy::{wsi::winit::MouseButton, texture::TextureBuilder};
use amethyst::renderer::{camera::{ActiveCamera, Camera, Projection},
                        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, Sprite},
                        resources::Tint,
                        palette::Srgba,
                        Texture, Transparent,

};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::is_mouse_button_down;

use uuid::Uuid;
use std::{thread, time};

use crate::image::{TwImage, TwActiveUiComponent};
use crate::inputshandler::{TwInputsHandler};
use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT, TowerData};
use crate::placeholder::TwPlaceHolder;

use log;
use std::cmp::Ordering::Equal;
use std::sync::Arc;
use std::path::Path;
use std::ffi::OsString;
use crate::raycasting_system::screen_to_world;


#[derive(SystemDesc)]
pub struct TwImageMoveSystem;

impl<'s> System<'s> for TwImageMoveSystem {
    type SystemData = (WriteStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       Read<'s, TwInputsHandler>);
    fn run(&mut self, (
            mut tw_images,
            mut transforms,
            tw_in,
        ): Self::SystemData) {
        for (transform, tw_image) in (&mut transforms, &mut tw_images).join() {
            if !tw_in.twimages_under_mouse.is_empty() {info!("{:?} {:?}", tw_in.twimages_under_mouse[0].0, tw_image.id)};
            if !tw_in.twimages_under_mouse.is_empty() && tw_in.twimages_under_mouse[0].0 == tw_image.id {
                if let Some(button) = &tw_in.alt_mouse_button_pressed {
                    if let Some(world_pos) = &tw_in.mouse_world_position {
                        if tw_image.mouse_offset.is_none() {
                            let offset = (transform.translation().x - world_pos.0, transform.translation().y - world_pos.1);
                            tw_image.mouse_offset = Some(offset);
                        }
                        if let Some(offset) = tw_image.mouse_offset {
                            transform.set_translation_x(world_pos.0 + offset.0);
                            transform.set_translation_y(world_pos.1 + offset.1);
                        }
                    }
                } else {
                    tw_image.mouse_offset = None
                }
            } else {
                tw_image.mouse_offset = None
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
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, TwImage>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        tw_images,
        entities
    ): Self::SystemData) {
        for (tw_image, entity) in (&tw_images, &*entities).join() {
            if time::Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                if tw_in.keys_pressed.contains(&VirtualKeyCode::Delete) && tw_in.keys_pressed.len() == 1 {
                    if !tw_in.twimages_under_mouse.is_empty() && tw_in.twimages_under_mouse[0].0 == tw_image.id {
                        info!("TwImage is deleting, {:?}", entity);
                        let index = tw_in.twimages_under_mouse.iter().position(|x| x.0 == tw_image.id).unwrap();
                        tw_in.twimages_under_mouse.remove(index);
                        entities.delete(entity).unwrap();
                        tw_in.stopwatch.restart();
                    }
                }
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageToFrontSystem;

impl<'s> System<'s> for TwImageToFrontSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>);
    fn run(&mut self, (
        mut tw_in,
        tw_images,
        mut transforms,
    ): Self::SystemData) {
        let mut images = {
            let (img) = (&tw_images).join();
            let mut images = img.map(|t| t).collect::<Vec<_>>();
            images.sort_by(|a, b| a.z_order.partial_cmp(&b.z_order).unwrap_or(Equal));
            images
        };
        for (tw_image, transform) in (&tw_images, &mut transforms).join() {
            let mut current_index = tw_image.z_order as usize;
            if tw_in.keys_pressed.contains(&VirtualKeyCode::F) && tw_in.keys_pressed.contains(&VirtualKeyCode::LShift) && tw_in.keys_pressed.len() == 2 {
                if time::Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                    if !tw_in.twimages_under_mouse.is_empty() && tw_in.twimages_under_mouse[0].0 == tw_image.id {
                        let i = images.iter().position(|x| x.id == tw_image.id).unwrap();
                        let pop = images.swap_remove(i);
                        images.push(pop);
                        current_index = images.iter().position(|x| x.id == tw_image.id).unwrap();
                        tw_in.stopwatch.restart();
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
                       WriteStorage<'s, TwPlaceHolder>,
                       WriteStorage<'s, Transform>,
                       Write<'s, TowerData>,
                       Entities<'s>,
                       Write<'s, AssetStorage<Texture>>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       WriteExpect<'s, Loader>,
                       Write<'s, LazyUpdate>);
    fn run(&mut self, (
        mut tw_images,
        mut tw_places,
        mut transforms,
        mut tw_data,
        entities,
        mut asset_texture,
        asset_sprite,
        mut loader,
        mut world
    ): Self::SystemData) {
        for (tw_place, transform, entity) in (&mut tw_places, &mut transforms, &*entities).join() {
            let arc_cache = Arc::clone(&tw_data.cache);
            let cache_res = match arc_cache.try_lock() {
                Ok(cache) => Some(cache),
                Err(e) => None
            };
            if !cache_res.is_none() {
                let mut cache = cache_res.unwrap();
                if !cache.is_empty() {
                    let (tw_image, texture_data) = cache.get(&tw_place.twimage_path).unwrap();
                    // create entity
                    let texture_storage = &mut asset_texture;
                    let mut sprites = Vec::with_capacity(1);
                    let loader = &mut loader;
                    let texture = loader.load_from_data(texture_data.clone(), (), &texture_storage);
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
                    let mut transform = transform.clone();
                    transform.set_translation_z(tw_data.twimage_count);
                    let sprite_render = SpriteRender {
                        sprite_sheet: sprite_handle.clone(),
                        sprite_number: 0,
                    };
                    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));
                    world.create_entity(&*entities)
                        .with(transform)
                        .with(sprite_render)
                        .with(tw_image.clone())
                        .with(Transparent)
                        .with(tint)
                        .build();
                    if !tw_place.from_next {
                        tw_data.twimage_count = tw_data.twimage_count + 1.0;
                    }
                    // delete placeholder
                    entities.delete(entity);
                    // set working dir
                    let tw_image_path = tw_image.file_name.clone();
                    tw_data.working_dir = Path::new(&tw_image_path).parent().unwrap().as_os_str().to_owned()
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct TwImageNextSystem;

impl<'s> System<'s> for TwImageNextSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       WriteStorage<'s, TwImage>,
                       Entities<'s>,
                       WriteExpect<'s, TowerData>,
                       Write<'s, LazyUpdate>);
    fn run(&mut self, (
        mut tw_in,
        mut tw_images,
        mut entities,
        mut tower_data,
        world,
    ): Self::SystemData) {
        if let Some((tw_image, entity)) = (&mut tw_images, &*entities).join().last() {
            if let Some(index) = tower_data.files_order.iter().position(|r| r == &OsString::from(&tw_image.file_name)) {
                let mut index = index.clone() as i16;
                let mut new_path = OsString::new();
                if tw_in.keys_pressed.contains(&VirtualKeyCode::Right) && tw_in.keys_pressed.len() == 1 {
                    if time::Duration::from_millis(200) <= tw_in.stopwatch.elapsed() {
                        index += 1;
                        if index < tower_data.files_order.len() as i16 {
                            new_path = tower_data.files_order[index as usize].clone();
                            info!("{:?}", new_path);
                            world.insert(entity, TwPlaceHolder {from_next: true, to_cache: true, twimage_path: new_path.to_str().unwrap().to_owned() });
                            tower_data.file_to_cache.push(new_path);
                        }
                        tw_in.stopwatch.restart();
                    }
                }
                if tw_in.keys_pressed.contains(&VirtualKeyCode::Left) {
                    if time::Duration::from_millis(200) <= tw_in.stopwatch.elapsed() {
                        index -= 1;
                        if index >= 0 {
                            new_path = tower_data.files_order[index as usize].clone();
                            info!("{:?}", new_path);
                            world.insert(entity, TwPlaceHolder {from_next: true, to_cache: true, twimage_path: new_path.to_str().unwrap().to_owned() });
                            tower_data.file_to_cache.push(new_path);
                        }
                        tw_in.stopwatch.restart();
                    }
                }
            }
        }
    }
}
