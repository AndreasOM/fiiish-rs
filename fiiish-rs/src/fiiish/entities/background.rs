
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
		// fake long running time to simulate precision loss
		self.pos.x = - 2.0 * 60.0 * 60.0 * 60.0 * 240.0 * 0.5;
	}

	fn teardown( &mut self ){

	}

	fn update( &mut self, euc: &mut EntityUpdateContext ){
		self.pos.x -= euc.time_step() as f32 * 240.0 * 0.5; // :HACK: speed is roughly guestimated to feel kind of nearly right
		// repeat value to avoid precision loss
		while self.pos.x < -1024.0 {
			self.pos.x += 1024.0;
		}
	}

	fn render( &mut self, renderer: &mut Renderer ){
		renderer.use_layer( LayerId::Background as u8 );
		renderer.use_effect( EffectId::Background as u16 );
		renderer.use_texture( "background" );
		let a = renderer.aspect_ratio();
		let mut mtx = Matrix32::scaling_xy( 1.0*a, 1.0 );
		mtx.pos.x = - self.pos.x / 1024.0;
		renderer.set_tex_matrix( &mtx );
		renderer.render_textured_fullscreen_quad();
		renderer.set_tex_matrix( &Matrix32::identity() );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
