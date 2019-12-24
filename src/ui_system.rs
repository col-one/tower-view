use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use amethyst::ecs::prelude::*;
use amethyst_imgui::{
	imgui,
	imgui::{im_str, ImString, Condition},
	RenderImgui,
};
use amethyst::input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings};
use crate::image::{TwActiveComponent, TwImage};


pub const UI_WIDTH: f32 = 300.0;

#[derive(Default, Clone, Copy)]
pub struct SliderAlphaSystem {
	pub open: bool,
}

impl<'s> amethyst::ecs::System<'s> for SliderAlphaSystem {
	type SystemData = (Read<'s, InputHandler<StringBindings>>,
	                   ReadStorage<'s, TwActiveComponent>,
					   WriteStorage<'s, TwImage>);
	fn run(&mut self, (
			input,
		   	twactives,
			mut twimages
	) : Self::SystemData) {
		for (twactive, twimage) in (&twactives, &mut twimages).join() {
			if input.key_is_down(VirtualKeyCode::A) && input.key_is_down(VirtualKeyCode::LShift) {
				self.open = true;
			}
			if input.key_is_down(VirtualKeyCode::Escape) { self.open = false }
			if self.open {
				amethyst_imgui::with(|ui| {
					let window = imgui::Window::new(im_str!("Alpha Blending"))
						.always_auto_resize(true)
						.size([UI_WIDTH, 0.0], Condition::Always)
						.position([ui.io().mouse_pos[0] - UI_WIDTH * 0.5, ui.io().mouse_pos[1]], Condition::Appearing)
						.build(ui, || {
							let mut slider = imgui::Slider::new(im_str!("Amount"), 0.0..=1.0)
								.build(ui, &mut twimage.alpha);
						}
					);
				});
			}
		}
		if twactives.is_empty() { self.open = false }
    }
}


#[derive(Default, Clone, Copy)]
pub struct SliderRedSystem {
	pub open: bool,
}

impl<'s> amethyst::ecs::System<'s> for SliderRedSystem {
	type SystemData = (Read<'s, InputHandler<StringBindings>>,
	                   ReadStorage<'s, TwActiveComponent>,
					   WriteStorage<'s, TwImage>);
	fn run(&mut self, (
			input,
		   	twactives,
			mut twimages
	) : Self::SystemData) {
		for (twactive, twimage) in (&twactives, &mut twimages).join() {
			if input.key_is_down(VirtualKeyCode::C) && input.key_is_down(VirtualKeyCode::LShift) {
				self.open = true;
			}
			if input.key_is_down(VirtualKeyCode::Escape) { self.open = false }
			if self.open {
				amethyst_imgui::with(|ui| {
					let window = imgui::Window::new(im_str!("Channels"))
						.always_auto_resize(true)
						.size([UI_WIDTH, 0.0], Condition::Always)
						.position([ui.io().mouse_pos[0] - UI_WIDTH * 0.5, ui.io().mouse_pos[1]], Condition::Appearing)
						.build(ui, || {
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