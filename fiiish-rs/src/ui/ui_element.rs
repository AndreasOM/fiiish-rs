
use crate::math::Vector2;
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
	fn update( &mut self, time_step: f64 );
	fn render( &self, ui_renderer: &mut UiRenderer) {
		self.render_children( ui_renderer );
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
		false
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
