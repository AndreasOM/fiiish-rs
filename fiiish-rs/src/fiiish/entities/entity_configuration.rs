
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
	pub entity_type: EntityType,
	pub size: Vector2,
	pub animated_texture_configuration: AnimatedTextureConfiguration,
}

impl EntityConfiguration {
	pub fn new(size: Vector2, animated_texture_configuration: AnimatedTextureConfiguration) -> Self {
		Self {
			entity_type: EntityType::None,			
			size,
			animated_texture_configuration,
		}
	}
}

impl From< ( EntityType, Vector2, AnimatedTextureConfiguration ) > for EntityConfiguration
{
	fn from( t: ( EntityType, Vector2, AnimatedTextureConfiguration ) ) -> Self {
		Self{ 
			entity_type: t.0,
			size: t.1,
			animated_texture_configuration: t.2,
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

	pub fn load( &mut self, system: &mut System, name: &str ) -> bool {
		/*
		self.configs.insert(
			EntityId::ROCKA as u32,
			EntityConfiguration::new(
				Vector2::new( 150.0,  89.0 ),
				AnimatedTextureConfiguration::new( "rock-a", 0, 0, 0, 25.0 )
			)
		);
		*/
		self.configs.insert( EntityId::BLOCK1X1 as u32, ( EntityType::Decoration, ( 128.0, 128.0 ).into(), ( "block-1x1", 0, 0, 0, 25.0 ).into() ).into() );

		// pickups
		self.configs.insert( EntityId::PICKUPCOIN as u32, ( EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "coin_", 2, 1, 32, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::PICKUPRAIN as u32, ( EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "coin_blue_", 2, 1, 32, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::PICKUPEXPLOSION as u32, ( EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "coin_green_", 2, 1, 32, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::PICKUPMAGNET as u32, ( EntityType::Pickup, ( 32.0,  32.0 ).into(), ( "magnet_", 2, 1, 32, 25.0 ).into() ).into() );

		// obstacles
		self.configs.insert( EntityId::ROCKA as u32, ( EntityType::Obstacle, ( 150.0,  89.0 ).into(), ( "rock-a", 0, 0, 0, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::ROCKB as u32, ( EntityType::Obstacle, ( 204.0, 220.0 ).into(), ( "rock-b", 0, 0, 0, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::ROCKC as u32, ( EntityType::Obstacle, ( 166.0, 373.0 ).into(), ( "rock-c", 0, 0, 0, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::ROCKD as u32, ( EntityType::Obstacle, ( 197.0, 436.0 ).into(), ( "rock-d", 0, 0, 0, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::ROCKE as u32, ( EntityType::Obstacle, ( 175.0, 556.0 ).into(), ( "rock-e", 0, 0, 0, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::ROCKF as u32, ( EntityType::Obstacle, ( 413.0, 538.0 ).into(), ( "rock-f", 0, 0, 0, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDA as u32, ( EntityType::Obstacle, ( 63.0, 114.0 ).into(), ( "seaweed-a-", -2, 1, 48, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDB as u32, ( EntityType::Obstacle, ( 62.0, 181.0 ).into(), ( "seaweed-b-", -2, 1, 48, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDC as u32, ( EntityType::Obstacle, ( 72.0, 238.0 ).into(), ( "seaweed-c-", -2, 1, 48, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDD as u32, ( EntityType::Obstacle, ( 78.0, 296.0 ).into(), ( "seaweed-d-", -2, 1, 48, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDE as u32, ( EntityType::Obstacle, ( 98.0, 330.0 ).into(), ( "seaweed-e-", -2, 1, 48, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDF as u32, ( EntityType::Obstacle, ( 113.0, 375.0 ).into(), ( "seaweed-f-", -2, 1, 48, 25.0 ).into() ).into() );
		self.configs.insert( EntityId::SEAWEEDG as u32, ( EntityType::Obstacle, ( 115.0, 404.0 ).into(), ( "seaweed-g-", -2, 1, 48, 25.0 ).into() ).into() );
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
