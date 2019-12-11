use amethyst::core::{SystemDesc, Transform, math::{Point2, Vector2}};
use amethyst::derive::SystemDesc;
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::renderer::{camera::{ActiveCamera, Camera, Projection},
                        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat}, };
use amethyst::assets::{AssetStorage};

use crate::twimage::TwImage;
use crate::twinputshandler::TwInputHandler;
use crate::twutils::point_in_rect;

use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT};
use uuid::Uuid;
use std::{thread, time};


#[derive(SystemDesc)]
pub struct TwImageMoveSystem;

impl<'s> System<'s> for TwImageMoveSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>, );
    fn run(&mut self, (
            input,
            mut world,
            tw_images,
            mut transforms,
            sprites,
            sprite_sheets
        ): Self::SystemData) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        for (sprite, transform, tw_image) in (&sprites, &mut transforms, &tw_images).join() {
            let mouse_world_position = tw_input_handler.mouse_world_pos;
            let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
            let sprite = &sprite_sheet.sprites[sprite.sprite_number];
            let (min_x, max_x, min_y, max_y) = {
                (
                    transform.translation().x - (sprite.width * 0.5),
                    transform.translation().x + (sprite.width * 0.5),
                    transform.translation().y - (sprite.height * 0.5),
                    transform.translation().y + (sprite.height * 0.5),
                )
            };
            if mouse_world_position.x > min_x
                && mouse_world_position.x < max_x
                && mouse_world_position.y > min_y
                && mouse_world_position.y < max_y
            {
                if !tw_input_handler.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                    tw_input_handler.twimages_under_mouse.push((tw_image.id, tw_image.z_order));
                }
            } else {
                if tw_input_handler.twimages_under_mouse.iter().any(|x| x.0 == tw_image.id) {
                    let index = tw_input_handler.twimages_under_mouse.iter().position(|x| x.0 == tw_image.id).unwrap();
                    tw_input_handler.twimages_under_mouse.remove(index);
                }
            }
            tw_input_handler.twimages_under_mouse.sort_by(|a, b| b.1.cmp(&a.1));
            if input.key_is_down(VirtualKeyCode::LAlt) && input.mouse_button_is_down(MouseButton::Left) {
                // set as active image the highest image z order
                if tw_input_handler.twimage_active.is_none() {
                    if !tw_input_handler.twimages_under_mouse.is_empty() && tw_input_handler.twimages_under_mouse[0].0 == tw_image.id {
                        tw_input_handler.set_twimage_active(Some(tw_image.id));
                    }
                }
                // trace vector to move image
                if tw_input_handler.last_mouse_pos.is_none() {
                    let world_pos = {
                        Some((tw_input_handler.mouse_world_pos.x, tw_input_handler.mouse_world_pos.y))
                    };
                    tw_input_handler.set_last_mouse_pos(world_pos);
                }
                if tw_input_handler.twimage_active == Some(tw_image.id) {
                    let world_pos = {
                        Some((tw_input_handler.mouse_world_pos.x, tw_input_handler.mouse_world_pos.y))
                    };
                    let (x, y) = tw_input_handler.last_mouse_pos.unwrap();
                    let (x2, y2) = world_pos.unwrap();
                    let dist = ((x2 - x), (y2 - y));
                    let delta_x = dist.0 - tw_input_handler.last_mouse_dist.0;
                    let delta_y = dist.1 - tw_input_handler.last_mouse_dist.1;
                    tw_input_handler.last_mouse_dist = (dist.0, dist.1);
                    transform.prepend_translation_x(delta_x);
                    transform.prepend_translation_y(delta_y);
                }
            // reset of position data mouse and active image
            } else if input.key_is_down(VirtualKeyCode::LAlt) {
                tw_input_handler.set_last_mouse_pos(None);
                tw_input_handler.set_twimage_active(None);
                tw_input_handler.last_mouse_dist = (0.0, 0.0);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct TwImageLayoutSystem;

impl<'s> System<'s> for TwImageLayoutSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       ReadStorage<'s, TwImage>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, SpriteRender>,
                       Read<'s, AssetStorage<SpriteSheet>>, );
    fn run(&mut self, (
        input,
        tw_images,
        mut transforms,
        sprites,
        sprite_sheets
    ): Self::SystemData) {
        if input.key_is_down(VirtualKeyCode::L) {
            let mut i = 0;
            'out: for x in 0..2 {
                    for y in 0..2 {
                        if i >= tw_images.count() { break 'out }
                        let (tw_image, transform, sprite) = (&tw_images, &mut transforms, &sprites).join().nth(i).unwrap();
                        let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                        let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                        transform.set_translation_x(sprite.width * x as f32);
                        transform.set_translation_y(sprite.height * y as f32);
                        i += 1;
                }
            }
        }
    }
}
