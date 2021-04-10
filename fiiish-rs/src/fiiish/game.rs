use std::rc::Rc;
use std::cell::RefCell;

use crate::math::Vector2;
use crate::renderer::{
//	AnimatedTexture,
	Color,
	Renderer,
	Texture,
};
use crate::system::System;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::entities::{
	Background,
	Coin,
	Entity,
//	EntityConfiguration,
	EntityConfigurationManager,
	EntityId,
	EntityManager,
	EntityType,
	Fish,
	Obstacle,
};
use crate::fiiish::EntityUpdateContext;
//use crate::fiiish::layer_ids::LayerId;
use crate::fiiish::ZoneManager;

use crate::DebugRenderer;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum GameState {
	None,
	WaitForStart,
	Playing,
	Dead,
}

#[derive(Debug)]
pub struct Game {
	fishes: Vec<Fish>,
	entity_manager: EntityManager,
	entity_configuration_manager: EntityConfigurationManager,
	zone_manager: ZoneManager,
	state: GameState,

	debug_renderer: Rc < Option < RefCell< DebugRenderer >  > >,
}

impl Game {
	pub fn new() -> Self {
		Self {
			fishes: 						Vec::new(),
			entity_manager:	 	 	 	 	EntityManager::new(),
			entity_configuration_manager:	EntityConfigurationManager::new(),
			zone_manager:					ZoneManager::new(),
			state:							GameState::WaitForStart,
			debug_renderer:					Rc::new( None ),
		}
	}

	pub fn enable_debug_renderer( &mut self, debug_renderer: &Rc< Option< RefCell< DebugRenderer > > > ) {
		self.debug_renderer = Rc::clone( debug_renderer );
	}

	pub fn disable_debug_renderer( &mut self ) {
		self.debug_renderer = Rc::new( None );
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

		self.zone_manager.setup();

		self.zone_manager.load_zones( system );

		let mut p = Fish::new();
		let ec = self.entity_configuration_manager.get_config( EntityId::FIIISH as u32 );
		p.setup( &ec );
		self.fishes.push( p );

		let b = Background::new();
//		b.setup( "backround" );
		self.entity_manager.add( Box::new( b ) );

	}

	pub fn teardown( &mut self ) {
		self.entity_manager.teardown();
		for p in self.fishes.iter_mut() {
			p.teardown( );
		}
	}

	fn collect_pickups( &mut self, euc: &EntityUpdateContext ) {
		/*
*/
		for f in self.fishes.iter() {
			if !f.is_alive() {
				// dead fish don't collect coins
				continue;
			}
			let pickup_range = 10.0;
			let magnet_range = 200.0;
			let fp = f.pos();

			if let Some( debug_renderer ) = &*euc.debug_renderer {
				let mut debug_renderer = debug_renderer.borrow_mut();
				let color = Color::from_rgba( 0.9, 0.9, 0.1, 0.8 );
				debug_renderer.add_circle( &fp, magnet_range, 5.0, &color );
				debug_renderer.add_circle( &fp, pickup_range, 2.0, &color );
			}


			for e in self.entity_manager.iter_mut() {
				if e.entity_type() == EntityType::Pickup {
					if e.is_alive() {
						let p: &mut Coin = match e.as_any_mut().downcast_mut::<Coin>() {
							Some(p) => p,
	        				None => panic!("&e isn't a Coin!"),
	    				};

	//    				dbg!(&p);
						let pp = *p.pos();

						let delta = pp.sub( &fp );
						let dist = delta.length();
//						dbg!(&dist);
						if dist < pickup_range { // fish over pickup
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
		for p in self.fishes.iter_mut() {
			if p.name() == "fish" {
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
						for e in self.entity_manager.iter_mut() {
							// :TODO: fade out or something
							e.kill();
						}
						self.zone_manager.clear_zone();
					}
				}
			}
		}



		let mut euc = EntityUpdateContext::new()
						.set_time_step( wuc.time_step );

		euc.set_world_movement( &Vector2::new( -fish_movement.x, 0.0 ) );

		euc.set_game_state( &self.state );
		euc.set_debug_renderer( &self.debug_renderer );

		// :HACK: for testing background state transitions
		if wuc.was_key_pressed( 'b' as u8 ) {
			euc.enable_change_background_state();
		}

		if wuc.was_key_pressed( 'r' as u8 ) {
//			self.spawn_pickups();
			self.zone_manager.spawn_pickups( &self.entity_configuration_manager, &mut self.entity_manager );

		}

		for p in self.fishes.iter_mut() {
			p.update( &mut euc );
		}

		for e in self.entity_manager.iter_mut() {
			e.update( &mut euc );
		}

		self.entity_manager.remove_dead();

		if self.state == GameState::Playing {
			self.zone_manager.update( &mut euc );
			if self.zone_manager.is_zone_done() {
	//			println!("Reached end of zone, spawning new zone");
				self.zone_manager.next_zone( &self.entity_configuration_manager, &mut self.entity_manager, &Vector2::new(1500.0,0.0) );
			}
			self.collect_pickups( &euc );
		}
	}

	pub fn render( &mut self, renderer: &mut Renderer) {
		for p in self.fishes.iter_mut() {
			p.render( renderer );
		}
		for e in self.entity_manager.iter_mut() {
			e.render( renderer );
		}
	}
}
