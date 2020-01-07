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

use crate::placeholder::TwPlaceHolder;
use crate::image::*;
use crate::tower::TowerData;
use std::sync::{Arc, Mutex, MutexGuard};

fn caching_image(mut cache: MutexGuard<'_, Vec<(TwImage, TextureData)>>, path: String) {
    info!("TwImage is loading in cache.");
    cache.push(load_texture_from_file(&path));
    thread::sleep(Duration::from_secs(5));
    info!("TwImage loaded in cache.");
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
        let mut holder_to_del = Vec::new();
        for (tw_holder, entity) in (&mut tw_holders, &*entities).join() {
            if tw_holder.to_cache {
                let cache = Arc::clone(&td.cache);
                let path = tw_holder.twimage_path.clone();
                thread::spawn(move || {caching_image(cache.lock().unwrap(), path);});
                holder_to_del.push(entity);
                tw_holder.to_cache = false;
            }
        }
    }
}
