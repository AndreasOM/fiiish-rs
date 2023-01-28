use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;

use crate::fiiish::entities::Entity;
use crate::fiiish::entities::EntityData;
use crate::fiiish::entities::EntityState;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::entities::EntityId;
use crate::fiiish::entities::EntityType;
//use crate::fiiish::entities::entity_ids::*;

use crate::fiiish::EntityUpdateContext;
use oml_game::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Color,
	Renderer
};

#[derive(Debug)]
pub struct Obstacle {
	name: String,
	crc: u32,
	pos: Vector2,
	size: Vector2,
	rotation: f32,
	animated_texture: AnimatedTexture,
	layer: LayerId,
	entity_id: EntityId,
	entity_data: EntityData,
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
			layer: LayerId::Obstacles,
			entity_id: EntityId::NONE,
			entity_data: EntityData::default(),
		}
	}

	pub fn set_rotation( &mut self, rotation: f32 ) {
		self.rotation = rotation;
	}

	pub fn set_layer( &mut self, layer: LayerId ) {
		self.layer = layer;
	}

	pub fn pos( &self ) -> &Vector2 {
		&self.pos
	}

	pub fn size( &self ) -> &Vector2 {
		&self.size
	}

	pub fn rotation( &self ) -> f32 {
		self.rotation
	}

	pub fn radius( &self ) -> f32 {
		self.size.length() * 0.5
	}

	pub fn entity_id( &self ) -> EntityId {
		self.entity_id
	}
}

impl Entity for Obstacle {
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
		self.entity_id = ec.entity_id;
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

		if let Some( debug_renderer ) = &*euc.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			let color = Color::from_rgba( 0.1, 0.5, 0.1, 0.8 );
			debug_renderer.add_line( &self.pos, &Vector2::zero(), 1.0, &color );
			debug_renderer.add_frame( &self.pos, &self.size, 5.0, &color );
//			let radius = self.size.length() * 0.5;
//			debug_renderer.add_circle( &self.pos, radius, 5.0, &color );
		}

	}

	fn render( &mut self, renderer: &mut Renderer ){
		if self.is_dead() {
			return;
		}
		let opacity = 1.0 - self.entity_data.deadness();

		renderer.use_layer( self.layer as u8 );
		renderer.use_effect( EffectId::ColoredTextured as u16 );
		renderer.set_color( &Color::from_a( opacity as f32 ) );
		self.animated_texture.r#use( renderer );
		renderer.render_textured_quad_with_rotation( &self.pos, &self.size, self.rotation );
	}

	fn kill( &mut self ) {
		self.entity_data.state = EntityState::Dieing{ time: 0.0 };
	}

	fn name( &self ) -> &str {
		&self.name
	}

	fn entity_type( &self ) -> EntityType {
		EntityType::Obstacle
	}

}
