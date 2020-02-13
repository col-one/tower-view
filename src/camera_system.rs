use amethyst::{
    input::{VirtualKeyCode},
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::{Window, dpi::LogicalPosition},
    renderer::camera::{Camera, Projection, Perspective},
};


use std::time::Duration;

use crate::camera::{TwCamera};
use crate::inputshandler::{TwInputsHandler};
use crate::tower::{TowerData, WINDOWWIDTH};

use std::cmp::max;


#[derive(SystemDesc, Default)]
pub struct CameraTranslateNavigationSystem {
    locked_mouse: (f32, f32)
}

impl<'s> System<'s> for CameraTranslateNavigationSystem {
    type SystemData = (Read<'s, TwInputsHandler>,
                       ReadStorage<'s, TwCamera>,
                       WriteStorage<'s, Transform>,);
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        mut transforms
    ): Self::SystemData) {
        for (_, transform) in (&tw_cameras, &mut transforms).join() {
            if tw_in.keys_pressed.contains(&VirtualKeyCode::Space) && !tw_in.mouse_button_pressed.is_none() {
                if self.locked_mouse == tw_in.mouse_position_history[1] {
                    break
                }
                let dist = tw_in.get_mouse_delta_distance();
                transform.prepend_translation_x(-(dist.0));
                transform.prepend_translation_y(dist.1);
                self.locked_mouse = tw_in.mouse_position_history[1];
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraKeepRatioSystem;

impl<'s> System<'s> for CameraKeepRatioSystem {
    type SystemData = (WriteStorage<'s, Camera>,
                       ReadExpect<'s, Window>,
                       Read<'s, TwInputsHandler>);

    fn run(&mut self, (
        mut camera,
        window,
        _tw_in
    ): Self::SystemData) {
        let window_size = window.get_inner_size().unwrap();
        let camera = (&mut camera).join().next().unwrap();
        let persp = Perspective::new((window_size.width / window_size.height) as f32, std::f32::consts::FRAC_PI_3, 0.01, 6000.0);
        let proj = Projection::Perspective(persp);
        camera.set_projection(proj);
    }
}


#[derive(SystemDesc, Default)]
pub struct CameraZoomNavigationSystem {
    locked_mouse: (f32, f32),
}

impl<'s> System<'s> for CameraZoomNavigationSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, Camera>,
                       WriteStorage<'s, Transform>,
                       );
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        mut transforms,
    ): Self::SystemData) {
        for (_cam, transform) in (&tw_cameras, &mut transforms).join() {
            if let Some(_button) = tw_in.ctrl_mouse_button_pressed {
                if self.locked_mouse == tw_in.mouse_position_history[1] {
                    return
                }
                let dist = tw_in.get_mouse_delta_distance();
                transform.prepend_translation_z(dist.1 * 1.2);
                self.locked_mouse = tw_in.mouse_position_history[1];
            }
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct CameraCenterSystem {
    released: bool
}

impl<'s> System<'s> for CameraCenterSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, Camera>,
                       WriteStorage<'s, Transform>,
                       WriteExpect<'s, Window>,
                       );
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        mut transforms,
        window,
    ): Self::SystemData) {
        for (_, transform) in (&tw_cameras, &mut transforms).join() {
            if let Some(_button) = tw_in.mouse_double_clicked {
                if !self.released && tw_in.keys_pressed.len() == 0 {
                    window.set_cursor_position(LogicalPosition::new((window.get_inner_size().unwrap().width * 0.5) as f64,
                                                                    (window.get_inner_size().unwrap().height * 0.5) as f64)).expect("can't set mouse cursor.");
                    if let Some(mouse_world_click) = tw_in.mouse_world_clicked_position {
                        transform.set_translation_x(mouse_world_click.0);
                        transform.set_translation_y(mouse_world_click.1);
                    }
                }
                self.released = true;
            } else {
                self.released = false;
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraFitNavigationSystem;

impl<'s> System<'s> for CameraFitNavigationSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, TwCamera>,
                       WriteStorage<'s, Transform>,
                       Read<'s, TowerData>);

    fn run(&mut self, (
        tw_in,
        tw_cameras,
        mut transforms,
        tw_data,
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::F) && tw_in.keys_pressed.len() == 1 {
            if Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                let (_, transform) = (&tw_cameras, &mut transforms).join().next().unwrap();
                transform.set_translation_x((tw_data.active_rect.min.x + tw_data.active_rect.max.x) / 2.0);
                transform.set_translation_y((tw_data.active_rect.min.y + tw_data.active_rect.max.y) / 2.0);
                transform.set_translation_z(max(tw_data.active_rect.height() as i32, tw_data.active_rect.width() as i32) as f32);
            }
        }
        if tw_in.keys_pressed.contains(&VirtualKeyCode::F) && tw_in.keys_pressed.contains(&VirtualKeyCode::LShift) && tw_in.keys_pressed.len() == 2 {
            if Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                let (_, transform) = (&tw_cameras, &mut transforms).join().next().unwrap();
                transform.set_translation_x(tw_data.scene_middle_point.x);
                transform.set_translation_y(tw_data.scene_middle_point.y);
                transform.set_translation_z(max(tw_data.scene_rect.height() as i32, tw_data.scene_rect.width() as i32) as f32);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraOrignalScaleSystem;

impl<'s> System<'s> for CameraOrignalScaleSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, TwCamera>,
                       ReadStorage<'s, Camera>,
                       WriteStorage<'s, Transform>);
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        cameras,
        mut transforms,
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::S) && tw_in.keys_pressed.len() == 1 {
            if Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                let (_, _, transform) = (&cameras, &tw_cameras, &mut transforms).join().next().unwrap();
                let real_size_dist = WINDOWWIDTH / (2.0 * (std::f32::consts::FRAC_PI_3 / 2.0).tan());
                transform.set_translation_z(real_size_dist);
            }
        }
    }
}
