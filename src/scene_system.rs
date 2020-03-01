/// contains systems related to all the TwImages, could be merged in image_system...
/// and related to the scene
use amethyst::{
    core::{SystemDesc, Transform, math::{Point2, Point3, Vector2}},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::Window,
    renderer::{sprite::{SpriteRender, SpriteSheet}, Camera,
               debug_drawing::{DebugLines},
               palette::Srgba},
    assets::{AssetStorage},
};
use geo::{LineString};
use geo::algorithm::bounding_rect::BoundingRect;



use crate::tower::{TowerData};
use crate::image::{TwImage};
use crate::inputshandler::TwInputsHandler;
use crate::camera::world_to_screen;


#[derive(SystemDesc, Default)]
pub struct SceneBoundingBox;
/// Calculate the scene bounding box mean all images coord
/// also calculate the bbox of the active image
/// each bbox is converted from world to screen coord
/// to fit camera correctly
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
        let _count = (&twimages, &transforms).join().count();
        let mut points = Vec::new();
        let mut active_points = Vec::new();
        let (camera, cam_transform) = (&cameras, &transforms).join().next().unwrap();
        for (sprite, _twimage, transform) in (&sprites, &twimages, &transforms).join() {
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
            let _screen_point_top = world_to_screen(camera, world_point_top, diag.clone(), cam_transform ) * window.get_hidpi_factor() as f32;
            let world_point_bottom = Point3::new(bottom_l_point.0, bottom_l_point.1, 0.0);
            let _screen_point_bottom = world_to_screen(camera, world_point_bottom, diag, cam_transform ) * window.get_hidpi_factor() as f32;
//            tw_data.debug_line_start = world_point_bottom.clone();
//            tw_data.debug_line_end = world_point_top.clone();
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
/// draw GL line from TowerData.debug_line_start and TowerData.debug_line_end.
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

