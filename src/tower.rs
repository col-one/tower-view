use amethyst::{
    ecs::prelude::*,
    prelude::*,
};
use crate::twimage;
use crate::twcamera;
use crate::twinputshandler;

pub const BACKGROUNDCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOWWIDTH: f32 = 1080.0;
pub const WINDOWHEIGHT: f32 = 720.0;



#[derive(Default)]
pub struct Tower;

impl SimpleState for Tower {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<twimage::TwImage>();
        world.entry::<twinputshandler::TwInputHandler>().or_insert_with(|| twinputshandler::TwInputHandler::default());
        //
        let (tw_image, texture_data) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/maxresdefault.jpg");
        let sprite_sheet = twimage::create_sprite_sheet(world, texture_data, &tw_image);
        twimage::create_entity_twimage(world, tw_image, sprite_sheet);
        //
        twcamera::initialise_camera(world);
    }
}
