
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
}

impl SettingsDialog {
	pub fn new( ) -> Self {
		Self {
			size: Vector2::new( 1024.0, 1024.0 ),
			music_togglebutton: None,
			sound_togglebutton: None,
		}
	}

	pub fn update_from_game( &mut self, game: &mut Game ) {
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

		// :TODO: unhack for HACK above
//			root.borrow_element_mut().set_gravity( &Vector2::new( -1.0, 1.0 ) );
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



//		let image = container.add_child_element( UiImage::new( &self.imagename, &self.imagesize ) );
//		self.image = Some( image );
	}
	fn handle_ui_event( &mut self, container: &mut UiElementContainerData, _event: &UiEvent, event_sender: &Sender< Box< dyn UiEventResponse > > ) -> bool {
/*		
		println!("Button clicked");
		let ev = Box::new( UiEventResponseButtonClicked{ button_name: container.name.clone() } );
		event_sender.send( ev ).unwrap();

		true
*/
		false
	}
	fn preferred_size( &self ) -> Option< &Vector2 > {
//		Some( &self.imagesize )
		Some( &self.size )
//		None
	}
}
