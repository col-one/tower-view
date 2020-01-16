use structopt::StructOpt;
use amethyst::{
    ecs::prelude::*,
    prelude::*,
    input,
    renderer::rendy::hal::pso::Rect,
    core::transform::Transform,
    winit::dpi::LogicalPosition,
    renderer::types::TextureData
};

use crate::image;
use crate::camera;
use crate::inputshandler;
use crate::image::TwImage;
use crate::args_cli::Opt;
use crate::inputshandler::{get_drop_file, get_moved_mouse};
use crate::placeholder;
use crate::inputshandler::TwInputHandler;
use std::sync::mpsc::{Sender, Receiver};
use std::future::Future;
use std::sync::{Arc, Mutex};
use crate::placeholder::TwPlaceHolder;
use std::ffi::{OsStr, OsString};
use std::collections::HashMap;

use crate::utils::{list_valid_files, is_valid_file};
use std::path::Path;

pub const BACKGROUNDCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOWWIDTH: f32 = 1080.0;
pub const WINDOWHEIGHT: f32 = 720.0;


pub struct TowerData {
    pub twimage_count: f32,
    pub scene_rect: Rect,
    pub cache: Arc<Mutex<HashMap<String, (TwImage, TextureData)>>>,
    pub working_dir: OsString,
    pub file_to_cache: Vec<OsString>,
    pub files_order: Vec<OsString>,
}

impl Default for TowerData {
    fn default() -> Self {
        Self {
            twimage_count: 0.0,
            scene_rect: Rect{x:0i16, y:0i16, w:0i16, h:0i16},
            cache: Arc::new(Mutex::new(HashMap::new())),
            working_dir: OsStr::new(".").to_owned(),
            file_to_cache: Vec::new(),
            files_order: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct Tower {
    pub mouse_position: (f64, f64)
}

impl<'a> SimpleState for Tower {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        // command line arguments
        let opt = Opt::from_args();
        world.insert(opt);
        // load image from inputs arg
        image::load_image_from_inputs_arg(world);
        // init tower data
        let mut tower_data = TowerData::default();
        // get file to cache
        tower_data.file_to_cache = list_valid_files(&world.fetch::<TowerData>().working_dir);
        tower_data.files_order = tower_data.file_to_cache.clone();
        info!("{:?}", tower_data.file_to_cache);
        world.insert(tower_data);
        camera::initialise_camera(world);
        world.register::<TwPlaceHolder>();
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if let Some(drop_file) = get_drop_file(&event) {
                if is_valid_file(Path::new(&drop_file)) {
                    let mut position = Transform::default();
                    position.set_translation_x(self.mouse_position.0 as f32 - WINDOWWIDTH * 0.5);
                    position.set_translation_y(-(self.mouse_position.1 as f32 - WINDOWHEIGHT * 0.5));
                    // todo: use screen to world position.
                    let mut world = data.world;
                    let sprite = placeholder::sprite_twplaceholder(world);
                    placeholder::create_entity_twplaceholder(world, drop_file, position, sprite);
                } else {
                    warn!("Invalid format for {:?}", &drop_file);
                }
            }
            if let Some(mouse_pos) = get_moved_mouse(&event) {
                self.mouse_position = (mouse_pos.x, mouse_pos.y);
            }
        }
        Trans::None
    }
}

