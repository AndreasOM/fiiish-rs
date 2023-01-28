
use std::sync::mpsc::Sender;

use oml_game::renderer::debug_renderer::DebugRenderer;
use oml_game::math::Vector2;

use crate::ui::*;

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
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn setup_within_container( &mut self, _container: &mut UiElementContainerData ) {}

	fn recalculate_size( &mut self, _container: &mut UiElementContainerData ) {}
	fn add_child( &mut self, _child: &mut UiElementContainerData ) {}
	fn update( &mut self, _container: &UiElementContainerData, _time_step: f64 ) {}
	fn render( &self, _container: &UiElementContainerData, _ui_renderer: &mut UiRenderer) {}
	fn layout( &mut self, container: &mut UiElementContainerData, _pos: &Vector2 ){
		for c in container.borrow_children_mut().iter_mut() {
			c.borrow_mut().layout( &Vector2::zero() );
		}
//		container.set_pos( pos );	// no! This is the default anyway
	}
	fn render_debug( &self, _container: &UiElementContainerData, _debug_renderer: &mut DebugRenderer, _offset: &Vector2 ) {}
	fn handle_ui_event( &mut self, _container: &mut UiElementContainerData, _event: &UiEvent, _event_sender: &Sender< Box< dyn UiEventResponse > > ) -> Option< Box < dyn UiEventResponse > > {
//		Vec::new()
		None
	}
	fn handle_ui_event_response( &mut self, response: Box< dyn UiEventResponse > ) -> Option< Box< dyn UiEventResponse > > {
		Some( response )
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
