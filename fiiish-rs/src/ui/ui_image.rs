
use crate::math::Vector2;
use crate::renderer::{
//	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiElementFadeState,
	UiRenderer,
};

pub struct UiImage {
	pos: Vector2,
	size: Vector2,
	name: String,
	fade_state: UiElementFadeState,
}

impl UiImage {
	pub fn new( name: &str, size: &Vector2 ) -> Self {
		Self {
			pos: Vector2::zero(),
			size: *size,
			name: name.to_owned(),
			fade_state: UiElementFadeState::FadedIn,
		}
	}
}

impl UiElement for UiImage {
	fn update( &mut self, _time_step: f64 ) {
	}
	fn render( &self, ui_renderer: &mut UiRenderer) {
		ui_renderer.use_texture( &self.name );
		ui_renderer.render_textured_quad( &self.pos, &self.size );
	}
	fn layout( &mut self, pos: &Vector2 ) {
		self.pos = *pos;
	}
	fn size( &self ) -> &Vector2 {
		&self.size
	}
	fn pos( &self ) -> &Vector2 {
		&self.pos
	}
	fn fade_state( &self ) -> &UiElementFadeState {
		&self.fade_state
	}
	fn set_fade_state( &mut self, fade_state: &UiElementFadeState ) {
		self.fade_state = *fade_state;
	}

}
