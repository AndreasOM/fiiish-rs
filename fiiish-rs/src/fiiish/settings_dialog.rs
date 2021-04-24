
use std::rc::Rc;
use std::cell::RefCell;

use std::sync::mpsc::Sender;

use crate::fiiish::game::Game;
use crate::math::Vector2;
use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerData,
	UiElementContainerHandle,
	UiEvent,
	UiEventResponse,
	UiEventResponseButtonClicked,
	UiImage,
	UiHbox,
	UiToggleButton,
};

#[derive(Debug)]
pub struct SettingsDialog {
	size: Vector2,
	music_togglebutton: Option< UiElementContainerHandle >,
	sound_togglebutton: Option< UiElementContainerHandle >,
	game: Rc< RefCell< Game > >,
}

impl SettingsDialog {
	pub fn new( game: &mut Rc< RefCell< Game > > ) -> Self {
		Self {
			size: Vector2::new( 1024.0, 1024.0 ),
			music_togglebutton: None,
			sound_togglebutton: None,
			game: game.clone(),
		}
	}
}

impl UiElement for SettingsDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let mut background = container.add_child_element( UiImage::new( "screen_frame_2", &self.size ) );

		let mut hbox = UiHbox::new();
		hbox.set_padding( 16.0 );

		let mut background = background.borrow_mut();
		let mut hbox = background.add_child_element( hbox ).borrow_mut();

		let music_togglebutton = hbox.add_child_element( UiToggleButton::new( "button_music_on", "button_music_off", &Vector2::new( 128.0, 128.0 ) ) );
		{
			let mut p = music_togglebutton.borrow_mut();
			p.set_name( "MusicToggleButton" );
		}
		self.music_togglebutton = Some( music_togglebutton.clone() );
		let sound_togglebutton = hbox.add_child_element( UiToggleButton::new( "button_sound_on", "button_sound_off", &Vector2::new( 128.0, 128.0 ) ) );
		{
			let mut p = sound_togglebutton.borrow_mut();
			p.set_name( "SoundToggleButton" );
		}
		self.sound_togglebutton = Some( sound_togglebutton.clone() );
	}
	fn update( &mut self, _time_step: f64 ) {
		let game = self.game.borrow_mut();
		if let Some( music_togglebutton ) = &mut self.music_togglebutton {
			let mut music_togglebutton = music_togglebutton.borrow_mut();
			let music_togglebutton = music_togglebutton.borrow_element_mut();
			match music_togglebutton.as_any_mut().downcast_mut::<UiToggleButton>() {
				Some( mtb ) => {
					if game.is_music_enabled() {
						mtb.goto_a();
					} else {
						mtb.goto_b();
					}
				},
				None => panic!("{:?} isn't a UiToggleButton!", &music_togglebutton),
			};
		}
		if let Some( sound_togglebutton ) = &mut self.sound_togglebutton {
			let mut sound_togglebutton = sound_togglebutton.borrow_mut();
			let sound_togglebutton = sound_togglebutton.borrow_element_mut();
			match sound_togglebutton.as_any_mut().downcast_mut::<UiToggleButton>() {
				Some( mtb ) => {
					if game.is_sound_enabled() {
						mtb.goto_a();
					} else {
						mtb.goto_b();
					}
				},
				None => panic!("{:?} isn't a UiToggleButton!", &sound_togglebutton),
			};
		}
	}

	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.size )
	}
}
