/// image_system.rs contains all the image related systems
/// Most of the systems use TwActiveComponent created and removed by the raytracing system
/// TwActiveComponent is attached to the active TwImage which the one has the mouse on it
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::input::{VirtualKeyCode};
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::renderer::{sprite::{SpriteRender, SpriteSheet, Sprite},
                        resources::Tint,
                        palette::Srgba,
                        Texture, Transparent,

};
use amethyst::assets::{AssetStorage, Loader};


use std::{time};

use crate::image::{TwImage, TwActiveComponent};
use crate::inputshandler::{TwInputsHandler};
use crate::tower::{TowerData};
use crate::placeholder::TwPlaceHolder;

use std::cmp::Ordering::Equal;
use std::sync::Arc;
use std::ffi::OsString;
use std::ops::Index;


#[derive(SystemDesc, Default)]
pub struct TwImageMoveSystem {
    click_offset: Option<(f32, f32)>
}
/// Move the active TwImage, it match the world mouse coord and save the image offset to avoid
/// the centering of the image under the mouse
/// self.active_busy is useful to avoid the move of an other image if the mouse enter in
/// during the move of the current active image
impl<'s> System<'s> for TwImageMoveSystem {
    type SystemData = (WriteStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       WriteExpect<'s, TwInputsHandler>,
                       ReadStorage<'s, TwActiveComponent>);
    fn run(&mut self, (
            _tw_images,
            mut transforms,
            mut tw_in,
            _tw_actives,
        ): Self::SystemData) {
        if let Some(_button) = &tw_in.alt_mouse_button_pressed {
            tw_in.active_busy = true;
            if let Some(top_active_entity) = tw_in.active_entities.last() {
                if let Some(world_pos) = &tw_in.mouse_world_position {
                    if self.click_offset.is_none() {
                        let offset = (transforms.get(*top_active_entity).unwrap().translation().x - world_pos.0,
                                      transforms.get(*top_active_entity).unwrap().translation().y - world_pos.1);
                        self.click_offset = Some(offset);
                    }
                    if let Some(offset) = self.click_offset {
                        let trans = transforms.get_mut(*top_active_entity).unwrap();
                        trans.set_translation_x(world_pos.0 + offset.0);
                        trans.set_translation_y(world_pos.1 + offset.1);
                        debug!("Image is moved of {:?}", (world_pos.0 + offset.0, world_pos.1 + offset.1));
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
/// spread all images as a grid, try to get a square grid as mush as possible.
/// get the ceil of the sqrt of the images count to defined the size of the grid.
/// the size of each cell is the max height and max width from all images
/// An offset is apply between each cell
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
            // get the ceil of the sqrt of the images count, to defined the size of the grid
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
            for _j in 0..twimage_count as usize {
                let (_tw_image, _transform, sprite, entity) = comp_iter.next().unwrap();
                join_entities.push(entity);
                let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                sprite_heights.push(sprite.height);
                sprite_widths.push(sprite.width);
            }
            sprite_heights.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal));
            sprite_widths.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal));
            // TODO: offset as settings
            let offset = 10.0;
            let mut i = 0;
            for x in 0..xy_limit as usize {
                for y in 0..xy_limit as usize {
                    if i >= twimage_count as usize { continue }
                    let e = join_entities[i];
                    let transform = transforms.get_mut(e).unwrap();
                    transform.set_translation_x((sprite_widths.last().unwrap() + offset) * x as f32);
                    transform.set_translation_y((sprite_heights.last().unwrap() + offset) * y as f32);
                    i += 1;
                }
            }
            debug!("Images are layout as an atlas with an offset of {:?}", offset);
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct TwImageRotateSystem;
/// rotate 90 degree clockwise the active image
impl<'s> System<'s> for TwImageRotateSystem {
    type SystemData = (WriteExpect<'s, TwInputsHandler>,
                       WriteExpect<'s, TowerData>,
                       WriteStorage<'s, Transform>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        mut tw_data,
        mut transforms,
        entities
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::R) && tw_in.keys_pressed.len() == 1 {
            if time::Duration::from_millis(5000) <= tw_in.stopwatch.elapsed() {
                if let Some(active_entity) = tw_in.active_entities.last() {
                    debug!("TwImage is rotating, {:?}", active_entity);
                    let trans = transforms.get_mut(*active_entity).unwrap();
                    trans.append_rotation_z_axis(90.0_f32.to_radians());
                    // time offset
                    tw_in.keys_pressed.remove(0);
                }
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageDeleteSystem;
/// delete the active image more precisely the entity
/// and clean the active_entities vector and also the z_ordered_entities in case of two images
/// are stack each other.
impl<'s> System<'s> for TwImageDeleteSystem {
    type SystemData = (WriteExpect<'s, TwInputsHandler>,
                       WriteExpect<'s, TowerData>,
                       Entities<'s>);
    fn run(&mut self, (
        mut tw_in,
        mut tw_data,
        entities
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Delete) && tw_in.keys_pressed.len() == 1 {
            if time::Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                if let Some(active_entity) = tw_in.active_entities.last() {
                    debug!("TwImage is deleting, {:?}", active_entity);
                    entities.delete(*active_entity).expect("Fail error to delete entity");
                    // clean entities copies in tw_in and tw_data
                    tw_in.active_entities.clear();
                    tw_in.z_ordered_entities.clear();
                    tw_in.stopwatch.restart();
                    tw_data.twimage_count -= 1.0;
                }
            }
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct TwImageToFrontSystem;
/// bring the active image as the highest z value. It bring on top of all others.
/// To keep a consistent z order of the other images, all the images are reordered according to the
/// new z order.
/// each z order is multiply by a factor, 0.001
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
                let current_index = tw_in.z_ordered_entities.iter().position(|e| e == &entity).unwrap();
                // TODO: z factor value as settings
                transform.set_translation_z(current_index as f32 * 0.001);
                debug!("TwImage {:?} is bring to front of the other by move its z value. The z_ordered_entities is reorder by the new z value", tw_image);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageApplyBlendingSystem;
/// apply the different channel value, attribute of TwImage, to the associated Tint component
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
/// create TwImage from the TowerData cache. For each TwPlaceHolder TextureData and TwImage is retrieve
/// from cache, then sprite and transform is created
/// then the entity's PlaceHolder is deleted
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
        _tw_images,
        mut tw_places,
        mut transforms,
        mut tw_data,
        entities,
        mut asset_texture,
        asset_sprite,
        mut loader,
        world,
        mut tw_in
    ): Self::SystemData) {
        for (tw_place, transform, entity) in (&mut tw_places, &mut transforms, &*entities).join() {
            let arc_cache = Arc::clone(&tw_data.cache);
            let cache_res = match arc_cache.try_lock() {
                Ok(cache) => Some(cache),
                Err(_e) => None
            };
            if !cache_res.is_none() {
                let cache = cache_res.unwrap();
                if !cache.is_empty() {
                    if let Some((tw_image, texture_data)) = cache.get(&tw_place.twimage_path) {
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
                        transform.set_translation_z(tw_data.twimage_count * 0.001);
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
                        entities.delete(entity).expect("Failed to delete entity.");
                        tw_in.z_ordered_entities.clear();
                        tw_in.active_entities.clear();
                        debug!("TwPlaceHolder is replaced by the TwImage from cache.")
                    }
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct TwImageNextSystem;
/// when arrows key are pushed, the current entity's TwImage is deleted and a new TwPlaceHolder
/// is built. The TwPlaceHolder image path is retrieve from the TowerData.files_order vec this path
/// is send to th cache list too.
/// Right arrow get next image
/// Left arrow get previous image
impl<'s> System<'s> for TwImageNextSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, TwImage>,
                       Entities<'s>,
                       WriteExpect<'s, TowerData>,
                       Write<'s, LazyUpdate>);
    fn run(&mut self, (
        mut tw_in,
        tw_images,
        _entities,
        mut tower_data,
        world,
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::Right) && tw_in.keys_pressed.len() == 1 {
            if time::Duration::from_millis(200) <= tw_in.stopwatch.elapsed() {
                if let Some(active_entity) = tw_in.active_entities.last() {
                    let tw_image = tw_images.get(*active_entity).unwrap();
                    if let Some(index) = tower_data.files_order.iter().position(|r| r == &OsString::from(&tw_image.file_name)) {
                        let mut index = index.clone() as i16;
                        index += 1;
                        if index < tower_data.files_order.len() as i16 {
                            let new_path = tower_data.files_order[index as usize].clone();
                            debug!("Next TwImage is loading {:?}", new_path);
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
                        index -= 1;
                        if index >= 0 {
                            let new_path = tower_data.files_order[index as usize].clone();
                            debug!("Next TwImage is loading {:?}", new_path);
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
