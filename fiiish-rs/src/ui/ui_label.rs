
use crate::math::Vector2;
use crate::renderer::{
	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerData,
	UiElementFadeState,
	UiEvent,
	UiRenderer,
};

pub struct UiLabel {
	size: Vector2,
	color: Color,
	text: String,
}

impl UiLabel {
	pub fn new( size: &Vector2, text: &str ) -> Self {
		Self {
			size: *size,
			color: Color::from_rgba( 0.8, 0.8, 0.8, 0.8 ),
			text: text.to_owned(),
		}
	}
}

impl UiElement for UiLabel {
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
			//ui_renderer.render_quad( &container.pos, &self.size );
			// :TODO: font
			ui_renderer.print( &container.pos, &self.text );

			ui_renderer.pop_opacity();
			ui_renderer.pop_color();
		}		
	}

}
