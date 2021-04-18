
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

	fn handle_ui_event( &mut self, event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
		false
		/*
		dbg!(&event);
		match event {
			UiEvent::MouseClick{ pos, button } => {
				let pos = pos.sub( self.pos() );
				if self.is_hit_by( &pos ) {
//					println!( "Hit with {} children", self.borrow_base_mut().children.len() );
					for c in self.borrow_base_mut().children.iter_mut() {
						let cpos = pos.sub( c.pos() );
//						let pos = *pos;
//						println!("New pos: {},{} (child @ {}, {} -> {}, {})", pos.x, pos.y , c.pos().x, c.pos().y, cpos.x, cpos.y );
						if c.is_hit_by( &cpos ) {
//							println!("Child is hit");
							let ev = UiEvent::MouseClick{ pos, button: *button };
							if c.handle_ui_event( &ev ) {
								return true;
							}
						} else {
//							println!("Child NOT hit");
						}
					}
					false
				} else {
//					println!( "Not hit" );
					false
				}
			},
			_ => false,
		}
		*/
	}

	// local coordinates!
	fn is_hit_by( &self, pos: &Vector2 ) -> bool {
		false
		/* :TODO:
//		dbg!(pos, self.pos(), self.size() );
		let hs = self.size().scaled( 0.5 );
//		let bl = self.pos().sub( &hs );
//		let tr = self.pos().add( &hs );
		let bl = Vector2::zero().sub( &hs );
		let tr = Vector2::zero().add( &hs );
//		dbg!(&pos,&tl,&br);
//		println!("is_hit_by {:?} {:?}-{:?}", &pos, &bl, &tr);
		pos.x >= bl.x
		&& pos.y >= bl.y
		&& pos.x <= tr.x
		&& pos.y <= tr.y
		*/
	}
}

impl std::fmt::Debug for dyn UiElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//		writeln!( f,"[Trait] UiElement: {}x{} @ {}, {}", self.size().x, self.size().y, self.pos().x, self.pos().y )
		writeln!( f, "[Trait] UiElement" )
	}
}
