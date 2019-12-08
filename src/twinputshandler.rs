use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use amethyst::core::math::{dimension::U3, Point3};

use uuid::Uuid;
use crate::twimage::TwImage;

pub struct TwInputHandler {
    pub last_mouse_pos: Option<(f32, f32)>,
    pub last_mouse_dist: (f32, f32),
    pub mouse_world_pos: Point3<f32>,
    pub twimages_under_mouse: Vec<(Uuid, u8)>,
    pub twimage_active: Option<Uuid>,
}

impl Default for TwInputHandler {
    fn default() -> Self {
        TwInputHandler {
            last_mouse_pos: None,
            last_mouse_dist: (0.0, 0.0),
            mouse_world_pos: Point3::new(0.0, 0.0, 0.0),
            twimages_under_mouse: Vec::new(),
            twimage_active: None,
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
}
