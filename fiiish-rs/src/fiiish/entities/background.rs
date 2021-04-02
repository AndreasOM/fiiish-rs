
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::EntityUpdateContext;
use crate::math::{
	Matrix32,
	Vector2
};
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
		let a = renderer.aspect_ratio();
		let mtx = Matrix32::scaling_xy( 1.0*a, 1.0 );
		renderer.set_tex_matrix( &mtx );
		renderer.render_textured_fullscreen_quad();
		renderer.set_tex_matrix( &Matrix32::identity() );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
