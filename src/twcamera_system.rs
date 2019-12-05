use amethyst::{
    prelude::*,
    input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::MouseButton,
    renderer::camera::{ActiveCamera, Camera, Projection},
    renderer::rendy::wsi::winit::Window,
};
use std::time::Duration;

use crate::twcamera::TwCamera;
use crate::twinputshandler::TwInputHandler;
use crate::tower::WINDOWHEIGHT;


#[derive(SystemDesc)]
pub struct CameraTranslateNavigationSystem;

impl<'s> System<'s> for CameraTranslateNavigationSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwCamera>,
                       WriteStorage<'s, Transform>,);

    fn run(&mut self, (input, mut world, tw_cameras, mut transforms): Self::SystemData) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        for (_, transform) in (&tw_cameras, &mut transforms).join() {
            if input.key_is_down(VirtualKeyCode::Space) && input.mouse_button_is_down(MouseButton::Left) {
                if tw_input_handler.last_mouse_pos.is_none() {
                    tw_input_handler.set_last_mouse_pos(input.mouse_position());
                } else {
                    let (x, y) = tw_input_handler.last_mouse_pos.unwrap();
                    let (x2, y2) = input.mouse_position().unwrap();
                    let dist = ((x2 - x), (y2 - y));
                    let delta_x = dist.0 - tw_input_handler.last_mouse_dist.0;
                    let delta_y = dist.1 - tw_input_handler.last_mouse_dist.1;
                    tw_input_handler.last_mouse_dist = (dist.0, dist.1);
                    transform.prepend_translation_x(-delta_x);
                    transform.prepend_translation_y(delta_y);
                }
            } else if input.key_is_down(VirtualKeyCode::Space) {
                tw_input_handler.set_last_mouse_pos(None);
                tw_input_handler.last_mouse_dist = (0.0, 0.0);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraKeepRatioSystem;

impl<'s> System<'s> for CameraKeepRatioSystem {
    type SystemData = (
    WriteStorage<'s, Camera>,
    ReadExpect<'s, Window>,
    );

    fn run(&mut self, (mut camera, window): Self::SystemData) {
        let window_size = window.get_inner_size().unwrap();
        let camera = (&mut camera).join().next().unwrap();
        let proj = Projection::perspective(
            (window_size.width / window_size.height) as f32,
            std::f32::consts::FRAC_PI_3,
            0.1,
            2000.0,
        );
        camera.set_projection(proj);
    }
}


#[derive(SystemDesc)]
pub struct CameraZoomNavigationSystem;

impl<'s> System<'s> for CameraZoomNavigationSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwCamera>,
                       WriteStorage<'s, Transform>,);

    fn run(&mut self, (input, mut world, tw_cameras, mut transforms): Self::SystemData) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        for (_, transform) in (&tw_cameras, &mut transforms).join() {
            if input.key_is_down(VirtualKeyCode::LControl) && input.mouse_button_is_down(MouseButton::Left) {
                if tw_input_handler.last_mouse_pos.is_none() {
                    tw_input_handler.set_last_mouse_pos(input.mouse_position());
                } else {
                    let (x, y) = tw_input_handler.last_mouse_pos.unwrap();
                    let (x2, y2) = input.mouse_position().unwrap();
                    let dist = ((x2 - x), (y2 - y));
                    let delta_x = dist.0 - tw_input_handler.last_mouse_dist.0;
                    let delta_y = dist.1 - tw_input_handler.last_mouse_dist.1;
                    tw_input_handler.last_mouse_dist = (dist.0, dist.1);
                    transform.prepend_translation_z(delta_y);
                }
            } else if input.key_is_down(VirtualKeyCode::LControl) {
                tw_input_handler.set_last_mouse_pos(None);
                tw_input_handler.last_mouse_dist = (0.0, 0.0);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraFitNavigationSystem;

impl<'s> System<'s> for CameraFitNavigationSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       ReadStorage<'s, TwCamera>,
                       WriteStorage<'s, Transform>,);

    fn run(&mut self, (input, mut world, tw_cameras, mut transforms): Self::SystemData) {
        let mut tw_input_handler = world.entry::<TwInputHandler>().or_insert_with(|| TwInputHandler::default());
        for (_, transform) in (&tw_cameras, &mut transforms).join() {
            if input.key_is_down(VirtualKeyCode::F) {
               transform.set_translation_z(WINDOWHEIGHT);
            }
        }
    }
}
