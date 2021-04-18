
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
	fn render_debug( &self, _container: &UiElementContainerData,debug_renderer: &mut DebugRenderer, offset: &Vector2 ) {}
	fn handle_ui_event( &mut self, _container: &mut UiElementContainerData, event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
		false
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
		None
	}
//	fn set_size( &mut self, size: &Vector2 ) {}
	
	// old below

	fn find_child_mut( &mut self, path: &[ &str ] ) -> Option< &mut dyn UiElement > where Self: Sized {
		/*
		if path.len() == 0 { // nothing left to check
			return None;
		}
		let (head, tail ) = path.split_at(1);
		let head = head[ 0 ];

		println!("Checking {} for {}, {:?}", self.name(), head, tail );

		if head == self.name() {
			if tail.len() == 0 {
				return Some( self );
			} else {
				println!("Found {} ... {:?}", &head, &tail );
				return self.find_child_mut( tail );
			}
		}

		println!("Checking {} children for {}, {:?}", self.borrow_base().children.len(), head, tail );

		for c in self.borrow_base_mut().children.iter_mut() {
			if let Some( r ) = c.find_child_mut( path ) {
				return Some( r );
			}
		}
/*
		for p in path.iter() {
			if self.name() == p {

			}
		}
*/
		*/
		None
	}

}

impl std::fmt::Debug for dyn UiElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//		writeln!( f,"[Trait] UiElement: {}x{} @ {}, {}", self.size().x, self.size().y, self.pos().x, self.pos().y )
		writeln!( f, "[Trait] UiElement" )
	}
}
