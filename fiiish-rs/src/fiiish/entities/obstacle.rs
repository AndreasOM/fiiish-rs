
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::EntityUpdateContext;
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Renderer
};

pub struct Obstacle {
	name: String,
	crc: u32,
	pos: Vector2,
	size: Vector2,
	rotation: f32,
	animated_texture: AnimatedTexture,
}

// rock-f 413 x 538
impl Obstacle {
	pub fn new( pos: &Vector2, crc: u32 ) -> Self {
		Self {
			name: String::new(),
			pos: *pos, //Vector2::zero(),
			crc: crc,
			size: Vector2::new( 413.0, 538.0 ),
			rotation: 0.0,
			animated_texture: AnimatedTexture::new(),
		}
	}

	pub fn set_rotation( &mut self, rotation: f32 ) {
		self.rotation = rotation;
	}
}

impl Entity for Obstacle {
	fn setup( &mut self, name: &str) {
		self.name = name.to_owned();
		self.animated_texture.setup( "rock-f", 0, 0, 1, 25.0 );
		match self.crc {
			0xd058353c => self.animated_texture.setup( "rock-a", 0, 0, 1, 25.0 ),
			0x49516486 => self.animated_texture.setup( "rock-b", 0, 0, 1, 25.0 ),
			0x3e565410 => self.animated_texture.setup( "rock-c", 0, 0, 1, 25.0 ),
			0xa032c1b3 => self.animated_texture.setup( "rock-d", 0, 0, 1, 25.0 ),
			0xd735f125 => self.animated_texture.setup( "rock-e", 0, 0, 1, 25.0 ),
			0x4e3ca09f => self.animated_texture.setup( "rock-f", 0, 0, 1, 25.0 ),
			0x6fe93bef => self.animated_texture.setup( "seaweed-a-", -2, 1, 47, 25.0 ),
			_ => self.animated_texture.setup( "block-1x1", 0, 0, 1, 25.0 ),
		};
		match self.crc {
			0xd058353c => self.size=Vector2::new( 150.0,  89.0 ),
			0x49516486 => self.size=Vector2::new( 204.0, 220.0 ),
			0x3e565410 => self.size=Vector2::new( 166.0, 373.0 ),
			0xa032c1b3 => self.size=Vector2::new( 197.0, 436.0 ),
			0xd735f125 => self.size=Vector2::new( 175.0, 556.0 ),
			0x4e3ca09f => self.size=Vector2::new( 413.0, 538.0 ),
			0x6fe93bef => self.size=Vector2::new( 63.0, 114.0 ),
			_ => self.size=Vector2::new( 128.0, 128.0 ),
		};
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
		renderer.use_layer( LayerId::Obstacles as u8 );
		renderer.use_effect( EffectId::Textured as u16 );
//		renderer.use_texture( "coin_01" );
		self.animated_texture.r#use( renderer );
		renderer.render_textured_quad_with_rotation( &self.pos, &self.size, self.rotation );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
