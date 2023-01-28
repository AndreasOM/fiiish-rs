
use oml_game::math::Vector2;
use crate::renderer::{
	Color,
};

use crate::ui::*;

pub struct UiSpacer {
	size: Vector2,
	color: Color,
}

impl UiSpacer {
	pub fn new( size: &Vector2, color: &Color ) -> Self {
		Self {
			size: *size,
			color: *color,
		}
	}
}

static UI_LABEL_VISIBLE: bool = false; 

impl UiElement for UiSpacer {
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
		if UI_LABEL_VISIBLE {
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

}
