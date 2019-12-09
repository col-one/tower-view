use structopt::StructOpt;
use amethyst::{
    ecs::prelude::*,
    prelude::*,
};
use crate::twimage;
use crate::twcamera;
use crate::twinputshandler;
use crate::twimage::TwImage;
use crate::twargs_cli::Opt;


pub const BACKGROUNDCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOWWIDTH: f32 = 1080.0;
pub const WINDOWHEIGHT: f32 = 720.0;



#[derive(Default)]
pub struct Tower;

impl<'a> SimpleState for Tower {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        // command line arguments
        let opt = Opt::from_args();
        world.insert(opt);
        twimage::load_image_from_paths(world);
        //
//        world.entry::<twinputshandler::TwInputHandler>().or_insert_with(|| twinputshandler::TwInputHandler::default());
        //


//        let (mut tw_image, texture_data) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/maxresdefault.jpg");
//        let (mut tw_image2, texture_data2) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/maxresdefault.jpg");
//        let (mut tw_image3, texture_data3) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/maxresdefault.jpg");
//        let (mut tw_image4, texture_data4) = twimage::load_texture_from_file("/home/colin/workspace/tower/assets/texture/maxresdefault.jpg");
//        let sprite_sheet = twimage::create_sprite_sheet(world, texture_data, &tw_image);
//        let sprite_sheet2 = twimage::create_sprite_sheet(world, texture_data2, &tw_image2);
//        let sprite_sheet3 = twimage::create_sprite_sheet(world, texture_data3, &tw_image3);
//        let sprite_sheet4 = twimage::create_sprite_sheet(world, texture_data4, &tw_image4);
//        tw_image.z_order = 0;
//        twimage::create_entity_twimage(world, tw_image, sprite_sheet);
//        tw_image2.z_order = 1;
//        twimage::create_entity_twimage(world, tw_image2, sprite_sheet2);
//        tw_image3.z_order = 3;
//        twimage::create_entity_twimage(world, tw_image3, sprite_sheet3);
//        tw_image3.z_order = 4;
//        twimage::create_entity_twimage(world, tw_image4, sprite_sheet4);

        //
        twcamera::initialise_camera(world);
    }
}
