use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use amethyst::core::{Stopwatch, math::{dimension::U3, Point3, Point2}};
use amethyst::{
        winit::{VirtualKeyCode, Event, WindowEvent},
};

use uuid::Uuid;
use crate::twimage::TwImage;

pub struct TwInputHandler {
    pub middlepoint: Point2<f32>,
    pub zoomfactor: f32,
    pub stopwatch: Stopwatch,
    pub last_mouse_pos: Option<(f32, f32)>,
    pub last_mouse_dist: (f32, f32),
    pub mouse_world_pos: Point3<f32>,
    pub twimages_under_mouse: Vec<(Uuid, u8)>,
    pub twimage_active: Option<Uuid>,
    pub z: f32,
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