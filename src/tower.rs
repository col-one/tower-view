use amethyst::{
    ecs::prelude::*,
    prelude::*,
};
use crate::twimage;
use crate::twcamera;
use crate::twinputshandler;
use crate::twimage::TwImage;

pub const BACKGROUNDCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOWWIDTH: f32 = 1080.0;
pub const WINDOWHEIGHT: f32 = 720.0;



#[derive(Default)]
pub struct Tower {
//    tw_images: Vec<&'a TwImage>,
}

impl<'a> SimpleState for Tower {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<twimage::TwImage>();
        world.entry::<twinputshandler::TwInputHandler>().or_insert_with(|| twinputshandler::TwInputHandler::default());
        //
        let (mut tw_image, texture_data) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/maxresdefault.jpg");
        let (mut tw_image2, texture_data2) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/logo_abc.png");
        let sprite_sheet = twimage::create_sprite_sheet(world, texture_data, &tw_image);
        let sprite_sheet2 = twimage::create_sprite_sheet(world, texture_data2, &tw_image2);
        tw_image.z_order = 0;
        twimage::create_entity_twimage(world, tw_image, sprite_sheet);
//        self.tw_images.push(&tw_image);
        tw_image2.z_order = 1;
        twimage::create_entity_twimage(world, tw_image2, sprite_sheet2);
//        self.tw_images.push(&tw_image2);

        //
        twcamera::initialise_camera(world);
    }
}
