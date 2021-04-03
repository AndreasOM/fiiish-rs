
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::entities::EntityId;
use crate::fiiish::entities::entity_ids::*;

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
			name: "obstacle".to_string(),
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
		renderer.use_layer( LayerId::Obstacles as u8 );
		renderer.use_effect( EffectId::Textured as u16 );
		self.animated_texture.r#use( renderer );
		renderer.render_textured_quad_with_rotation( &self.pos, &self.size, self.rotation );
	}

	fn name( &self ) -> &str {
		&self.name
	}
}
