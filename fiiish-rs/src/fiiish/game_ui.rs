
use crate::math::Vector2;
use crate::renderer::{
	Color,
	Renderer,
	Texture,
};
use crate::system::System;

use crate::ui::{
	UiElement,
	UiHbox,
	UiImage,
	UiRenderer,
	UiVbox,
};

use crate::window_update_context::WindowUpdateContext;
use crate::fiiish::app_update_context::AppUpdateContext;

#[derive(Debug)]
pub struct GameUi {
	root: Box< dyn UiElement >,
}

impl GameUi {
	pub fn new() -> Self {
/* for layout testing
		let mut root = Box::new( UiVbox::new() );
		root.set_padding( 16.0 );
*/
		let mut root = Box::new( UiHbox::new() );
		root.set_padding( 16.0 );
		root.add_child( Box::new( UiImage::new( "button_pause", &Vector2::new( 128.0, 128.0 ) ) ) );
		root.add_child( Box::new( UiImage::new( "button_settings", &Vector2::new( 128.0, 128.0 ) ) ) );
		root.add_child( Box::new( UiImage::new( "button_fiiish", &Vector2::new( 128.0, 128.0 ) ) ) );
//		root1.layout( &Vector2::zero() );
/* for layout testing
		root.add_child( root1 );
		for _ in 0..4 {
			let mut root2 = Box::new( UiHbox::new() );
			root2.set_padding( 0.0 );
			root2.add_child( Box::new( UiImage::new( "button_pause", &Vector2::new( 2.0*128.0, 128.0 ) ) ) );
			root2.add_child( Box::new( UiImage::new( "button_settings", &Vector2::new( 128.0, 0.5*128.0 ) ) ) );
			root2.add_child( Box::new( UiImage::new( "button_fiiish", &Vector2::new( 2.0*128.0, 128.0 ) ) ) );
	//		root2.layout( &Vector2::new( 0.0, 128.0 ) );
			root.add_child( root2 );
		}
*/
		root.layout( &Vector2::zero() );

//		dbg!(&root);
//		todo!("die");

		Self {
//			root: Box::new( UiImage::new( "button_pause", &Vector2::new( 128.0, 128.0 ) ) ),
			root,
		}

	}
	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {

	}
	pub fn teardown( &mut self ) {
	}

	pub fn update( &mut self, wuc: &mut WindowUpdateContext, _auc: &mut AppUpdateContext ) {
		self.root.update( wuc.time_step() );
	}
	pub fn render( &mut self, renderer: &mut Renderer) {
		let mut ui_renderer = UiRenderer::new( renderer );
		self.root.render( &mut ui_renderer );
	}

}
