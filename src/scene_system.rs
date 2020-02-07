use amethyst::{
    window::ScreenDimensions,
    prelude::*,
    core::{SystemDesc, Transform, math::{Point, Point2, Point3, Vector2}, Time},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::Window,
    renderer::{sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat}, Camera,
               debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
               palette::Srgba},
    assets::{AssetStorage},
};
use geo::{LineString};
use geo::algorithm::bounding_rect::BoundingRect;

use std::time::Duration;

use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH, TowerData};
use crate::image::{TwImage, TwActiveUiComponent};
use crate::inputshandler::TwInputsHandler;
use crate::camera::world_to_screen;
use cgmath::num_traits::real::Real;


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
                       Read<'s, TwInputsHandler>,
                       ReadExpect<'s, Window>);

    fn run(&mut self, (
        mut tw_data,
        transforms,
        twimages,
        sprite_sheet,
        sprites,
        cameras,
        tw_in,
        window
    ): Self::SystemData) {
        let count = (&twimages, &transforms).join().count();
        let mut points = Vec::new();
        let mut active_points = Vec::new();

        let (camera, cam_transform) = (&cameras, &transforms).join().next().unwrap();


        for (sprite, twimage, transform) in (&sprites, &twimages, &transforms).join() {
            let sprite_sheet = sprite_sheet.get(&sprite.sprite_sheet).unwrap();
            let sprite = &sprite_sheet.sprites[sprite.sprite_number];
            let top_l_point = (transform.translation().x - (sprite.width * 0.5), transform.translation().y + (sprite.height * 0.5));
            let top_r_point = (transform.translation().x + (sprite.width * 0.5), transform.translation().y + (sprite.height * 0.5));
            let bottom_l_point = (transform.translation().x - (sprite.width * 0.5), transform.translation().y - (sprite.height * 0.5));
            let bottom_r_point = (transform.translation().x + (sprite.width * 0.5), transform.translation().y - (sprite.height * 0.5));
            points.push((transform.translation().x, transform.translation().y));
            points.push(top_l_point);
            points.push(top_r_point);
            points.push(bottom_l_point);
            points.push(bottom_r_point);

            let win_size =  window.get_inner_size().unwrap();
            let diag = Vector2::new(win_size.width as f32, win_size.height as f32);
            let world_point_top = Point3::new(top_l_point.0, top_l_point.1, 0.0);
            let screen_point_top = world_to_screen(camera, world_point_top, diag.clone(), cam_transform ) * window.get_hidpi_factor() as f32;
            let world_point_bottom = Point3::new(bottom_l_point.0, bottom_l_point.1, 0.0);
            let screen_point_bottom = world_to_screen(camera, world_point_bottom, diag, cam_transform ) * window.get_hidpi_factor() as f32;
            tw_data.debug_line_start = world_point_bottom.clone();
            tw_data.debug_line_end = world_point_top.clone();
        }
        if let Some(active_entity) = tw_in.active_entities.last() {
            let sprite = &sprites.get(*active_entity).unwrap();
            let transform = &transforms.get(*active_entity).unwrap();
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


#[derive(SystemDesc)]
pub struct DebugLinesSystem;

impl<'s> System<'s> for DebugLinesSystem {
    type SystemData = (
        Write<'s, DebugLines>, // Request DebugLines resource
        Read<'s, TowerData>,
    );

    fn run(&mut self, (mut debug_lines_resource, tw_data): Self::SystemData) {
        debug_lines_resource.draw_line(
            [tw_data.debug_line_start.x, tw_data.debug_line_start.y, 0.0].into(),
            [tw_data.debug_line_end.x, tw_data.debug_line_end.y, 0.0].into(),
            Srgba::new(0.5, 0.05, 0.65, 1.0),
        );
    }
}
