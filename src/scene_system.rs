use amethyst::{
    window::ScreenDimensions,
    prelude::*,
    core::{SystemDesc, Transform, math::{Point, Point2, Point3, Vector2}},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::Window,
    renderer::{sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat}, Camera},
    assets::{AssetStorage},
    renderer::rendy::hal::pso::Rect,
};


use std::time::Duration;

use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH, TowerData};
use crate::inputshandler::TwInputHandler;
use crate::image::TwImage;


#[derive(SystemDesc)]
pub struct SceneBoundingBox;

impl<'s> System<'s> for SceneBoundingBox {
    type SystemData = (Write<'s, World>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, TwImage>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       ReadStorage<'s, Camera>);

    fn run(&mut self, (
        mut world,
        transforms,
        twimages,
        sprite_sheet,
        cameras
    ): Self::SystemData) {
        let mut tw_input_handler = world.fetch_mut::<TwInputHandler>();
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for (twimage, transform) in (&twimages, &transforms).join() {
            sum_x += transform.translation().x;
            sum_y += transform.translation().y;
        }
        tw_input_handler.middlepoint = Point2::new((sum_x / transforms.count() as f32),
                                                   (sum_y / transforms.count() as f32));
    }
}
