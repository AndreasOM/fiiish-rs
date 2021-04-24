use std::rc::Rc;
use std::cell::RefCell;

use std::sync::mpsc::{
	channel,
	Sender,
	Receiver,
};

use crate::DebugRenderer;
use crate::fiiish::PauseDialog;
use crate::fiiish::SettingsDialog;
use crate::fiiish::game::Game;
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;
use crate::math::Vector2;
use crate::renderer::{
	Color,
	Renderer,
	Texture,
};
use crate::system::System;

use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerHandle,
	UiElementFadeState,
	UiEvent,
	UiEventResponse,
	UiEventResponseButtonClicked,

	UiButton,
	UiGravityBox,
	UiHbox,
	UiImage,
	UiRenderer,
	UiToggleButton,
	UiVbox,
};

use crate::window_update_context::WindowUpdateContext;
use crate::fiiish::app_update_context::AppUpdateContext;

#[derive(Debug)]
pub struct GameUi {
//	root: Option< Box< dyn UiElement > >,
	root: Option< UiElementContainer >,
	size: Vector2,

	pause_togglebutton: Option< UiElementContainerHandle >,

	debug_renderer: Rc < Option < RefCell< DebugRenderer >  > >,

	event_response_sender: Sender< Box< dyn UiEventResponse > >,
	event_response_receiver: Receiver< Box< dyn UiEventResponse > >,

	game: Option< Rc< RefCell< Game > > >,
}

impl GameUi {
	pub fn new() -> Self {
		let ( tx, rx ) = channel();
		Self {
			root: None,
			size: Vector2::zero(),
			pause_togglebutton: None,
			debug_renderer:	Rc::new( None ),
			event_response_sender: tx,
			event_response_receiver: rx,
			game: None,
		}
	}

	pub fn enable_debug_renderer( &mut self, debug_renderer: &Rc< Option< RefCell< DebugRenderer > > > ) {
		self.debug_renderer = Rc::clone( debug_renderer );
	}

	pub fn disable_debug_renderer( &mut self ) {
		self.debug_renderer = Rc::new( None );
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer, game: &mut Rc< RefCell< Game > > ) {
		self.game = Some( game.clone() );

		let mut root = UiGravityBox::new( );
		root.set_padding( 16.0 );
		let mut root = UiElementContainer::new( Box::new( root ) );
		root.set_name( "root" );

		// setup dialogs

		match root.borrow_element_mut().as_any_mut().downcast_mut::<UiGravityBox>() {
				Some( root_gravity_box ) => {
					root_gravity_box.set_gravity( &Vector2::new( -1.0, 1.0 ) );
				},
				None => (),
		};

		let pause_dialog = root.add_child_element( PauseDialog::new( &mut self.game.as_mut().unwrap() )  );
		pause_dialog.borrow_mut().set_name( "PauseDialog" );
		pause_dialog.borrow_mut().fade_out( 0.0 );

		match root.borrow_element_mut().as_any_mut().downcast_mut::<UiGravityBox>() {
				Some( root_gravity_box ) => {
					root_gravity_box.set_gravity( &Vector2::new( 0.0, 0.0 ) );
				},
				None => (),
		};

		let settings_dialog = root.add_child_element( SettingsDialog::new( &mut self.game.as_mut().unwrap() )  );
		settings_dialog.borrow_mut().set_name( "SettingsDialog" );
		settings_dialog.borrow_mut().fade_out( 0.0 );

		root.layout( &Vector2::zero() );

//		root.dump_info( "", &Vector2::zero() );
//		todo!("die");

		root.fade_out( 0.0 );
		root.fade_in( 2.0 );
//		self.root = Some( Box::new( root ) );
		self.root = Some( root );
	}

	pub fn teardown( &mut self ) {
		self.root = None;
		self.game = None;
	}

	pub fn set_size( &mut self, size: &Vector2 ) {
		self.size = *size;
		if let Some( root ) = &mut self.root {
			root.set_size( &self.size );
			root.layout( &Vector2::zero() );
//			root.dump_info( "", &Vector2::zero() );
//			todo!("die");
		}		
	}

	fn toggle_settings_dialog( &mut self ) {
		println!("Toggling settings dialog");
		if let Some( root ) = &mut self.root {
			if let Some( mut settings_dialog ) = root.find_child_mut( &[ "SettingsDialog" ] ) {
				let mut settings_dialog = settings_dialog.borrow_mut();
				match settings_dialog.fade_state() {
					UiElementFadeState::FadedOut
					| UiElementFadeState::FadingOut( _ ) => {
						settings_dialog.fade_in( 1.0 );
					},
					UiElementFadeState::FadedIn
					| UiElementFadeState::FadingIn( _ ) => {
						settings_dialog.fade_out( 1.0 );
					},
				}
			}
		}
	}

	pub fn update( &mut self, wuc: &mut WindowUpdateContext, auc: &mut AppUpdateContext ) {
		if let Some( root ) = &mut self.root {

			if wuc.was_mouse_button_pressed( 0 ) {
				let cp = auc.cursor_pos();
				println!("Left Mouse Button was pressed @ {}, {}", cp.x, cp.y );
				let ev = UiEvent::MouseClick{ pos: *cp, button: 0 };
				if let Some( ev ) = root.handle_ui_event( &ev, &self.event_response_sender ) {
					println!( "Click handled" );
					self.event_response_sender.send( ev ).unwrap();
				}
			}

			if let Some( game ) = &mut self.game {
				let game = game.borrow();
				if let Some( mut pause_dialog ) = root.find_child_mut( &[ "PauseDialog" ] ) {
					let mut pause_dialog = pause_dialog.borrow_mut();
					if game.is_playing() {
						pause_dialog.fade_in( 1.0 );
					} else {
						pause_dialog.fade_out( 1.0 );
					}
				}
				if let Some( mut settings_dialog ) = root.find_child_mut( &[ "SettingsDialog" ] ) {
					let mut settings_dialog = settings_dialog.borrow_mut();
					if !game.is_paused() {
						settings_dialog.fade_out( 1.0 );
					}
				}
			}
			root.update( wuc.time_step() );

			if let Some( debug_renderer ) = &*self.debug_renderer {
				let mut debug_renderer = debug_renderer.borrow_mut();
				root.render_debug( &mut debug_renderer, &Vector2::zero() );
			}

		}

		// handle pending event responses
		if let Some( game ) = &mut self.game.clone() {
			let mut game = game.borrow_mut();
			let events: Vec< Box< dyn UiEventResponse > > = self.event_response_receiver.try_iter().collect();
			for ev in events {
	//			while let Some( ev ) =  ev_iter.next() {
				match ev.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
					Some( e ) => {
						println!("Button {} clicked", &e.button_name );
						match e.button_name.as_str() {
							"ButtonPause" => {
								game.toggle_pause();
							},
							"ButtonSettings" => {
									self.toggle_settings_dialog();
							},
							_ => {
								println!( "Unhandled button click from {}", &e.button_name );
							},
						}
					},
					None => {},
				};
			}
		}
	}
	pub fn render( &mut self, renderer: &mut Renderer) {
		if let Some( root ) = &mut self.root {
			// :CHEAT:
			renderer.use_layer( LayerId::Ui as u8 );
			renderer.use_effect( EffectId::ColoredTextured as u16 );

			let mut ui_renderer = UiRenderer::new( renderer );
			root.render( &mut ui_renderer );
		}
	}

}
