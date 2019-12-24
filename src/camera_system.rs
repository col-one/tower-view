use amethyst::{
    prelude::*,
    input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
    core::{SystemDesc, Transform, math::{Vector3, Vector2}},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::MouseButton,
    renderer::camera::{ActiveCamera, Camera, Projection},
    renderer::rendy::wsi::winit::Window,
    renderer::rendy::hal::pso::Rect,
};
use std::time::Duration;

use crate::camera::TwCamera;
use crate::inputshandler::TwInputHandler;
use crate::tower::{WINDOWHEIGHT, WINDOWWIDTH, TowerData};


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
    Read<'s, World>,
    );

    fn run(&mut self, (mut camera, window, world): Self::SystemData) {
        let tw_input_handler = world.fetch::<TwInputHandler>();
        let window_size = window.get_inner_size().unwrap();
        let camera = (&mut camera).join().next().unwrap();
        let proj = Projection::orthographic(
            -window_size.width as f32 / (2.0 * tw_input_handler.zoomfactor),
            window_size.width as f32 / (2.0 * tw_input_handler.zoomfactor),
            -window_size.height as f32 / (2.0 * tw_input_handler.zoomfactor),
            window_size.height as f32 / (2.0 * tw_input_handler.zoomfactor),
            0.1,
            6000.0,
        );
        camera.set_projection(proj);
    }
}


#[derive(SystemDesc)]
pub struct CameraZoomNavigationSystem;

impl<'s> System<'s> for CameraZoomNavigationSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, World>,
                       );
    fn run(&mut self, (input, mut world): Self::SystemData) {
        let mut tw_input_handler = world.fetch_mut::<TwInputHandler>();
        if input.key_is_down(VirtualKeyCode::LControl) && input.mouse_button_is_down(MouseButton::Left) {
            if tw_input_handler.last_mouse_pos.is_none() {
                tw_input_handler.set_last_mouse_pos(input.mouse_position());
            } else {
                let (x, y) = tw_input_handler.last_mouse_pos.unwrap();
                let (x2, y2) = input.mouse_position().unwrap();
                let dist = Vector2::new((x2 - x), (y2 - y));
                let delta = Vector2::new(dist.x - tw_input_handler.last_mouse_dist.0, dist.y - tw_input_handler.last_mouse_dist.1);
                tw_input_handler.last_mouse_dist = (dist.x, dist.y);
                tw_input_handler.zoomfactor = (tw_input_handler.zoomfactor - (delta.y * 0.01)).max(0.01);
            }
        } else if input.key_is_down(VirtualKeyCode::LControl) {
            tw_input_handler.set_last_mouse_pos(None);
            tw_input_handler.last_mouse_dist = (0.0, 0.0);
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
        let mut tw_input_handler = world.fetch_mut::<TwInputHandler>();
        if input.key_is_down(VirtualKeyCode::F) && !input.key_is_down(VirtualKeyCode::LShift){
            tw_input_handler.zoomfactor = 1.0;
            let (_, transform) = (&tw_cameras, &mut transforms).join().next().unwrap();
            transform.set_translation_x(tw_input_handler.middlepoint.x);
            transform.set_translation_y(tw_input_handler.middlepoint.y);
            info!("{:?}", tw_input_handler.middlepoint);
        }
    }
}

