use std::rc::Rc;
use std::cell::RefCell;

use crate::DebugRenderer;
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
	UiEvent,
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
}

impl GameUi {
	pub fn new() -> Self {
		Self {
			root: None,
			size: Vector2::zero(),
			pause_togglebutton: None,
			debug_renderer:	Rc::new( None ),
		}
	}

	pub fn enable_debug_renderer( &mut self, debug_renderer: &Rc< Option< RefCell< DebugRenderer > > > ) {
		self.debug_renderer = Rc::clone( debug_renderer );
	}

	pub fn disable_debug_renderer( &mut self ) {
		self.debug_renderer = Rc::new( None );
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		let mut root = UiGravityBox::new( );
		root.set_padding( 16.0 );
		// :HACK:
		root.set_gravity( &Vector2::new( -1.0, 1.0 ) );
		let mut root = UiElementContainer::new( Box::new( root ) );

		{
			let mut pause_menu = UiHbox::new();
			pause_menu.set_padding( 16.0 );

			// :TODO: unhack for HACK above
//			root.borrow_element_mut().set_gravity( &Vector2::new( -1.0, 1.0 ) );
			let mut pause_menu = root.add_child_element( pause_menu ).borrow_mut();
			pause_menu.set_name( "PauseMenu" );
			pause_menu.fade_out( 0.0 );
//			pause_menu.fade_in( 3.0 );

//			pause_menu.add_child_element( UiImage::new( "button_pause", &Vector2::new( 128.0, 128.0 ) ) );
			let pause_togglebutton = pause_menu.add_child_element( UiToggleButton::new( "button_pause", "button_play", &Vector2::new( 128.0, 128.0 ) ) );
			{
				let mut p = pause_togglebutton.borrow_mut();
				p.set_name( "ButtonPause" );
			}
			self.pause_togglebutton = Some( pause_togglebutton.clone() );

			let mut button_settings = pause_menu.add_child_element( UiButton::new( "button_settings", &Vector2::new( 128.0, 128.0 ) ) ).borrow_mut();
			button_settings.set_name( "ButtonSettings" );
			button_settings.fade_out( 0.0 );

//			pause_menu.add_child_element( UiImage::new( "button_fiiish", &Vector2::new( 128.0, 128.0 ) ) );
		}

		// example
		/*
		{
			if let Some( button_settings ) = root.find_child_mut( &[ "PauseMenu", "ButtonSettings" ] ) {
				button_settings.fade_out( 0.0 );
			}

//			todo!("die");
		}
		*/

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

	}

	pub fn update( &mut self, game: &mut Game, wuc: &mut WindowUpdateContext, auc: &mut AppUpdateContext ) {
		if let Some( root ) = &mut self.root {

			if wuc.was_mouse_button_pressed( 0 ) {
				let cp = auc.cursor_pos();
				println!("Left Mouse Button was pressed @ {}, {}", cp.x, cp.y );
				let ev = UiEvent::MouseClick{ pos: *cp, button: 0 };
				if let Some( r ) = root.handle_ui_event( &ev ) {
					println!("Click handled -> {:?}", &r );
					match r.as_any().downcast_ref::<UiEventResponseButtonClicked>() {
						Some( e ) => {
							println!("Button {} clicked", &e.button_name );
							match e.button_name.as_str() {
								"ButtonPause" => {
									game.toggle_pause();
								},
								"ButtonSettings" => {
//									self.toggle_settings_dialog();
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

			if let Some( mut pause_menu ) = root.find_child_mut( &[ "PauseMenu" ] ) {
				let mut pause_menu = pause_menu.borrow_mut();
				if game.is_playing() {
					pause_menu.fade_in( 1.0 );
				} else {
					pause_menu.fade_out( 1.0 );
				}
			}
			if let Some( mut settings_button ) = root.find_child_mut( &[ "PauseMenu", "ButtonSettings" ] ) {
				let mut settings_button = settings_button.borrow_mut();
				if game.is_paused() {
					settings_button.fade_in( 1.0 );
				} else {
					settings_button.fade_out( 1.0 );
				}
			}
			if let Some( p ) = &mut self.pause_togglebutton {
				let mut p = p.borrow_mut();
				let p = p.borrow_element_mut();
//				let tb: UiToggleButton = dynamic_cast<UiToggleButton>( p );
				let tb: &mut UiToggleButton = match p.as_any_mut().downcast_mut::<UiToggleButton>() {
					Some(p) => p,
					None => panic!("{:?} isn't a UiToggleButton!", &p),
				};
				if game.is_paused() {
					tb.goto_b();
				} else {
					tb.goto_a();
				}
			}

			root.update( wuc.time_step() );

			if let Some( debug_renderer ) = &*self.debug_renderer {
				let mut debug_renderer = debug_renderer.borrow_mut();
				root.render_debug( &mut debug_renderer, &Vector2::zero() );
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
