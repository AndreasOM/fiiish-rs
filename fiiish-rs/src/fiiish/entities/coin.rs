
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityType;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::EntityUpdateContext;
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Renderer
};

#[derive(Debug)]
pub struct Coin {
	name: String,
	crc: u32,
	pos: Vector2,
	size: Vector2,
	animated_texture: AnimatedTexture,
	animation_offset: u16,
	alive: bool,
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
			alive: true,
		}
	}

	pub fn set_pos( &mut self, pos: &Vector2 ) {
		self.pos = *pos;
	}
	pub fn pos( &self ) -> &Vector2 {
		&self.pos
	}
}

impl Entity for Coin {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
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
		if self.alive {
			renderer.use_layer( LayerId::Pickups as u8 );
			renderer.use_effect( EffectId::Textured as u16 );
			self.animated_texture.r#use( renderer );
			renderer.render_textured_quad( &self.pos, &self.size );
		}
	}

	fn kill( &mut self ) {
		self.alive = false;
	}

	fn is_alive( &self ) -> bool {
		self.alive
	}

	fn name( &self ) -> &str {
		&self.name
	}
	fn entity_type( &self ) -> EntityType {
		EntityType::Pickup
	}

}
