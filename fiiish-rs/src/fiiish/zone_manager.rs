
use rand::prelude::*;
use std::rc::Rc;

use crate::fiiish::entities::{
	Coin,
	Entity,
	EntityConfigurationManager,
	EntityManager,
	EntityType,
	Obstacle,
};
use crate::fiiish::EntityUpdateContext;
use crate::fiiish::layer_ids::LayerId;
use crate::fiiish::Zone;
use crate::math::Vector2;
use crate::system::System;

#[derive(Debug)]
pub struct ZoneManager {
	zones: Vec< Rc< Zone > >,
	pos: Vector2,
	active_zone: Option< Rc< Zone > >,
}

impl ZoneManager {
	pub fn new() -> Self {
		Self {
			zones: Vec::new(),
			pos: Vector2::zero(),
			active_zone: None,
		}
	}

	pub fn setup( &mut self ) {

	}

	pub fn teardown( &mut self ) {

	}

	pub fn is_zone_done( &self ) -> bool {
		if let Some( active_zone ) = &self.active_zone {
//			dbg!(&self.pos.x, active_zone.size().x);
			self.pos.x >= active_zone.size().x
		} else {
			true
		}
	}

	pub fn update( &mut self, euc: &mut EntityUpdateContext ) {
		self.pos.x -= euc.world_movement().x;
	}

	pub fn clear_zone( &mut self ) {
		self.active_zone = None;
	}
	pub fn next_zone( &mut self, ecm: &EntityConfigurationManager, em: &mut EntityManager, offset: &Vector2 ) {
		// select zone
		let n = self.zones.len();
		let r = rand::random::<f64>();
		let zi = ( n as f64 * r ).floor() as usize;
		dbg!(&zi);
		let mut next_zone: Option< Rc< Zone > > = None;
		for ( i, z ) in self.zones.iter().enumerate() {
			// :TODO: add actual logic to select zone
			if self.active_zone.is_none() {
				// always return the first one first
				next_zone = Some( Rc::clone( z ) );				
				break;
			} else if i == 0 && n > 0  {
				continue;
			}
			if i >= zi {
				next_zone = Some( Rc::clone( z ) );
				break;
			}
		}

		if let Some( next_zone ) = next_zone {
			println!("Found next zone: {}", next_zone.name() );
			self.active_zone = Some( next_zone );
			self.pos.x = 0.0;
			self.spawn_zone( ecm, em, offset );
		} else {
//			println!("Didn't find any zone");
		}
	}

	pub fn spawn_pickups( &mut self, ecm: &EntityConfigurationManager, em: &mut EntityManager ) {
		println!(":HACK: Use spawn_pickups for development only!");
		if let Some( zone ) = &self.active_zone {
			for l in zone.layer_iter() {
				for o in l.object_iter() {
					let ec = ecm.get_config( o.crc );
					dbg!(&ec);
					if ec.entity_type == EntityType::Pickup {
						let mut c = Coin::new( &o.pos, 0, o.crc );
						c.setup( &ec );

						em.add( Box::new( c ) );
					}
				}
			}		

		}
	}

	fn spawn_zone( &mut self, ecm: &EntityConfigurationManager, em: &mut EntityManager, offset: &Vector2 ) {
		if let Some( zone ) = &self.active_zone {
			for l in zone.layer_iter() {
				for o in l.object_iter() {
					let ec = ecm.get_config( o.crc );
//					dbg!(&ec);
					match ec.entity_type {
						EntityType::Pickup => {
							//println!("Coin {:?}", &o );
							let mut c = Coin::new( &o.pos.add( offset ), 0, o.crc );
							c.setup( &ec );

							em.add( Box::new( c ) );
						},
						EntityType::Obstacle => {
							//println!("Coin {:?}", &o );
							let mut r = Obstacle::new( &o.pos.add( offset ), o.crc );
	//						let mut r = Obstacle::new_from_config( &ec );
							r.setup( &ec );
	//						r.setup( "rock" );
							r.set_rotation( o.rotation );

							em.add( Box::new( r ) );
						},
						EntityType::Decoration => {
							let mut r = Obstacle::new( &o.pos.add( offset ), o.crc );
							r.setup( &ec );
							r.set_rotation( o.rotation );
							r.set_layer( LayerId::DecorationFront );

							em.add( Box::new( r ) );
						},
						_ => {
							println!("Unhandled entity {:?} with config {:?}", &o, &ec)
						},
					}
				}
			}		

		}
	}

	fn load_zone( &mut self, system: &mut System, name: &str ) {
		let mut zone = Zone::new();
		zone.load( system, name );

		self.zones.push( Rc::new( zone ) );
	}
	pub fn load_zones( &mut self, system: &mut System ) {
		// :TODO: use config file to load all zone
		self.load_zone( system, "0000_ILoveFiiishAndRust" );
		self.load_zone( system, "0001_ZigZag" );
		self.load_zone( system, "0002_ZigZag-B" );
		self.load_zone( system, "0003_ZigZag-C" );
		self.load_zone( system, "0004_ZigZag-D" );
		self.load_zone( system, "0005_ZigZag-E" );
		self.load_zone( system, "0006_ZigZag-F" );
		self.load_zone( system, "0010_Tunnel" );
		self.load_zone( system, "0020_Funnel" );
		self.load_zone( system, "1001_SeaweedAgain" );
		self.load_zone( system, "2001_SmoothBump" );
		self.load_zone( system, "2002_BigRockFlow" );
		self.load_zone( system, "2003_QuickDip" );
		self.load_zone( system, "2004_Curves" );
		self.load_zone( system, "2005_GoUp" );
	}
}
