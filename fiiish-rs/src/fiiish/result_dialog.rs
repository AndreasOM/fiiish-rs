
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
pub struct ResultDialog {
	game: Rc< RefCell< Game > >,
}

impl ResultDialog {
	pub fn new( game: &mut Rc< RefCell< Game > > ) -> Self {
		Self {
			game: game.clone(),
		}
	}
}

impl UiElement for ResultDialog {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn setup_within_container( &mut self, container: &mut UiElementContainerData ) {
		let mut background = container.add_child_element( UiImage::new( "screen_frame", &Vector2::new( 1024.0, 1024.0 ) ) );

		{
			let mut parent = background.borrow_mut();
			let mut vbox = UiVbox::new();
			// hbox.set_padding( 0.0 );
			let mut vbox = parent.add_child_element( vbox );
			{
				let mut parent = vbox.borrow_mut();

				parent.add_child_element( UiBlock::new( &Vector2::new( 16.0, 240.0 ), &Color::from_rgba( 0.2, 0.6, 0.8, 0.5 ) ) ); // top space

				let mut hbox = UiHbox::new();
				// hbox.set_padding( 0.0 );
				let mut hbox = parent.add_child_element( hbox );
				// center box with actual results
				{
					let mut parent = hbox.borrow_mut();
					parent.add_child_element( UiBlock::new( &Vector2::new( 350.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.8, 0.5 ) ) ); // left space
					{
						parent.add_child_element( UiBlock::new( &Vector2::new( 520.0, 412.0 ), &Color::from_rgba( 0.2, 0.2, 0.8, 0.5 ) ) ); // placeholder
					}
					parent.add_child_element( UiBlock::new( &Vector2::new( 154.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.2, 0.5 ) ) ); // right space
				}
				parent.add_child_element( UiBlock::new( &Vector2::new( 16.0, 54.0 ), &Color::from_rgba( 0.5, 0.5, 0.5, 0.5 ) ) ); // middle space
				{
					// button row

					let mut hbox = UiHbox::new();
					// hbox.set_padding( 0.0 );
					let mut hbox = parent.add_child_element( hbox );
					{
						let mut parent = hbox.borrow_mut();
						parent.add_child_element( UiBlock::new( &Vector2::new( 128.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.8, 0.5 ) ) ); // left space
						{
							let mut button_space = parent.add_child_element( UiBlock::new( &Vector2::new( 384.0, 128.0 ), &Color::from_rgba( 0.5, 0.5, 0.5, 0.5 ) ) ); // placeholder for buttons

							{
								let mut parent = button_space.borrow_mut();
								let mut hbox = UiHbox::new();
								// hbox.set_padding( 0.0 );
								let mut hbox = parent.add_child_element( hbox );
								{
									let mut parent = hbox.borrow_mut();
									let mut play_button = parent.add_child_element( UiButton::new( "button_play", &Vector2::new( 128.0, 128.0 ) ) );
									play_button.borrow_mut().set_name( "PlayButton" );
									parent.add_child_element( UiBlock::new( &Vector2::new( 256.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.2, 0.5 ) ) ); // right space
								}
							}
						}
						parent.add_child_element( UiBlock::new( &Vector2::new( 512.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.2, 0.5 ) ) ); // right space
					}
				}
				parent.add_child_element( UiBlock::new( &Vector2::new( 16.0, 190.0 ), &Color::from_rgba( 0.5, 0.5, 0.5, 0.5 ) ) ); // bottom space
			}
		}
		/*

		let mut hbox = UiHbox::new();
		hbox.set_padding( 16.0 );

		let mut background = background.borrow_mut();
		let mut hbox = background.add_child_element( hbox ).borrow_mut();

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
		*/
		container.set_size( background.borrow().size() );
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
