
use crate::math::Vector2;
use crate::renderer::{
//	Color,
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

pub struct UiImage {
	imagename: String,
	imagesize: Vector2,
}

impl UiImage {
	pub fn new( imagename: &str, size: &Vector2 ) -> Self {
		Self {
			imagename: imagename.to_owned(),
			imagesize: *size,
		}
	}
}

impl UiElement for UiImage {
	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.imagesize )
	}
	fn handle_ui_event( &mut self, container: &mut UiElementContainerData, event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
		println!("UiImage got event -> {} -> {}", &container.name, &self.imagename );
		false
	}
	/*
	fn update( &mut self, _time_step: f64 ) {
	}
	*/
	fn render( &self, container: &UiElementContainerData, ui_renderer: &mut UiRenderer) {
		if *container.fade_state() != UiElementFadeState::FadedOut {
			let l = container.get_fade_level();
			ui_renderer.push_opacity( l );
			ui_renderer.use_texture( &self.imagename );
			ui_renderer.render_textured_quad( &container.pos, &self.imagesize );
			ui_renderer.pop_opacity();
		}		
	}

}
