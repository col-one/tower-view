use amethyst::{
    window::ScreenDimensions,
    prelude::*,
    core::{SystemDesc, Transform, math::{Point, Point3, Vector2}},
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


#[derive(SystemDesc)]
pub struct SceneBoundingBox;

impl<'s> System<'s> for SceneBoundingBox {
    type SystemData = (Write<'s, World>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>,
                       ReadStorage<'s, Camera>);

    fn run(&mut self, (
        mut world,
        transforms,
        sprite_renders,
        sprite_sheet,
        cameras
    ): Self::SystemData) {
        let mut td = world.entry::<TowerData>().or_insert_with(|| TowerData::default());
        let (camera_transform, camera) = (&transforms, &cameras).join().next().unwrap();
        td.scene_rect = Rect{x:0i16, y:0i16, w:0i16, h:0i16};
        for (sprite_render, transform) in (&sprite_renders, &transforms).join() {
            let sprite_sheet = sprite_sheet.get(&sprite_render.sprite_sheet).unwrap();
            let sprite = &sprite_sheet.sprites[sprite_render.sprite_number];
            let twimage_rect = Rect{x: transform.translation().x as i16 - (sprite.width / 2.0) as i16,
                                    y: transform.translation().y as i16 - (sprite.height / 2.0) as i16,
                                    w: transform.translation().x as i16 + (sprite.width / 2.0) as i16,
                                    h: transform.translation().y as i16 + (sprite.height / 2.0) as i16,};
            let x = match twimage_rect.x {
                rect if rect <= td.scene_rect.x => rect,
                _ => td.scene_rect.x
            };
            let y = match twimage_rect.y {
                rect if rect <= td.scene_rect.y => rect,
                _ => td.scene_rect.y
            };
            let w = match twimage_rect.w {
                rect if rect >= td.scene_rect.w => (rect + td.scene_rect.w).abs(),
                _ => td.scene_rect.w
            };
            let h = match twimage_rect.h {
                rect if rect >= td.scene_rect.h => (rect + td.scene_rect.h).abs(),
                _ => td.scene_rect.h
            };
            td.scene_rect = Rect{x, y, w, h};
            let diag = Vector2::new(WINDOWWIDTH, WINDOWHEIGHT);
            let dd = camera.projection().world_to_screen(Point3::new(td.scene_rect.x as f32, td.scene_rect.y as f32, 0.0), diag, camera_transform);
        }
    }
}
