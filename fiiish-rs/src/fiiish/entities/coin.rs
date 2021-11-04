
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityData;
use crate::fiiish::entities::EntityState;
use crate::fiiish::entities::EntityType;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::EntityUpdateContext;
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Color,
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
	entity_data: EntityData,
}

impl Coin {
	pub fn new(pos: &Vector2, animation_offset: u16, crc: u32 ) -> Self {
		let mut s = Self {
			name: String::new(),
			crc: crc,
			pos: *pos, //Vector2::zero(),
			size: Vector2::new( 32.0, 32.0 ),
			animated_texture: AnimatedTexture::new(),
			animation_offset,
			alive: true,
			entity_data: EntityData::default(),
		};

		s.entity_data.dieing_time = 3.0;

		s
	}

	pub fn set_pos( &mut self, pos: &Vector2 ) {
		self.pos = *pos;
	}
	pub fn pos( &self ) -> &Vector2 {
		&self.pos
	}

	pub fn collect( &mut self ) {
		self.entity_data.state = EntityState::Dead;
	}
}

impl Entity for Coin {
	fn data(&self) -> &EntityData {
		&self.entity_data
	}
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
			// :TODO: decide if self destruction is a good idea
			self.kill();
		}
		self.entity_data.update( euc.time_step() );		
	}

	fn render( &mut self, renderer: &mut Renderer ){
		if self.alive {
			let deadness = self.entity_data.deadness();
			let opacity = 1.0 - deadness;

			renderer.use_layer( LayerId::Pickups as u8 );
			renderer.use_effect( EffectId::ColoredTextured as u16 );
			renderer.set_color( &Color::from_a( opacity as f32 ) );
			self.animated_texture.r#use( renderer );
			let o = Vector2::new( 0.0, -5000.0*( deadness*deadness ) as f32 );
			renderer.render_textured_quad( &self.pos.add( &o ), &self.size );
		}
	}

	fn kill( &mut self ) {
		self.entity_data.state = EntityState::Dieing{ time: 0.0 };
	}

	fn name( &self ) -> &str {
		&self.name
	}
	fn entity_type( &self ) -> EntityType {
		EntityType::Pickup
	}

}
