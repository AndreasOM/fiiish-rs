
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::EntityUpdateContext;
use crate::math::Vector2;
use crate::renderer::{
	Renderer
};

pub struct Background {
	name: String,
	pos: Vector2,
}

impl Background {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			pos: Vector2::zero(),
		}
	}
}

impl Entity for Background {
	fn setup( &mut self, name: &str) {
		self.name = name.to_owned();
	}

	fn teardown( &mut self ){

	}

	fn update( &mut self, euc: &mut EntityUpdateContext ){
	}

	fn render( &mut self, renderer: &mut Renderer ){
		renderer.use_layer( LayerId::Background as u8 );
		renderer.use_effect( EffectId::Textured as u16 );
		renderer.use_texture( "background" );
		let size =Vector2::new( 1024.0, 1024.0 );	// :TODO: actually render fullscreen
		renderer.render_textured_quad( &self.pos, &size );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
