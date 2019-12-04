use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;


pub struct TwInputHandler {
    pub last_mouse_pos: Option<(f32, f32)>,
    pub last_mouse_dist: (f32, f32),
}

impl Default for TwInputHandler {
    fn default() -> Self {
        TwInputHandler {
            last_mouse_pos: None,
            last_mouse_dist: (0.0, 0.0),
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
}
