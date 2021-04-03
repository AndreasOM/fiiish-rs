
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::EntityUpdateContext;
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Renderer
};

pub struct Coin {
	name: String,
	crc: u32,
	pos: Vector2,
	size: Vector2,
	animated_texture: AnimatedTexture,
	animation_offset: u16,
}

impl Coin {
	pub fn new(pos: &Vector2, animation_offset: u16, crc: u32 ) -> Self {
		Self {
			name: String::new(),
			crc: crc,
			pos: *pos, //Vector2::zero(),
			size: Vector2::new( 32.0, 32.0 ),
			animated_texture: AnimatedTexture::new(),
			animation_offset,
		}
	}
}

impl Entity for Coin {
	fn setup( &mut self, ec: &EntityConfiguration ) {
		self.size = ec.size;
		self.animated_texture.setup_from_config( &ec.animated_texture_configuration );
	}


	fn teardown( &mut self ){

	}

	fn update( &mut self, euc: &mut EntityUpdateContext ){
		self.animated_texture.update( euc.time_step() );
		self.pos.x += euc.world_movement().x;
		if self.pos.x < -1500.0 {
			self.pos.x += 2.0* 1500.0;
		}
	}

	fn render( &mut self, renderer: &mut Renderer ){
		renderer.use_layer( LayerId::Pickups as u8 );
		renderer.use_effect( EffectId::Textured as u16 );
		self.animated_texture.r#use( renderer );
		renderer.render_textured_quad( &self.pos, &self.size );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
