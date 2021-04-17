
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
	UiGravityBox,
	UiHbox,
	UiImage,
	UiRenderer,
	UiVbox,
};

use crate::window_update_context::WindowUpdateContext;
use crate::fiiish::app_update_context::AppUpdateContext;

#[derive(Debug)]
pub struct GameUi {
//	root: Option< Box< dyn UiElement > >,
	root: Option< UiGravityBox >,
	size: Vector2,
}

impl GameUi {
	pub fn new() -> Self {
		Self {
			root: None,
			size: Vector2::zero(),
		}
	}
	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		let mut root = UiGravityBox::new( );
		root.set_padding( 16.0 );

		{
			let mut pause_menu = UiHbox::new();
			pause_menu.set_padding( 16.0 );
			pause_menu.add_child( Box::new( UiImage::new( "button_pause", &Vector2::new( 128.0, 128.0 ) ) ) );
			pause_menu.add_child( Box::new( UiImage::new( "button_settings", &Vector2::new( 128.0, 128.0 ) ) ) );
			pause_menu.add_child( Box::new( UiImage::new( "button_fiiish", &Vector2::new( 128.0, 128.0 ) ) ) );
			pause_menu.fade_out( 0.0 );
			pause_menu.fade_in( 3.0 );
			root.add_child( Box::new( pause_menu ), &Vector2::new( -1.0, 1.0 ) );
		}

		root.layout( &Vector2::zero() );
		root.fade_out( 0.0 );
		root.fade_in( 2.0 ); // ten seconds? yes, just for testing
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
		}		
	}

	pub fn update( &mut self, wuc: &mut WindowUpdateContext, _auc: &mut AppUpdateContext ) {
		if let Some( root ) = &mut self.root {
			root.update( wuc.time_step() );
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
