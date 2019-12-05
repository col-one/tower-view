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
//use cgmath::{Matrix4, Point3, Vector3};
//use cgmath::InnerSpace;
use crate::tower::{WINDOWWIDTH, WINDOWHEIGHT};

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
        let tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        for (sprite, transform, tw_image) in (&sprites, &transforms, &tw_images).join() {
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
                println!("Inside !")
            } else {
                println!("Outsie !")
            }
        }
//            if input.key_is_down(VirtualKeyCode::LAlt) &&
//                input.mouse_button_is_down(MouseButton::Left) && pointer_inside {
//                if tw_input_handler.last_mouse_pos.is_none() {
//                    tw_input_handler.set_last_mouse_pos(input.mouse_position());
//                } else {
//                    let (x, y) = tw_input_handler.last_mouse_pos.unwrap();
//                    let (x2, y2) = input.mouse_position().unwrap();
//                    let dist = ((x2 - x), (y2 - y));
//                    let delta_x = dist.0 - tw_input_handler.last_mouse_dist.0;
//                    let delta_y = dist.1 - tw_input_handler.last_mouse_dist.1;
//                    tw_input_handler.last_mouse_dist = (dist.0, dist.1);
//                    transform.prepend_translation_x(delta_x);
//                    transform.prepend_translation_y(-delta_y);
//                }
//            } else if input.key_is_down(VirtualKeyCode::LAlt) {
//                tw_input_handler.set_last_mouse_pos(None);
//                tw_input_handler.last_mouse_dist = (0.0, 0.0);
//            }
//        }
    }
}

