
use std::rc::Rc;
use std::cell::RefCell;

use std::sync::mpsc::Sender;

use crate::fiiish::game::Game;
use crate::math::Vector2;
use crate::renderer::Color;
use crate::ui::{
	UiBlock,
	UiButton,
	UiElement,
	UiElementContainer,
	UiElementContainerData,
	UiElementContainerHandle,
	UiEvent,
	UiEventResponse,
	UiEventResponseButtonClicked,
	UiImage,
	UiHbox,
	UiLabel,
	UiToggleButton,
	UiVbox,
};

#[derive(Debug)]
pub struct CounterDialog {
	game: Rc< RefCell< Game > >,
}

impl CounterDialog {
	pub fn new( game: &mut Rc< RefCell< Game > > ) -> Self {
		Self {
			game: game.clone(),
		}
	}
}

impl UiElement for CounterDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let mut vbox = UiVbox::new();
		vbox.set_padding( 16.0 );
		let mut vbox = container.add_child_element( vbox );

		let mut vbox = vbox.borrow_mut();

		{
			let mut hbox = UiHbox::new();
			hbox.set_padding( 16.0 );
			let mut hbox = UiElementContainer::new_from_element( hbox );
			hbox.add_child_element( UiLabel::new( &Vector2::new( 128.0, 64.0 ), "[DISTANCE]" ) );
			hbox.add_child_element( UiImage::new( "mini_icon_flag", &Vector2::new( 64.0, 64.0 ) ) );
			vbox.add_child( hbox );
		}

		{
			let mut hbox = UiHbox::new();
			hbox.set_padding( 16.0 );
			let mut hbox = UiElementContainer::new_from_element( hbox );
			hbox.add_child_element( UiLabel::new( &Vector2::new( 128.0, 64.0 ), "[COINS]" ) );
			hbox.add_child_element( UiImage::new( "mini_icon_coin", &Vector2::new( 64.0, 64.0 ) ) );
			vbox.add_child( hbox );
		}

		container.set_size( vbox.size() );
	}
	fn update( &mut self, _container: &UiElementContainerData, _time_step: f64 ) {
		let game = self.game.borrow_mut();

		// :TODO: update coin & distance labels
		/*
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
		*/
	}

	fn handle_ui_event( &mut self, _container: &mut UiElementContainerData, _event: &UiEvent, _event_sender: &Sender< Box< dyn UiEventResponse > > ) -> Option< Box < dyn UiEventResponse > > {
		// stop traversal into children
		None
	}
}
