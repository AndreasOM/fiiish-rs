
use crate::DebugRenderer;
use crate::math::Vector2;
use crate::renderer::Color;
use crate::ui::{
	UiElementBase,
	UiElementContainer,
	UiElementContainerData,
	UiEvent,
	UiRenderer,
};

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct UiElementFadeData {
	pub level: f32,
	pub speed: f32,
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum UiElementFadeState {
	FadedOut,
	FadingIn( UiElementFadeData ),
	FadedIn,
	FadingOut( UiElementFadeData ),
}

pub trait UiElement {
	fn recalculate_size( &mut self, _container: &mut UiElementContainerData ) {}
	fn add_child( &mut self, _child: &mut UiElementContainerData ) {}
	fn update( &mut self, _time_step: f64 ) {}
	fn render( &self, _container: &UiElementContainerData, _ui_renderer: &mut UiRenderer) {}
	fn layout( &mut self, _container: &mut UiElementContainerData, _pos: &Vector2 ){}
	fn render_debug( &self, _container: &UiElementContainerData, _debug_renderer: &mut DebugRenderer, _offset: &Vector2 ) {}
	fn handle_ui_event( &mut self, _container: &mut UiElementContainerData, _event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
		false
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
		None
	}
//	fn set_size( &mut self, size: &Vector2 ) {}
}

impl std::fmt::Debug for dyn UiElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//		writeln!( f,"[Trait] UiElement: {}x{} @ {}, {}", self.size().x, self.size().y, self.pos().x, self.pos().y )
		writeln!( f, "[Trait] UiElement" )
	}
}
