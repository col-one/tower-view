use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use amethyst::core::{Stopwatch, math::{dimension::U3, Point3, Point2}};
use amethyst::{
        winit::{VirtualKeyCode, Event, WindowEvent, dpi::LogicalPosition, ElementState, MouseButton,
                KeyboardInput, WindowId, DeviceId, ModifiersState},
};

use uuid::Uuid;
use crate::image::TwImage;
use std::time::Duration;


#[derive(PartialEq)]
pub enum MouseState {
    Free,
    Down,
    Up
}


pub struct TwInputHandler {
    pub middlepoint: Point2<f32>,
    pub zoomfactor: f32,
    pub stopwatch: Stopwatch,
    pub last_mouse_pos: Option<(f32, f32)>,
    pub last_mouse_dist: (f32, f32),
    pub mouse_world_pos: Point3<f32>,
    pub twimages_under_mouse: Vec<(Uuid, f32)>,
    pub twimage_active: Option<Uuid>,
    pub z: f32,
    pub mouse_state: MouseState,
}

impl Default for TwInputHandler {
    fn default() -> Self {
        let mut timer = Stopwatch::new();
        timer.start();
        TwInputHandler {
            middlepoint: Point2::new(0.0, 0.0),
            zoomfactor: 1.0,
            stopwatch: timer,
            last_mouse_pos: None,
            last_mouse_dist: (0.0, 0.0),
            mouse_world_pos: Point3::new(0.0, 0.0, 0.0),
            twimages_under_mouse: Vec::new(),
            twimage_active: None,
            z: 0.0,
            mouse_state: MouseState::Free,
        }
    }
}

impl Component for TwInputHandler {
    type Storage = DenseVecStorage<Self>;
}

impl TwInputHandler {
    pub fn set_last_mouse_pos(&mut self, pos: Option<(f32, f32)>) {
        self.last_mouse_pos = pos;
    }
    pub fn set_twimage_active(&mut self, uuid: Option<Uuid>) {
        self.twimage_active = uuid;
    }
    pub fn set_default(&mut self) {
        self.last_mouse_pos = None;
        self.last_mouse_dist = (0.0,0.0);
        self.mouse_world_pos = Point3::new(0.0, 0.0, 0.0);
        self.twimage_active = None;
    }
}

pub fn get_drop_file(event: &Event) -> Option<String> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::DroppedFile(path) => Some(path.to_str().unwrap().to_owned()),
            _ => None,
        },
        _ => None,
    }
}

pub fn get_moved_mouse(event: &Event) -> Option<&LogicalPosition> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::CursorMoved { position: logical, .. } => Some(logical),
            _ => None,
        },
        _ => None,
    }
}

pub fn alt_mouse_pressed(event: &Event) -> Option<MouseButton> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::MouseInput { state: ElementState::Pressed, button, modifiers: ModifiersState {
                shift: false,
                ctrl: false,
                alt: true,
                logo: false}, ..
            } => Some(button.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn alt_mouse_released(event: &Event) -> Option<MouseButton> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::MouseInput { state: ElementState::Released, button, modifiers: ModifiersState {
                shift: false,
                ctrl: false,
                alt: true,
                logo: false}, ..
            } => Some(button.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn ctrl_mouse_pressed(event: &Event) -> Option<MouseButton> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::MouseInput { state: ElementState::Pressed, button, modifiers: ModifiersState {
                shift: false,
                ctrl: true,
                alt: false,
                logo: false}, ..
            } => Some(button.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn ctrl_mouse_released(event: &Event) -> Option<MouseButton> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::MouseInput { state: ElementState::Released, button, modifiers: ModifiersState {
                shift: false,
                ctrl: true,
                alt: false,
                logo: false}, ..
            } => Some(button.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn mouse_released(event: &Event) -> Option<MouseButton> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::MouseInput { state: ElementState::Released, button, modifiers: ModifiersState {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false}, ..
            } => Some(button.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn key_pressed(event: &Event) -> Option<VirtualKeyCode> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::KeyboardInput { input: KeyboardInput {
                        virtual_keycode,
                        state: ElementState::Pressed,
                        .. }, .. } => *virtual_keycode,
            _ => None,
        },
        _ => None,
    }
}

pub fn key_released(event: &Event) -> Option<VirtualKeyCode> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::KeyboardInput { input: KeyboardInput {
                        virtual_keycode,
                        state: ElementState::Released,
                        .. }, .. } => *virtual_keycode,
            _ => None,
        },
        _ => None,
    }
}

#[derive(Default)]
pub struct TwInputsHandler {
    pub stopwatch: Stopwatch,
    pub last_dropped_file_path: Option<String>,
    pub mouse_position: Option<(f32, f32)>,
    pub mouse_world_position: Option<(f32, f32)>,
    pub mouse_button_pressed: Option<MouseButton>,
    pub alt_mouse_button_pressed: Option<MouseButton>,
    pub ctrl_mouse_button_pressed: Option<MouseButton>,
    pub keys_pressed: Vec<VirtualKeyCode>,
    pub last_key_released: Option<VirtualKeyCode>,
    pub twimages_under_mouse: Vec<(Uuid, f32)>,
    pub twimage_active: Option<Uuid>,
}
//impl TwInputsHandler {
//    pub fn get_elapsed_time(&self) -> Duration {
//        if let Some(timer) = &self.stopwatch {
//            return timer.elapsed()
//        }
//        Duration::from_secs(0)
//    }
//    pub fn restart_time(&mut self) {
//        if let Some(mut timer) = &self.stopwatch {
//            timer.restart();
//        }
//    }
//}
