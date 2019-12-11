use structopt::StructOpt;
use amethyst::{
    ecs::prelude::*,
    prelude::*,
    input,
    renderer::rendy::hal::pso::Rect,
};

use crate::twimage;
use crate::twcamera;
use crate::twinputshandler;
use crate::twimage::TwImage;
use crate::twargs_cli::Opt;
use crate::twinputshandler::get_drop_file;


pub const BACKGROUNDCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOWWIDTH: f32 = 1080.0;
pub const WINDOWHEIGHT: f32 = 720.0;


pub struct TowerData {
    pub twimage_count: u8,
    pub scene_rect: Rect,
}

impl Default for TowerData {
    fn default() -> Self {
        Self {
            twimage_count: 0u8,
            scene_rect: Rect{x:0i16, y:0i16, w:0i16, h:0i16},
        }
    }
}

#[derive(Default)]
pub struct Tower;

impl<'a> SimpleState for Tower {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        // command line arguments
        let opt = Opt::from_args();
        world.insert(opt);
        // init tower data
        let mut tower_data = TowerData{twimage_count: 0u8, scene_rect: Rect{x:0i16, y:0i16, w:0i16, h:0i16}};
        world.insert(tower_data);
        // load image from inputs arg
        twimage::load_image_from_inputs_arg(world);
        twcamera::initialise_camera(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if let Some(drop_file) = get_drop_file(&event) {
                let mut world = data.world;
                twimage::load_image_from_path(world, &drop_file);
            }
        }
        Trans::None
    }
}

