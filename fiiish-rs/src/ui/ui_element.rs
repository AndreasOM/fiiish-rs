
use crate::DebugRenderer;
use crate::math::Vector2;
use crate::renderer::Color;
use crate::ui::{
	UiElementBase,
	UiEvent,
	UiRenderer,
};

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct UiElementFadeData {
	level: f32,
	speed: f32,
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum UiElementFadeState {
	FadedOut,
	FadingIn( UiElementFadeData ),
	FadedIn,
	FadingOut( UiElementFadeData ),
}

pub trait UiElement {
	fn update( &mut self, time_step: f64 ) {
		self.update_fade_state( time_step );
		for c in self.borrow_base_mut().children.iter_mut() {
			c.update( time_step );
		}
	}
	fn render( &self, ui_renderer: &mut UiRenderer) {
		self.render_children( ui_renderer );
	}
	fn render_debug( &self, debug_renderer: &mut DebugRenderer, offset: &Vector2 ) {
		for c in self.borrow_base().children.iter() {
			let co = offset.add( c.pos() );
			c.render_debug( debug_renderer, &co );
		}
		debug_renderer.add_line( &Vector2::zero(), &Vector2::zero().add( &offset ), 3.0, &Color::white() );
	}

	fn dump_info( &self, indent: &str, offset: &Vector2 ) {
		println!("{} {},{} {},{}", indent, self.pos().x, self.pos().y, self.size().x, self.size().y );
		let new_indent = format!("{}  ", indent);
		for c in self.borrow_base().children.iter() {
			let co = offset;//.add( c.pos() );
			c.dump_info( &new_indent, &co );
		}
	}

	fn render_children( &self, ui_renderer: &mut UiRenderer ) {
		if *self.fade_state() != UiElementFadeState::FadedOut {
			ui_renderer.push_translation( &self.borrow_base().pos );
			let l = self.get_fade_level();
			ui_renderer.push_opacity( l );
			for c in self.borrow_base().children.iter() {
				c.render( ui_renderer );
			}
			ui_renderer.pop_opacity();
			ui_renderer.pop_transform();
		}		
	}
	fn layout( &mut self, pos: &Vector2 ){}
	fn handle_ui_event( &mut self, event: &UiEvent ) -> bool {	// bool will change to ... Option< Something >
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
	}

	// local coordinates!
	fn is_hit_by( &self, pos: &Vector2 ) -> bool {
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
	}

	fn borrow_base( &self ) -> &UiElementBase;
	fn borrow_base_mut( &mut self ) -> &mut UiElementBase;

	fn size( &self ) -> &Vector2 {
		&self.borrow_base().size
	}
	fn pos( &self ) -> &Vector2 {
		&self.borrow_base().pos
	}

	fn fade_state( &self ) -> &UiElementFadeState {
		&self.borrow_base().fade_state
	}
	fn set_fade_state( &mut self, fade_state: &UiElementFadeState ) {
		self.borrow_base_mut().fade_state = *fade_state;
	}

	fn fade_in( &mut self, duration: f32 ) {
		let fs = self.fade_state();
		if duration == 0.0 {
			self.set_fade_state( &UiElementFadeState::FadedIn );
		} else {
			let speed = 1.0/duration;
			match fs {
				UiElementFadeState::FadedIn => (),
				UiElementFadeState::FadedOut => {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: 0.0, speed } );
					self.set_fade_state( &fs );
				},
				UiElementFadeState::FadingIn( d ) => {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: d.level, speed } );
					self.set_fade_state( &fs );
				},
				UiElementFadeState::FadingOut( d ) => {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: d.level, speed } );
					self.set_fade_state( &fs );
				}
			}
		}
	}
	fn fade_out( &mut self, duration: f32 ) {
		let fs = self.fade_state();
		if duration == 0.0 {
			self.set_fade_state( &UiElementFadeState::FadedOut );
		} else {
			let speed = 1.0/duration;
			match fs {
				UiElementFadeState::FadedOut => (),
				UiElementFadeState::FadedIn => {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: 1.0, speed } );
					self.set_fade_state( &fs );
				},
				UiElementFadeState::FadingIn( d ) => {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: d.level, speed } );
					self.set_fade_state( &fs );
				},
				UiElementFadeState::FadingOut( d ) => {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: d.level, speed } );
					self.set_fade_state( &fs );
				}
			}
		}
	}
	fn update_fade_state( &mut self, time_step: f64 ) {
		let fs = self.fade_state();
		match fs {
			UiElementFadeState::FadedOut => (),
			UiElementFadeState::FadedIn => (),
			UiElementFadeState::FadingIn( d ) => {
				let new_level = d.level + d.speed * time_step as f32;
				if new_level < 1.0 {
					let fs = UiElementFadeState::FadingIn( UiElementFadeData{ level: new_level, speed: d.speed } );
					self.set_fade_state( &fs );					
				} else {
					self.set_fade_state( &UiElementFadeState::FadedIn );
				}
			}
			UiElementFadeState::FadingOut( d ) => {
				let new_level = d.level - d.speed * time_step as f32;
				if new_level > 0.0 {
					let fs = UiElementFadeState::FadingOut( UiElementFadeData{ level: new_level, speed: d.speed } );
					self.set_fade_state( &fs );					
				} else {
					self.set_fade_state( &UiElementFadeState::FadedOut );
				}
			}
		}
	}
	fn get_fade_level( &self ) -> f32 {
		let fs = self.fade_state();
		match fs {
			UiElementFadeState::FadedOut => 0.0,
			UiElementFadeState::FadedIn => 1.0,
			UiElementFadeState::FadingIn( d ) => d.level,
			UiElementFadeState::FadingOut( d ) => d.level,
		}
	}
}

impl std::fmt::Debug for dyn UiElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!( f,"[Trait] UiElement: {}x{} @ {}, {}", self.size().x, self.size().y, self.pos().x, self.pos().y )
	}
}
