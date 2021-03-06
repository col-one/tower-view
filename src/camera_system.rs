/// camera_system.rs is file where live all the camera system,
/// every view system related.
use amethyst::{
    input::{VirtualKeyCode},
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    ecs::prelude::*,
    renderer::rendy::wsi::winit::{Window, dpi::{LogicalPosition, LogicalSize}},
    renderer::camera::{Camera, Projection, Perspective},
};

use std::time::Duration;

use crate::camera::{TwCamera};
use crate::inputshandler::{TwInputsHandler};
use crate::tower::{TowerData};

use std::cmp::max;


#[derive(SystemDesc, Default)]
pub struct CameraTranslateNavigationSystem {
    locked_mouse: (f32, f32)
}
/// Translation camera system with space click and drag
/// a mouse position history is used to lock the move when the mouse doesn't move while space
/// key still pressed.
/// To keep a smooth translating depending on "zoom" the speed is multiply by the cam.z * 0.003
impl<'s> System<'s> for CameraTranslateNavigationSystem {
    type SystemData = (Read<'s, TwInputsHandler>,
                       ReadStorage<'s, TwCamera>,
                       WriteStorage<'s, Transform>,
                       WriteExpect<'s, Window>);
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        mut transforms,
        mut window
    ): Self::SystemData) {
        for (_, transform) in (&tw_cameras, &mut transforms).join() {
            if tw_in.keys_pressed.contains(&VirtualKeyCode::Space) && !tw_in.mouse_button_pressed.is_none() {
                if self.locked_mouse == tw_in.mouse_position_history[1] {
                    break
                }
                let dist = tw_in.get_mouse_delta_distance();
                // TODO: use speed factor as settings
                let delta_dist_x = (dist.0) * ((transform.translation().z * 0.003) * 0.4);
                let delta_dist_y = (dist.1) * ((transform.translation().z * 0.003) * 0.4);
                transform.prepend_translation_x(-delta_dist_x);
                transform.prepend_translation_y(delta_dist_y);
                debug!("Camera moved of {:?}", (delta_dist_x, delta_dist_y));
                self.locked_mouse = tw_in.mouse_position_history[1];
                // if mouse block by screen
                if self.locked_mouse.1 <= 4.0 {
                    debug!("Cursor lock top screen");
                    window.set_cursor_position(LogicalPosition::new(self.locked_mouse.0 as f64,  window.get_inner_size().unwrap().height - 5.0));
                }
                if self.locked_mouse.1 >=  window.get_inner_size().unwrap().height as f32 - 4.0 {
                    debug!("Cursor lock down screen, loop the mouse");
                    window.set_cursor_position(LogicalPosition::new(self.locked_mouse.0 as f64, 5.0));
                }
                if self.locked_mouse.0 <= 4.0 {
                    debug!("Cursor lock left screen");
                    window.set_cursor_position(LogicalPosition::new(window.get_inner_size().unwrap().width - 5.0, self.locked_mouse.1 as f64));
                }
                if self.locked_mouse.0 >=  window.get_inner_size().unwrap().width as f32 - 4.0 {
                    debug!("Cursor lock right screen, loop the mouse");
                    window.set_cursor_position(LogicalPosition::new( 5.0, self.locked_mouse.1 as f64));f
                }
                self.locked_mouse = tw_in.mouse_position_history[1];
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraKeepRatioSystem {
    pub previous_size: LogicalSize
}
/// Keep tracking when window size changed to create a new camera projection with the new
/// window width / height ratio and avoid image deformation.
impl<'s> System<'s> for CameraKeepRatioSystem {
    type SystemData = (WriteStorage<'s, Camera>,
                       ReadExpect<'s, Window>,
                       Read<'s, TwInputsHandler>);

    fn run(&mut self, (
        mut camera,
        window,
        _tw_in
    ): Self::SystemData) {
        if self.previous_size != window.get_inner_size().unwrap() {
            let window_size = window.get_inner_size().unwrap();
            let camera = (&mut camera).join().next().unwrap();
            // TODO: use near and far clipping as settings
            let persp = Perspective::new(
                (window_size.width / window_size.height) as f32,
                std::f32::consts::FRAC_PI_3, 1.0, 100000.0);
            let proj = Projection::Perspective(persp);
            camera.set_projection(proj);
            self.previous_size = window_size;
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct CameraZoomNavigationSystem {
    locked_mouse: (f32, f32),
}
/// It's call Zoom for handy reasons, but yeah it's not real zoom, it's translation along Z (dolly)
/// Zoom in/out by set camera Z axis. With Ctrl + click and drag. As camera translation system.
/// It use the mouse position history to lock the zoom if mouse doesn't move while Ctrl
/// key still pressed.
/// To keep a smooth zooming the speed is multiply by the cam.z * 0.003
impl<'s> System<'s> for CameraZoomNavigationSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, Camera>,
                       WriteStorage<'s, Transform>,
                       WriteExpect<'s, Window>
                       );
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        mut transforms,
        mut window
    ): Self::SystemData) {
        for (_cam, transform) in (&tw_cameras, &mut transforms).join() {
            if let Some(_button) = tw_in.ctrl_mouse_button_pressed {
                if self.locked_mouse == tw_in.mouse_position_history[1] {
                    return
                }
                let dist = tw_in.get_mouse_delta_distance();
                if transform.translation().z <= 1.01 {
                    return
                }
                // TODO: use speed factor as settings
                transform.prepend_translation_z(dist.1 * ((transform.translation().z * 0.003) * 1.2));
                self.locked_mouse = tw_in.mouse_position_history[1];
                // if mouse block by screen
                if self.locked_mouse.1 <= 4.0 {
                    debug!("Cursor lock top screen");
                    window.set_cursor_position(LogicalPosition::new(self.locked_mouse.0 as f64,  window.get_inner_size().unwrap().height - 5.0));
                }
                if self.locked_mouse.1 >=  window.get_inner_size().unwrap().height as f32 - 4.0 {
                    debug!("Cursor lock down screen, loop the mouse");
                    window.set_cursor_position(LogicalPosition::new(self.locked_mouse.0 as f64, 5.0));
                }
                self.locked_mouse = tw_in.mouse_position_history[1];
            }
        }
    }
}


#[derive(SystemDesc, Default)]
pub struct CameraCenterSystem {
    released: bool
}
/// if double clicked camera and cursor moved to the center.
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
                        debug!("Camera moved to {:?} and cursor moved to window center.", (mouse_world_click.0, mouse_world_click.1));
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
/// System that move camera on Z to fit the image size.
/// F key fit the active image
/// Shift + F fit the whole images scene, a bounding box is calculated with all image sizes and position
/// to get the center and maxi size of height or width.
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
                debug!("Camera is moved with {:?} to fit the active image", transform);
            }
        }
        if tw_in.keys_pressed.contains(&VirtualKeyCode::F) && tw_in.keys_pressed.contains(&VirtualKeyCode::LShift) && tw_in.keys_pressed.len() == 2 {
            if Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                let (_, transform) = (&tw_cameras, &mut transforms).join().next().unwrap();
                transform.set_translation_x(tw_data.scene_middle_point.x);
                transform.set_translation_y(tw_data.scene_middle_point.y);
                transform.set_translation_z(max(tw_data.scene_rect.height() as i32, tw_data.scene_rect.width() as i32) as f32);
                debug!("Camera is moved with {:?} to fit the whole images scene", transform);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct CameraOriginalScaleSystem;
/// set the camera z to get the real size of image, from TowerData.real_size_z
/// With the shortcut s
impl<'s> System<'s> for CameraOriginalScaleSystem {
    type SystemData = (Write<'s, TwInputsHandler>,
                       ReadStorage<'s, TwCamera>,
                       ReadStorage<'s, Camera>,
                       WriteStorage<'s, Transform>,
                       Read<'s, TowerData>);
    fn run(&mut self, (
        tw_in,
        tw_cameras,
        cameras,
        mut transforms,
        tw_data
    ): Self::SystemData) {
        if tw_in.keys_pressed.contains(&VirtualKeyCode::S) && tw_in.keys_pressed.len() == 1 {
            if Duration::from_millis(500) <= tw_in.stopwatch.elapsed() {
                let (_, _, transform) = (&cameras, &tw_cameras, &mut transforms).join().next().unwrap();
                transform.set_translation_z(tw_data.real_size_z);
                debug!("Camera z transform is set to {:?} to fit the real image size", tw_data.real_size_z);
            }
        }
    }
}
