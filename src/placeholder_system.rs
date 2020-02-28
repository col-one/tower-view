use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;

use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::window::ScreenDimensions;

use amethyst::renderer::{camera::{Camera},
                        sprite::{SpriteSheet},
                        Texture,
};
use amethyst::assets::{AssetStorage, Loader};

use std::sync::Arc;
use std::path::Path;
use std::ffi::OsString;
use std::thread;

use crate::placeholder::{TwPlaceHolder};
use crate::image::*;
use crate::tower::{TowerData};
use crate::inputshandler::TwInputsHandler;
use crate::utils::is_valid_file;
use crate::raycasting_system::screen_to_world;


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
        world,
        mut tw_data,
        _loader,
        _texture,
        _sprite,
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
                    tw_data.file_to_cache.insert(0, OsString::from(&path));
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


#[derive(SystemDesc, Default)]
pub struct TwCachingImages {
    // dirty hack to avoid the directory caching to be the first to cache image rather than the
    // the cache TwHolder, idk yet tw_holder.count is 0 while execution order seems good:
    // - add inputs to TowerData > dropped system to create holder > TwCachingSystem
    // I guess is the LazyUpdate from TwImageDroppedSystem which create the un-sync
    pub ready_to_cache: bool
}

impl<'s> System<'s> for TwCachingImages {
    type SystemData = (WriteStorage<'s, TwPlaceHolder>,
                       Entities<'s>,
                       Write<'s, TowerData>,
                       );
    fn run(&mut self, (
        mut tw_holders,
        entities,
        mut td,
    ): Self::SystemData) {
        for (tw_holder, _entity) in (&mut tw_holders, &*entities).join() {
            if tw_holder.to_cache {
                let cache = Arc::clone(&td.cache);
                let path = tw_holder.twimage_path.clone();
                thread::spawn(move || {caching_image(cache.lock().unwrap(), path);});
                tw_holder.to_cache = false;
                // dirty hack
                self.ready_to_cache = true;
            }
        }
        if tw_holders.count() == 0 && self.ready_to_cache && !td.file_to_cache.is_empty() {
            let path = td.file_to_cache.pop().unwrap();
            let cache = Arc::clone(&td.cache);
            thread::spawn(move || {caching_image(cache.lock().unwrap(), path.to_str().unwrap().to_owned());});
        }
    }
}

