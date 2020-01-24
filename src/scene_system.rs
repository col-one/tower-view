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
};
use geo::{LineString};
use geo::algorithm::bounding_rect::BoundingRect;

use std::time::Duration;

use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH, TowerData};
use crate::image::TwImage;


#[derive(SystemDesc, Default)]
pub struct SceneBoundingBox {
    sum_x: f32,
    sum_y: f32,
}

impl<'s> System<'s> for SceneBoundingBox {
    type SystemData = (Write<'s, TowerData>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, TwImage>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       ReadStorage<'s, Camera>);

    fn run(&mut self, (
        mut tw_data,
        transforms,
        twimages,
        sprite_sheet,
        cameras
    ): Self::SystemData) {
        self.sum_x = 0.0;
        self.sum_y = 0.0;
        let count = (&twimages, &transforms).join().count();
        let mut points = Vec::new();
        for (twimage, transform) in (&twimages, &transforms).join() {
            self.sum_x += transform.translation().x;
            self.sum_y += transform.translation().y;
            points.push((transform.translation().x, transform.translation().y));
        }
        let bbox = LineString::from(points).bounding_rect().unwrap();
        tw_data.scene_middle_point = Point2::new((bbox.min.x + bbox.max.x) / 2.0, (bbox.min.y + bbox.max.y) / 2.0);
        tw_data.scene_rect = bbox;
    }
}
