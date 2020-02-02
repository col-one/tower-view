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

use crate::image::{TwImage, TwActiveUiComponent, TwActiveComponent};
use crate::inputshandler::{TwInputsHandler};
use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT, TowerData};
use crate::placeholder::TwPlaceHolder;

use log;
use std::cmp::Ordering::Equal;
use std::sync::Arc;
use std::path::Path;
use std::ffi::OsString;
use crate::raycasting_system::screen_to_world;


#[derive(SystemDesc, Default)]
pub struct TwImageMoveSystem {
    click_offset: Option<(f32, f32)>
}

impl<'s> System<'s> for TwImageMoveSystem {
    type SystemData = (WriteStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       WriteExpect<'s, TwInputsHandler>,
                       ReadStorage<'s, TwActiveComponent>);
    fn run(&mut self, (
            mut tw_images,
            mut transforms,
            mut tw_in,
            tw_actives,
        ): Self::SystemData) {
        if let Some(button) = &tw_in.alt_mouse_button_pressed {
            tw_in.active_busy = true;
            if let Some(top_active_entity) = tw_in.active_entities.last() {
                if let Some(world_pos) = &tw_in.mouse_world_position {
                    if self.click_offset.is_none() {
                        let offset = (transforms.get(*top_active_entity).unwrap().translation().x - world_pos.0,
                                      transforms.get(*top_active_entity).unwrap().translation().y - world_pos.1);
                        self.click_offset = Some(offset);
                    }
                    if let Some(offset) = self.click_offset {
                        let mut trans = transforms.get_mut(*top_active_entity).unwrap();
                        trans.set_translation_x(world_pos.0 + offset.0);
                        trans.set_translation_y(world_pos.1 + offset.1);
                    }
                }
            }
        } else {
            self.click_offset = None;
            tw_in.active_busy = false;
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct TwImageLayoutSystem;

impl<'s> System<'s> for TwImageLayoutSystem {
    type SystemData = (Read<'s, TwInputsHandler>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       Entities<'s>);
    fn run(&mut self, (
        tw_in,
        tw_images,
        mut transforms,
        sprites,
        sprite_sheets,
        entities
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::L) && tw_in.keys_pressed.len() == 1 {
            let twimage_count = tw_images.count() as f32;
            let xy_limit = match twimage_count.sqrt().ceil() {
                xy_limit if xy_limit < 2.0 => 2.0,
                _ => twimage_count.sqrt().ceil()
            };
            let mut comp_iter = (&tw_images, &mut transforms, &sprites, &*entities).join();
            let mut sprite_heights = Vec::new();
            let mut sprite_widths = Vec::new();
            let mut join_entities = Vec::new();
            // get max width and height
            for j in 0..twimage_count as usize {
                let (tw_image, transform, sprite, entity) = comp_iter.next().unwrap();
                join_entities.push(entity);
                let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                sprite_heights.push(sprite.height);
                sprite_widths.push(sprite.width);
            }
            sprite_heights.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal));
            sprite_widths.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal));

            let offset = 10.0;
            let mut i = 0;
            info!("{:?}", xy_limit);
            'out: for x in 0..xy_limit as usize {
                    for y in 0..xy_limit as usize {
                        if i >= twimage_count as usize { continue }
                        let e = join_entities[i];
                        let transform = transforms.get_mut(e).unwrap();
                        transform.set_translation_x((sprite_widths.last().unwrap() + offset) * x as f32);
                        transform.set_translation_y((sprite_heights.last().unwrap() + offset) * y as f32);
                        i += 1;
                }
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageDeleteSystem;

impl<'s> System<'s> for TwImageDeleteSystem {
    type SystemData = (WriteExpect<'s, TwInputsHandler>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        entities
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Delete) && tw_in.keys_pressed.len() == 1 {
            if time::Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                if let Some(active_entity) = tw_in.active_entities.last() {
                    info!("TwImage is deleting, {:?}", active_entity);
                    entities.delete(*active_entity).expect("Fail error to delete entity");
                    // clean entities copies in tw_in
                    tw_in.active_entities.clear();
                    tw_in.z_ordered_entities.clear();
                    tw_in.stopwatch.restart();
                }
            }
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct TwImageToFrontSystem;

impl<'s> System<'s> for TwImageToFrontSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       WriteStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        mut tw_images,
        mut transforms,
        entities,
    ): Self::SystemData) {
        for (tw_image, transform, entity) in (&mut tw_images, &mut transforms, &*entities).join() {
            let mut current_index = tw_image.z_order as usize;
            if tw_in.keys_pressed.contains(&VirtualKeyCode::T) && tw_in.keys_pressed.contains(&VirtualKeyCode::LShift) && tw_in.keys_pressed.len() == 2 {
                if time::Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                    if let Some(active_entity) = tw_in.active_entities.last() {
                        if *active_entity == entity {
                            let i = tw_in.z_ordered_entities.iter().position(|e| e == &entity).unwrap();
                            let pop = tw_in.z_ordered_entities.remove(i);
                            tw_in.z_ordered_entities.push(pop);
                            tw_in.stopwatch.restart();
                        }
                    }
                }
                current_index = tw_in.z_ordered_entities.iter().position(|e| e == &entity).unwrap();
                transform.set_translation_z(current_index as f32 * 0.001);
            }
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
                       Write<'s, LazyUpdate>,
                       WriteExpect<'s, TwInputsHandler>);
    fn run(&mut self, (
        mut tw_images,
        mut tw_places,
        mut transforms,
        mut tw_data,
        entities,
        mut asset_texture,
        asset_sprite,
        mut loader,
        mut world,
        mut tw_in
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
                    tw_in.z_ordered_entities.clear();
                    tw_in.active_entities.clear();
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
                       ReadStorage<'s, TwImage>,
                       Entities<'s>,
                       WriteExpect<'s, TowerData>,
                       Write<'s, LazyUpdate>);
    fn run(&mut self, (
        mut tw_in,
        tw_images,
        mut entities,
        mut tower_data,
        world,
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Right) && tw_in.keys_pressed.len() == 1 {
            if time::Duration::from_millis(200) <= tw_in.stopwatch.elapsed() {
                if let Some(active_entity) = tw_in.active_entities.last() {
                    let tw_image = tw_images.get(*active_entity).unwrap();
                    if let Some(index) = tower_data.files_order.iter().position(|r| r == &OsString::from(&tw_image.file_name)) {
                        let mut index = index.clone() as i16;
                        let mut new_path = OsString::new();
                        index += 1;
                        if index < tower_data.files_order.len() as i16 {
                            new_path = tower_data.files_order[index as usize].clone();
                            info!("{:?}", new_path);
                            world.insert(*active_entity, TwPlaceHolder { from_next: true, to_cache: true, twimage_path: new_path.to_str().unwrap().to_owned() });
                            tower_data.file_to_cache.push(new_path);
                        }
                    }
                }
            }
            tw_in.stopwatch.restart();
        }
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Left) && tw_in.keys_pressed.len() == 1 {
            if time::Duration::from_millis(200) <= tw_in.stopwatch.elapsed() {
                if let Some(active_entity) = tw_in.active_entities.last() {
                    let tw_image = tw_images.get(*active_entity).unwrap();
                    if let Some(index) = tower_data.files_order.iter().position(|r| r == &OsString::from(&tw_image.file_name)) {
                        let mut index = index.clone() as i16;
                        let mut new_path = OsString::new();
                        index -= 1;
                        if index >= 0 {
                            new_path = tower_data.files_order[index as usize].clone();
                            info!("{:?}", new_path);
                            world.insert(*active_entity, TwPlaceHolder { from_next: true, to_cache: true, twimage_path: new_path.to_str().unwrap().to_owned() });
                            tower_data.file_to_cache.push(new_path);
                        }
                    }
                }
            }
            tw_in.stopwatch.restart();
        }
    }
}
