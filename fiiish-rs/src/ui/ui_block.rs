
use oml_game::math::Vector2;
use oml_game::renderer::{
	Color,
};

use crate::ui::*;

pub struct UiBlock {
	size: Vector2,
	color: Color,
}

impl UiBlock {
	pub fn new( size: &Vector2, color: &Color ) -> Self {
		Self {
			size: *size,
			color: *color,
		}
	}
}

impl UiElement for UiBlock {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.size )
	}
	fn render( &self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_color( &self.color );
			ui_renderer.push_opacity( l );
			ui_renderer.render_quad( &container.pos, &self.size );
			ui_renderer.pop_opacity();
			ui_renderer.pop_color();
		}		
	}

}
