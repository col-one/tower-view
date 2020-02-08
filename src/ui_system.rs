use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst_imgui::{
	imgui,
	imgui::{im_str, ImString, Condition},
	RenderImgui,
};
use amethyst::renderer::rendy::wsi::winit::{Window};
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use crate::image::{TwActiveUiComponent, TwImage};
use crate::inputshandler::TwInputsHandler;


pub const UI_WIDTH: f32 = 300.0;


#[derive(Default, Clone, Copy)]
pub struct SliderChannelsSystem {
	pub open: bool,
}

impl<'s> amethyst::ecs::System<'s> for SliderChannelsSystem {
	type SystemData = (ReadExpect<'s, TwInputsHandler>,
	                   ReadStorage<'s, TwActiveUiComponent>,
					   WriteStorage<'s, TwImage>);
	fn run(&mut self, (
			tw_in,
		   	twactives,
			mut twimages
	) : Self::SystemData) {
		for (twactive, twimage) in (&twactives, &mut twimages).join() {
			if tw_in.keys_pressed.contains(&VirtualKeyCode::C) && tw_in.keys_pressed.contains(&VirtualKeyCode::LShift) && tw_in.keys_pressed.len() == 2 {
				self.open = true;
			}
			if tw_in.keys_pressed.contains(&VirtualKeyCode::Escape) { self.open = false }
			if self.open {
				amethyst_imgui::with(|ui| {
					let window = imgui::Window::new(im_str!("Channels"))
//						.always_auto_resize(true)
						.size([UI_WIDTH, 0.0], Condition::Always)
						.position([ui.io().mouse_pos[0] - UI_WIDTH * 0.5, ui.io().mouse_pos[1]], Condition::Appearing)
						.build(ui, || {
							let mut slider = imgui::Slider::new(im_str!("Alpha Amount"), 0.0..=1.0)
								.build(ui, &mut twimage.alpha);
							let mut slider = imgui::Slider::new(im_str!("Red Amount"), 0.0..=1.0)
								.build(ui, &mut twimage.red);
							let mut slider = imgui::Slider::new(im_str!("Green Amount"), 0.0..=1.0)
								.build(ui, &mut twimage.green);
							let mut slider = imgui::Slider::new(im_str!("Blue Amount"), 0.0..=1.0)
								.build(ui, &mut twimage.blue);
						}
					);
				});
			}
		}
		if twactives.is_empty() { self.open = false }
    }
}
