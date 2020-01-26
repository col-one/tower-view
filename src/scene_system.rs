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
use crate::image::{TwImage, TwActiveUiComponent};


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
                       ReadStorage<'s, SpriteRender>,
                       ReadStorage<'s, Camera>,
                       ReadStorage<'s, TwActiveUiComponent>);

    fn run(&mut self, (
        mut tw_data,
        transforms,
        twimages,
        sprite_sheet,
        sprites,
        cameras,
        actives
    ): Self::SystemData) {
        let count = (&twimages, &transforms).join().count();
        let mut points = Vec::new();
        let mut active_points = Vec::new();
        for (sprite, twimage, transform) in (&sprites, &twimages, &transforms).join() {
            let sprite_sheet = sprite_sheet.get(&sprite.sprite_sheet).unwrap();
            let sprite = &sprite_sheet.sprites[sprite.sprite_number];
            points.push((transform.translation().x, transform.translation().y));
            points.push((transform.translation().x - (sprite.width * 0.5), transform.translation().y - (sprite.height * 0.5)));
            points.push((transform.translation().x + (sprite.width * 0.5), transform.translation().y + (sprite.height * 0.5)));
        }
        for (sprite, active, transform) in (&sprites, &actives, &transforms).join() {
            let sprite_sheet = sprite_sheet.get(&sprite.sprite_sheet).unwrap();
            let sprite = &sprite_sheet.sprites[sprite.sprite_number];
            active_points.push((transform.translation().x, transform.translation().y));
            active_points.push((transform.translation().x - (sprite.width * 0.5), transform.translation().y - (sprite.height * 0.5)));
            active_points.push((transform.translation().x + (sprite.width * 0.5), transform.translation().y + (sprite.height * 0.5)));
        }
        if let Some(bbox) = LineString::from(points).bounding_rect() {
            tw_data.scene_middle_point = Point2::new((bbox.min.x + bbox.max.x) / 2.0, (bbox.min.y + bbox.max.y) / 2.0);
            tw_data.scene_rect = bbox;
        }
        if let Some(bbox_active) = LineString::from(active_points).bounding_rect() {
            tw_data.active_rect = bbox_active;
        }
    }
}
