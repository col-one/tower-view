/// contains all the function related to the inputs window
use amethyst::ecs::prelude::{Entity};

use amethyst::core::{Stopwatch};
use amethyst::{
        winit::{VirtualKeyCode, Event, WindowEvent, dpi::LogicalPosition, ElementState, MouseButton,
                KeyboardInput, ModifiersState},
};

use uuid::Uuid;



/// if match return an Option of the dropped file path
pub fn get_drop_file(event: &Event) -> Option<String> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::DroppedFile(path) => Some(path.to_str().unwrap().to_owned()),
            _ => None,
        },
        _ => None,
    }
}

/// if match return an option of a mouse screen position
pub fn get_moved_mouse(event: &Event) -> Option<&LogicalPosition> {
    match *event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::CursorMoved { position: logical, .. } => Some(logical),
            _ => None,
        },
        _ => None,
    }
}

/// match if alt key and mousse pressed, return an option of the mouse button pressed.
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

/// match if alt key and mousse released, return an option of the mouse button released.
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

/// match if ctrl key and mousse pressed, return an option of the mouse button pressed.
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

/// match if ctrl key and mousse released, return an option of the mouse button released.
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

/// return option of the current mouse button pressed
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

/// return option of the current mouse button released
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

/// return option of the any key code pressed
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

/// return option of the any key code released
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

/// struct that hold all the inputs values, could be better, some member could be part of TowerData
#[derive(Default)]
pub struct TwInputsHandler {
    pub stopwatch: Stopwatch,
    pub double_click_stopwatch: Stopwatch,
    pub last_dropped_file_path: Vec<String>,
    pub mouse_position: Option<(f32, f32)>,
    pub mouse_world_position: Option<(f32, f32)>,
    pub mouse_world_clicked_position: Option<(f32, f32)>,
    pub mouse_button_pressed: Option<MouseButton>,
    pub mouse_double_clicked: Option<MouseButton>,
    pub mouse_position_history: Vec<(f32, f32)>,
    pub alt_mouse_button_pressed: Option<MouseButton>,
    pub ctrl_mouse_button_pressed: Option<MouseButton>,
    pub keys_pressed: Vec<VirtualKeyCode>,
    pub last_key_released: Option<VirtualKeyCode>,
    pub twimages_under_mouse: Vec<(Uuid, f32)>,
    pub twimage_active: Option<Uuid>,
    pub active_entities: Vec<Entity>,
    pub z_ordered_entities: Vec<Entity>,
    pub window_zoom_factor: f32,
    pub active_busy: bool,
}

impl TwInputsHandler {
    /// get a distance between two mouse frame position
    pub fn get_mouse_delta_distance(&self) -> (f32, f32) {
        let (x, y) = self.mouse_position_history[0];
        let (x2, y2) = self.mouse_position_history[1];
        let dist = ((x2 - x), (y2 - y));
        dist
    }
}
