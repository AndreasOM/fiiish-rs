
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
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
	fn setup( &mut self, name: &str ) {
		self.name = name.to_owned();
		match self.crc {
			0xe4c651aa => self.animated_texture.setup( "coin_", 2, 1, 32, 25.0 ),
			0x06fd4c5a => self.animated_texture.setup( "coin_blue_", 2, 1, 32, 25.0 ),
			0xf75fd92f => self.animated_texture.setup( "coin_green_", 2, 1, 32, 25.0 ),
			0x235a41dd => self.animated_texture.setup( "magnet_", 2, 1, 32, 25.0 ),
			_ => {},
		}
		
		self.animated_texture.set_current_frame( self.animation_offset );
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
//		renderer.use_texture( "coin_01" );
		self.animated_texture.r#use( renderer );
		renderer.render_textured_quad( &self.pos, &self.size );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
