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

pub fn get_delta_position(position: (f32, f32), position2: (f32, f32)) -> (f32, f32) {
    let dist = ((position2.0 - position.0), (position2.1 - position.1));
    (dist.0, dist.1)
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

pub fn mouse_pressed(event: &Event) -> Option<MouseButton> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::MouseInput { state: ElementState::Pressed, button, modifiers: ModifiersState {
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
    pub mouse_world_clicked_position: Option<(f32, f32)>,
    pub mouse_button_pressed: Option<MouseButton>,
    pub mouse_position_history: Vec<(f32, f32)>,
    pub alt_mouse_button_pressed: Option<MouseButton>,
    pub ctrl_mouse_button_pressed: Option<MouseButton>,
    pub keys_pressed: Vec<VirtualKeyCode>,
    pub last_key_released: Option<VirtualKeyCode>,
    pub twimages_under_mouse: Vec<(Uuid, f32)>,
    pub twimage_active: Option<Uuid>,
    pub window_zoom_factor: f32,
}

impl TwInputsHandler {
    pub fn get_mouse_delta_distance(&self) -> (f32, f32) {
        let (x, y) = self.mouse_position_history[0];
        let (x2, y2) = self.mouse_position_history[1];
        let dist = ((x2 - x), (y2 - y));
        dist
    }
}
