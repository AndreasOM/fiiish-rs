
use crate::math::Vector2;
use crate::renderer::{
//	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiElementBase,
	UiElementFadeState,
	UiRenderer,
};

pub struct UiImage {
	base: UiElementBase,
	name: String,
	fade_state: UiElementFadeState,
}

impl UiImage {
	pub fn new( name: &str, size: &Vector2 ) -> Self {
		let mut base = UiElementBase::new();
		base.size = *size;
		Self {
			base,
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
		ui_renderer.render_textured_quad( &self.borrow_base().pos, &self.borrow_base().size );
	}
	fn layout( &mut self, pos: &Vector2 ) {
		self.borrow_base_mut().pos = *pos;
	}

	fn borrow_base( &self ) -> &UiElementBase {
		&self.base
	}

	fn borrow_base_mut( &mut self ) -> &mut UiElementBase {
		&mut self.base
	}

	fn fade_state( &self ) -> &UiElementFadeState {
		&self.fade_state
	}
	fn set_fade_state( &mut self, fade_state: &UiElementFadeState ) {
		self.fade_state = *fade_state;
	}

}
