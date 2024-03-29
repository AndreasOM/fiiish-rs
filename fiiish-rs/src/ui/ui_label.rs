use oml_game::math::Vector2;
use oml_game::renderer::Color;

use crate::ui::*;

pub struct UiLabel {
	size:      Vector2,
	color:     Color,
	text:      String,
	alignment: Vector2,
}

impl UiLabel {
	pub fn new(size: &Vector2, text: &str) -> Self {
		Self {
			size:      *size,
			color:     Color::from_rgba(0.8, 0.8, 0.8, 0.8),
			text:      text.to_owned(),
			alignment: Vector2::new(-1.0, 0.0),
		}
	}

	pub fn set_alignment(&mut self, alignment: &Vector2) {
		self.alignment = *alignment;
	}

	pub fn set_text(&mut self, text: &str) {
		self.text = text.to_owned();
	}

	pub fn set_color(&mut self, color: &Color) {
		self.color = *color;
	}
}

impl UiElement for UiLabel {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn preferred_size(&self) -> Option<&Vector2> {
		Some(&self.size)
	}
	fn render(&self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_color(&self.color);
			ui_renderer.push_opacity(l);
			ui_renderer.print(&container.pos, &container.size, &self.alignment, &self.text);

			ui_renderer.pop_opacity();
			ui_renderer.pop_color();
		}
	}
}
