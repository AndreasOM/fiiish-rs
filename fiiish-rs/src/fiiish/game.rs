
use crate::math::Vector2;
use crate::renderer::{
//	AnimatedTexture,
	Renderer,
	Texture,
};
use crate::system::System;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::entities::{
	Background,
	Coin,
	Entity,
	EntityManager,
	Obstacle,
	Player
};
use crate::fiiish::entities::{
//	EntityConfiguration,
	EntityConfigurationManager,
	EntityType,
};
use crate::fiiish::EntityUpdateContext;
use crate::fiiish::layer_ids::LayerId;
use crate::fiiish::Zone;

#[derive(Debug,Copy,Clone)]
pub enum GameState {
	None,
	WaitForStart,
	Playing,
	Dead,
}

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
	entity_manager: EntityManager,
	entity_configuration_manager: EntityConfigurationManager,
	zone: Zone,
	state: GameState,
}

impl Game {
	pub fn new() -> Self {
		Self {
			players: 						Vec::new(),
			entity_manager:	 	 	 	 	EntityManager::new(),
			entity_configuration_manager:	EntityConfigurationManager::new(),
			zone:							Zone::new(),
			state:							GameState::WaitForStart,
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		// load configuration
		// :TODO: actually load from a file
		self.entity_configuration_manager.load( system, "entity_config.whatever" );

		// load texture

		// not needed since they are in the global atlas already
//		AnimatedTexture::register_all( system, renderer, "fish_swim", 4 );
//		AnimatedTexture::register_all( system, renderer, "fish_die", 2 );

		renderer.register_texture( Texture::create( system, "background" ) );
		renderer.register_texture( Texture::create( system, "background_grad" ) );

		self.entity_manager.setup();

		// load zone(s)
		self.zone.load( system, "0000_ILoveFiiishAndRust" );

		// :HACK:
		for l in self.zone.layer_iter() {
			for o in l.object_iter() {
				let ec = self.entity_configuration_manager.get_config( o.crc );
//				dbg!(&ec);

				match ec.entity_type {
					EntityType::Pickup => {
						//println!("Coin {:?}", &o );
						let mut c = Coin::new( &o.pos, 0, o.crc );
						c.setup( &ec );

						self.entity_manager.add( Box::new( c ) );
					},
					EntityType::Obstacle => {
						//println!("Coin {:?}", &o );
						let mut r = Obstacle::new( &o.pos, o.crc );
//						let mut r = Obstacle::new_from_config( &ec );
						r.setup( &ec );
//						r.setup( "rock" );
						r.set_rotation( o.rotation );

						self.entity_manager.add( Box::new( r ) );
					},
					EntityType::Decoration => {
						let mut r = Obstacle::new( &o.pos, o.crc );
						r.setup( &ec );
						r.set_rotation( o.rotation );
						r.set_layer( LayerId::DecorationFront );

						self.entity_manager.add( Box::new( r ) );
					},
					_ => {
						println!("Unhandled entity {:?} with config {:?}", &o, &ec)
					},
				}
			}
		}

		let mut p = Player::new();
		p.setup( "player" );
		self.players.push( p );

		let b = Background::new();
//		b.setup( "backround" );
		self.entity_manager.add( Box::new( b ) );

	}

	pub fn teardown( &mut self ) {
		self.entity_manager.teardown();
		for p in self.players.iter_mut() {
			p.teardown( );
		}
	}

	fn spawn_pickups( &mut self ) {
		for l in self.zone.layer_iter() {
			for o in l.object_iter() {
				let ec = self.entity_configuration_manager.get_config( o.crc );
				dbg!(&ec);
				if ec.entity_type == EntityType::Pickup {
					let mut c = Coin::new( &o.pos, 0, o.crc );
					c.setup( &ec );

					self.entity_manager.add( Box::new( c ) );
				}
			}
		}		
	}

	fn collect_pickups( &mut self, euc: &EntityUpdateContext ) {
		for e in self.entity_manager.iter_mut() {
			if e.entity_type() == EntityType::Pickup {
				if e.is_alive() {
					let p: &mut Coin = match e.as_any_mut().downcast_mut::<Coin>() {
						Some(p) => p,
        				None => panic!("&e isn't a Coin!"),
    				};

//    				dbg!(&p);
					let pp = *p.pos();
					for f in self.players.iter() {
						if !f.is_alive() {
							// dead fish don't collect coins
							continue;
						}
						let magnet_range = 200.0;
						let fp = f.pos();

						let delta = pp.sub( &fp );
						let dist = delta.length();
//						dbg!(&dist);
						if dist < 10.0 { // fish over pickup
//							println!("Collected Pickup");
							p.kill();
						} else if dist < magnet_range {
							let magnet_speed = 300.0 * euc.time_step() as f32;
							let delta = delta.normalized();
							let delta = delta.scaled( -magnet_speed );
							let pp = pp.add( &delta );
							p.set_pos( &pp );
						}
					}
				}
			}
		}
//		todo!("die");
	}
	// :TODO: decide if we need the full WindowUpdateContext here
	pub fn update( &mut self, wuc: &mut WindowUpdateContext ) {

		let mut fish_movement = Vector2::zero();
		for p in self.players.iter_mut() {
			if p.name() == "player" {
				fish_movement = *p.movement();
				if p.is_alive() {
					self.state = GameState::Playing;
					if wuc.is_space_pressed {
						p.turn_down();
					} else {
						p.turn_up();
					};
					if wuc.was_key_pressed( 'k' as u8 ) {
						p.kill();
					}
				} else {
					// :TODO: handle via UI
					if p.can_respawn() {
						println!("Respawn");
						p.respawn();
						self.state = GameState::WaitForStart;
					} else {
						self.state = GameState::Dead;
					}
				}
			}
		}



		let mut euc = EntityUpdateContext::new()
						.set_time_step( wuc.time_step );

		euc.set_world_movement( &Vector2::new( -fish_movement.x, 0.0 ) );

		euc.set_game_state( &self.state );
		// :HACK: for testing background state transitions
		if wuc.was_key_pressed( 'b' as u8 ) {
			euc.enable_change_background_state();
		}

		if wuc.was_key_pressed( 'r' as u8 ) {
			self.spawn_pickups();
		}

		for p in self.players.iter_mut() {
			p.update( &mut euc );
		}

		for e in self.entity_manager.iter_mut() {
			e.update( &mut euc );
		}

		self.collect_pickups( &euc );
	}

	pub fn render( &mut self, renderer: &mut Renderer) {
		for p in self.players.iter_mut() {
			p.render( renderer );
		}
		for e in self.entity_manager.iter_mut() {
			e.render( renderer );
		}
	}
}
