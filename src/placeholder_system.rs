use amethyst::core::{SystemDesc, Transform, math::{Point2, Vector2}, Stopwatch};
use amethyst::derive::SystemDesc;
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::renderer::{camera::{ActiveCamera, Camera, Projection},
                        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
                        resources::Tint,
                        palette::Srgba,
                        types::TextureData,
                        Texture,
                        Sprite, Transparent,
};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::is_mouse_button_down;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;
use std::path::Path;

use crate::placeholder::{TwPlaceHolder, sprite_twplaceholder, create_entity_twplaceholder};
use crate::image::*;
use crate::tower::{TowerData, WINDOWHEIGHT, WINDOWWIDTH};
use crate::inputshandler::TwInputsHandler;
use crate::utils::is_valid_file;
use crate::raycasting_system::screen_to_world;
use std::ffi::OsString;


fn caching_image(mut cache: MutexGuard<'_, HashMap<String, (TwImage, TextureData)>>, path: String) {
    info!("TwImage is loading in cache. {:?}", &path);
    cache.insert(path.clone(), load_texture_from_file(&path));
//    thread::sleep(Duration::from_secs(5));
    info!("TwImage loaded in cache. {:?}", &path);
}


#[derive(SystemDesc)]
pub struct TwImageDroppedSystem;

impl<'s> System<'s> for TwImageDroppedSystem {
    type SystemData = (WriteExpect<'s, TwInputsHandler>,
                       Write<'s, LazyUpdate>,
                       WriteExpect<'s, TowerData>,
                       ReadExpect<'s, Loader>,
                       Read<'s, AssetStorage<Texture>>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       Entities<'s>,
                       ReadStorage<'s, Camera>,
                       ReadStorage<'s, Transform>,
                       ReadExpect<'s, ScreenDimensions>);
    fn run(&mut self, (
        mut tw_in,
        mut world,
        mut tw_data,
        loader,
        texture,
        sprite,
        entities,
        cameras,
        transforms,
        screen_dimensions
    ): Self::SystemData) {
        let mut path_to_load = Vec::new();
        if let Some(drop_file) = &tw_in.last_dropped_file_path.pop() { path_to_load.push(drop_file.clone()) }
        if !tw_data.inputs_path.is_empty() {
            while !tw_data.inputs_path.is_empty() {
                if let Some(path) = tw_data.inputs_path.pop() {
                    // transfer to cache list for next key
                    tw_data.file_to_cache.push(OsString::from(&path));
                    path_to_load.push(path);
                }
            }
        }
        for path in path_to_load {
            if is_valid_file(Path::new(&path)) {
                let (camera, transform) = (&cameras, &transforms).join().next().unwrap();
                let mut position = Transform::default();
                if let Some(mouse_position) = tw_in.mouse_position {
                    let world_position = screen_to_world((mouse_position.0, mouse_position.1),
                                                         camera, transform, &screen_dimensions);
                    position.set_translation_x(world_position.x);
                    position.set_translation_y(world_position.y);
                    position.set_translation_z(world_position.z);
                }
                world.create_entity(&*entities)
                    .with(position)
                    .with(TwPlaceHolder {from_next: false, twimage_path: path.clone(), to_cache: true })
                    .build();
            } else {
                warn!("Invalid format for {:?}", &path);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwPlaceHolderLoadTwImageSystem;

impl<'s> System<'s> for TwPlaceHolderLoadTwImageSystem {
    type SystemData = (WriteStorage<'s, TwPlaceHolder>,
                       Entities<'s>,
                       Write<'s, TowerData>,
                       );
    fn run(&mut self, (
        mut tw_holders,
        entities,
        mut td,
    ): Self::SystemData) {
        for (tw_holder, entity) in (&mut tw_holders, &*entities).join() {
            if tw_holder.to_cache {
                let cache = Arc::clone(&td.cache);
                let path = tw_holder.twimage_path.clone();
                if !cache.lock().unwrap().contains_key(&path) {
                    thread::spawn(move || {caching_image(cache.lock().unwrap(), path);});
                } else {
                    info!("Image already in cache {:?}", path);
                }
                tw_holder.to_cache = false;
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwPlaceHolderCacheSystem;

impl<'s> System<'s> for TwPlaceHolderCacheSystem {
    type SystemData = (Write<'s, TowerData>,
                       );
    fn run(&mut self, (
        mut td,
    ): Self::SystemData) {
        if !td.file_to_cache.is_empty() {
            let path = td.file_to_cache.pop().unwrap();
            let cache = Arc::clone(&td.cache);
            if !cache.lock().unwrap().contains_key(path.to_str().unwrap()) {
                thread::spawn(move || {caching_image(cache.lock().unwrap(), path.to_str().unwrap().to_owned());});
            } else {
                info!("Image already in cache {:?}", path);
            }
        }
    }
}


