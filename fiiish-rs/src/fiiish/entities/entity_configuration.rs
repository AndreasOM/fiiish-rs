
use std::collections::HashMap;

use crate::math::Vector2;

use crate::system::System;

use crate::fiiish::entities::EntityType;
use crate::fiiish::entities::entity_ids::*;


#[derive(Debug)]
pub struct AnimatedTextureConfiguration {
	pub prefix: String,
	pub number_of_digits: i8,
	pub first_frame: u16,
	pub last_frame: u16,
	pub fps: f32,
}

impl AnimatedTextureConfiguration {
	pub fn new( prefix: &str, number_of_digits: i8, first_frame: u16, last_frame: u16, fps: f32 ) -> Self {
		Self {
			prefix: prefix.to_owned(),
			number_of_digits,
			first_frame,
			last_frame,
			fps,
		}
	}
}

impl From< ( &str, i8, u16, u16, f32 ) > for AnimatedTextureConfiguration {
	fn from( t: ( &str, i8, u16, u16, f32 ) ) -> Self {
		Self {
			prefix: t.0.to_owned(),
			number_of_digits: t.1,
			first_frame: t.2,
			last_frame: t.3,
			fps: t.4,
		}
	}
}

#[derive(Debug)]
pub struct EntityConfiguration {
	pub entity_id: EntityId,
	pub entity_type: EntityType,
	pub size: Vector2,
	pub animated_texture_configuration: AnimatedTextureConfiguration,
}

impl EntityConfiguration {
	pub fn new(size: Vector2, animated_texture_configuration: AnimatedTextureConfiguration) -> Self {
		Self {
			entity_id: EntityId::NONE,
			entity_type: EntityType::None,			
			size,
			animated_texture_configuration,
		}
	}
}

impl From< ( EntityId, EntityType, Vector2, AnimatedTextureConfiguration ) > for EntityConfiguration
{
	fn from( t: ( EntityId, EntityType, Vector2, AnimatedTextureConfiguration ) ) -> Self {
		Self{ 
			entity_id:							t.0,
			entity_type:						t.1,
			size:								t.2,
			animated_texture_configuration:		t.3,
		}
	}
}

#[derive(Debug)]
pub struct EntityConfigurationManager {
	configs: HashMap<u32, EntityConfiguration>,
}

impl EntityConfigurationManager {
	pub fn new() -> Self {
		Self {
			configs: HashMap::new(),
		}
	}

	fn add_config( &mut self, ec: EntityConfiguration ) {
		self.configs.insert( ec.entity_id as u32, ec );
	}

	pub fn load( &mut self, _system: &mut System, _name: &str ) -> bool {
		/*
		self.configs.insert(
			EntityId::ROCKA as u32,
			EntityConfiguration::new(
				Vector2::new( 150.0,  89.0 ),
				AnimatedTextureConfiguration::new( "rock-a", 0, 0, 0, 25.0 )
			)
		);
		*/
//		self.configs.insert( EntityId::BLOCK1X1 as u32, ( EntityType::Decoration, ( 128.0, 128.0 ).into(), ( "block-1x1", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::BLOCK1X1, EntityType::Decoration, ( 128.0, 128.0 ).into(), ( "block-1x1", 0, 0, 0, 25.0 ).into() ).into() );

		// pickups
		self.add_config( ( EntityId::PICKUPCOIN, EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "coin_", 2, 1, 32, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::PICKUPRAIN, EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "coin_blue_", 2, 1, 32, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::PICKUPEXPLOSION, EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "coin_green_", 2, 1, 32, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::PICKUPMAGNET, EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "magnet_", 2, 1, 32, 25.0 ).into() ).into() );

		// obstacles
		self.add_config( ( EntityId::ROCKA, EntityType::Obstacle, ( 150.0,  89.0 ).into(), ( "rock-a", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::ROCKB, EntityType::Obstacle, ( 204.0, 220.0 ).into(), ( "rock-b", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::ROCKC, EntityType::Obstacle, ( 166.0, 373.0 ).into(), ( "rock-c", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::ROCKD, EntityType::Obstacle, ( 197.0, 436.0 ).into(), ( "rock-d", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::ROCKE, EntityType::Obstacle, ( 175.0, 556.0 ).into(), ( "rock-e", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::ROCKF, EntityType::Obstacle, ( 413.0, 538.0 ).into(), ( "rock-f", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDA, EntityType::Obstacle, ( 63.0, 114.0 ).into(), ( "seaweed-a-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDB, EntityType::Obstacle, ( 62.0, 181.0 ).into(), ( "seaweed-b-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDC, EntityType::Obstacle, ( 72.0, 238.0 ).into(), ( "seaweed-c-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDD, EntityType::Obstacle, ( 78.0, 296.0 ).into(), ( "seaweed-d-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDE, EntityType::Obstacle, ( 98.0, 330.0 ).into(), ( "seaweed-e-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDF, EntityType::Obstacle, ( 113.0, 375.0 ).into(), ( "seaweed-f-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::SEAWEEDG, EntityType::Obstacle, ( 115.0, 404.0 ).into(), ( "seaweed-g-", -2, 1, 48, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::FERRIS, EntityType::Decoration, ( 55.0, 41.0 ).into(), ( "ferris", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::HEART, EntityType::Decoration, ( 64.0, 64.0 ).into(), ( "heart", 0, 0, 0, 25.0 ).into() ).into() );
		self.add_config( ( EntityId::FIIISH, EntityType::Decoration, ( 64.0, 64.0 ).into(), ( "fiiish", 0, 0, 0, 25.0 ).into() ).into() );
		true
	}

	pub fn get_config( &self, crc: u32 ) -> &EntityConfiguration {

		match self.configs.get( &crc ) {
			Some( ec ) => ec,
			None => {
				// return any
				if self.configs.len() == 0 {
					panic!("Tried to get entity configuration without loading!");
				};
				println!( "Warning: No configuration found for entity {:X}", crc );

				self.configs.values().next().unwrap()
			} 
		}
	}
}
