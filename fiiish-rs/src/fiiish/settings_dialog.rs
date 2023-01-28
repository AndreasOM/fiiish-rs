
use std::rc::Rc;
use std::cell::RefCell;

use std::sync::mpsc::Sender;

use crate::fiiish::game::Game;
use oml_game::math::Vector2;
use crate::renderer::Color;
use crate::ui::{
//	UiBlock,
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
	UiSpacer,
	UiToggleButton,
	UiVbox,
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

		{
			let mut parent = background.borrow_mut();
			let mut vbox = UiVbox::new();
			// hbox.set_padding( 0.0 );
			let mut vbox = parent.add_child_element( vbox );
			{
				let mut parent = vbox.borrow_mut();

				parent.add_child_element( UiSpacer::new( &Vector2::new( 16.0, 240.0 ), &Color::from_rgba( 0.2, 0.6, 0.8, 0.5 ) ) ); // top space

				let mut hbox = UiHbox::new();
				// hbox.set_padding( 0.0 );
				let mut hbox = parent.add_child_element( hbox );
				// center box with actual results
				{
					let mut parent = hbox.borrow_mut();
					parent.add_child_element( UiSpacer::new( &Vector2::new( 250.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.8, 0.5 ) ) ); // left space
					{
						let mut table_space = parent.add_child_element( UiSpacer::new( &Vector2::new( 620.0, 412.0 ), &Color::from_rgba( 0.2, 0.2, 0.8, 0.5 ) ) ); // placeholder

						let mut parent = table_space.borrow_mut();
						let mut vbox = UiVbox::new();
						vbox.set_padding( 0.0 );
						let mut vbox = parent.add_child_element( vbox );
						{
							let mut parent = vbox.borrow_mut();
							{
								let mut text_space = parent.add_child_element( UiSpacer::new( &Vector2::new( 620.0, 284.0 ), &Color::from_rgba( 0.2, 0.2, 0.8, 0.5 ) ) ); // placeholder
								// 284

								let mut parent = text_space.borrow_mut();
								let mut vbox = UiVbox::new();
								vbox.set_padding( 0.0 );
								let mut vbox = parent.add_child_element( vbox );
								{
									let mut text_box = vbox.borrow_mut();
									let mut l = UiLabel::new( &Vector2::new( 620.0, 64.0 ), "Fiiish! RS" );
									l.set_alignment( &Vector2::zero() );
									text_box.add_child_element( l );
									const VERSION: &'static str = env!("CARGO_PKG_VERSION");
									let t = format!("Version {}\nNon-final alpha version!\nThanks for testing...", VERSION );
									let mut l = UiLabel::new( &Vector2::new( 620.0, 220.0 ), &t );
									l.set_alignment( &Vector2::new( -1.0, 1.0 ) );
									text_box.add_child_element( l );
								}
							}
							{
								let mut hbox = UiHbox::new();
								hbox.set_padding( 16.0 );

//								let mut parent = parent.borrow_mut();
								let mut hbox = parent.add_child_element( hbox ).borrow_mut();

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
						}
					}
					parent.add_child_element( UiSpacer::new( &Vector2::new( 154.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.2, 0.5 ) ) ); // right space
				}
				parent.add_child_element( UiSpacer::new( &Vector2::new( 16.0, 54.0 ), &Color::from_rgba( 0.5, 0.5, 0.5, 0.5 ) ) ); // middle space
				{
					// button row

					let mut hbox = UiHbox::new();
					// hbox.set_padding( 0.0 );
					let mut hbox = parent.add_child_element( hbox );
					{
						let mut parent = hbox.borrow_mut();
						parent.add_child_element( UiSpacer::new( &Vector2::new( 128.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.8, 0.5 ) ) ); // left space
						{
							let mut button_space = parent.add_child_element( UiSpacer::new( &Vector2::new( 384.0, 128.0 ), &Color::from_rgba( 0.5, 0.5, 0.5, 0.5 ) ) ); // placeholder for buttons

							{
								let mut parent = button_space.borrow_mut();
								let mut hbox = UiHbox::new();
								// hbox.set_padding( 0.0 );
								let mut hbox = parent.add_child_element( hbox );
								/*
								{
									let mut parent = hbox.borrow_mut();
									let mut play_button = parent.add_child_element( UiButton::new( "button_play", &Vector2::new( 128.0, 128.0 ) ) );
									play_button.borrow_mut().set_name( "PlayButton" );
									self.play_button = Some( play_button.clone() );
									parent.add_child_element( UiSpacer::new( &Vector2::new( 256.0, 16.0 ), &Color::from_rgba( 0.8, 0.6, 0.2, 0.5 ) ) ); // right space
								}
								*/
							}
						}
						parent.add_child_element( UiSpacer::new( &Vector2::new( 512.0, 16.0 ), &Color::from_rgba( 0.8, 0.2, 0.2, 0.5 ) ) ); // right space
					}
				}
				parent.add_child_element( UiSpacer::new( &Vector2::new( 16.0, 190.0 ), &Color::from_rgba( 0.5, 0.5, 0.5, 0.5 ) ) ); // bottom space
			}
		}
		container.set_size( background.borrow().size() );
	}
	fn update( &mut self, _container: &UiElementContainerData, _time_step: f64 ) {
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
	fn handle_ui_event_response( &mut self, response: Box< dyn UiEventResponse > ) -> Option< Box< dyn UiEventResponse > > {
		match response.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
			Some( e ) => {
				println!("SettingsDialog: Button {} clicked", &e.button_name );
				match e.button_name.as_str() {
					"MusicToggleButton" => {
						self.game.borrow_mut().toggle_music();
						return None;
					},
					"SoundToggleButton" => {
						self.game.borrow_mut().toggle_sound();
						return None;
					},
					_ => {
//						println!( "Unhandled button click from {}", &e.button_name );
					},
				}
			},
			_ => {},
		}
		Some( response )
	}

	fn preferred_size( &self ) -> Option< &Vector2 > {
		Some( &self.size )
	}
}
