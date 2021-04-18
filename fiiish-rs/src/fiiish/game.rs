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
use crate::fiiish::app_update_context::AppUpdateContext;

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
//use crate::fiiish::{ Shape, SubShape };
use crate::fiiish::ShapeCache;
use crate::fiiish::ZoneManager;

use crate::DebugRenderer;
use crate::{
	OverlapChecker,
	OverlapCheckerItem,
};

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum GameState {
	None,
	WaitForStart,
	Playing,
	Die,
	Dead,
}

#[derive(Debug)]
pub struct Game {
	fishes: Vec<Fish>,
	entity_manager: EntityManager,
	entity_configuration_manager: EntityConfigurationManager,
	zone_manager: ZoneManager,
	shape_cache: ShapeCache,
	state: GameState,
	is_paused: bool,

	debug_renderer: Rc < Option < RefCell< DebugRenderer >  > >,
}

impl Game {
	pub fn new() -> Self {
		Self {
			fishes: 						Vec::new(),
			entity_manager:	 	 	 	 	EntityManager::new(),
			entity_configuration_manager:	EntityConfigurationManager::new(),
			zone_manager:					ZoneManager::new(),
			shape_cache:					ShapeCache::new(),
			state:							GameState::None,
			is_paused:						false,
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

		self.shape_cache.load_shape( system, "fish_swim", EntityId::FISHSWIM );
		self.shape_cache.load_shape( system, "block-1x1", EntityId::BLOCK1X1 );

		self.shape_cache.load_shape( system, "rock-a", EntityId::ROCKA );
		self.shape_cache.load_shape( system, "rock-b", EntityId::ROCKB );
		self.shape_cache.load_shape( system, "rock-c", EntityId::ROCKC );
		self.shape_cache.load_shape( system, "rock-d", EntityId::ROCKD );
		self.shape_cache.load_shape( system, "rock-e", EntityId::ROCKE );
		self.shape_cache.load_shape( system, "rock-f", EntityId::ROCKF );

		self.shape_cache.load_shape( system, "seaweed-a", EntityId::SEAWEEDA );
		self.shape_cache.load_shape( system, "seaweed-b", EntityId::SEAWEEDB );
		self.shape_cache.load_shape( system, "seaweed-c", EntityId::SEAWEEDC );
		self.shape_cache.load_shape( system, "seaweed-d", EntityId::SEAWEEDD );
		self.shape_cache.load_shape( system, "seaweed-e", EntityId::SEAWEEDE );
		self.shape_cache.load_shape( system, "seaweed-f", EntityId::SEAWEEDF );
		self.shape_cache.load_shape( system, "seaweed-g", EntityId::SEAWEEDG );

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

		/* test rock for collision debugging
		let mut test_rock = Obstacle::new( &Vector2::zero(), EntityId::ROCKF as u32 );
		let test_rock_config = self.entity_configuration_manager.get_config( EntityId::ROCKF as u32 );
		test_rock.setup( &test_rock_config );
		test_rock.set_rotation( 30.0 );
		self.entity_manager.add( Box::new( test_rock ) );
		*/
	}

	pub fn teardown( &mut self ) {
		self.entity_manager.teardown();
		for p in self.fishes.iter_mut() {
			p.teardown( );
		}
	}


	fn collide_with_obstacles( &mut self, euc: &EntityUpdateContext ) {
		let fc = Color::from_rgba( 0.8, 0.8, 0.5, 0.8 );
		let oc = Color::from_rgba( 0.2, 0.5, 0.2, 0.4 );
		let foc = Color::from_rgba( 0.5, 0.2, 0.8, 0.8 );

		for f in self.fishes.iter_mut() {
			if !f.is_alive() {
				// dead fish don't collect coins
				continue;
			}
			let fp = f.pos().clone();
			let fr = f.radius();
			if let Some( debug_renderer ) = &*euc.debug_renderer {
				let mut debug_renderer = debug_renderer.borrow_mut();
				debug_renderer.add_circle( &fp, fr, 5.0, &fc );
			}

			for e in self.entity_manager.iter_mut() {
				if e.entity_type() == EntityType::Obstacle {
					if e.is_alive() {
						let o: &mut Obstacle = match e.as_any_mut().downcast_mut::<Obstacle>() {
							Some(o) => o,
	        				None => panic!("{:?} isn't a Obstacle!", &e ),
	    				};
						let op = o.pos();
						let or = o.radius();

						let fo = op.sub( &fp );
						let fod = fo.length();
						let fo_collide = fod < fr + or;

						if let Some( debug_renderer ) = &*euc.debug_renderer {
							let mut debug_renderer = debug_renderer.borrow_mut();
							if !fo_collide {
								debug_renderer.add_circle( &op, or, 5.0, &oc );
							} else {
								debug_renderer.add_circle( &op, or, 5.0, &foc );
								debug_renderer.add_line( &fp, &fp.add( &fo ), 3.0, &foc );
							}
						}

						if fo_collide {
							if let Some( fish_shape ) = self.shape_cache.find( EntityId::FISHSWIM ) {
								fish_shape.debug_render( &*self.debug_renderer, &Vector2::new( -64.0, -64.0 ), &fp, 0.0 );
								if let Some( obstacle_shape ) = self.shape_cache.find( o.entity_id() ) {
									let offset = o.size().scaled( -0.5 );
									let rot = o.rotation();
									obstacle_shape.debug_render( &*self.debug_renderer, &offset, &op, rot );
									let a = OverlapCheckerItem {
										shape: &fish_shape,
										pos: &fp,
										offset: &Vector2::new( -64.0, -64.0 ),
										rotation: 0.0,
									};
									let a = OverlapCheckerItem {
										shape: &fish_shape,
										pos: &fp,
										offset: &Vector2::new( -64.0, -64.0 ),
										rotation: f.rotation(),
									};
									let b = OverlapCheckerItem {
										shape: &obstacle_shape,
										pos: &op,
										offset: &offset,
										rotation: rot,
									};

									if OverlapChecker::do_shapes_overlap( &a, &b, &*self.debug_renderer ) {
										f.kill();
									}
								}
							};
						}
	    			}
	    		}
	    	}
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
	pub fn update( &mut self, wuc: &mut WindowUpdateContext, auc: &mut AppUpdateContext ) {

		let mut fish_movement = Vector2::zero();
		for p in self.fishes.iter_mut() {
			if p.name() == "fish" {
				fish_movement = *p.movement();
				match self.state {
					GameState::None => {
						println!("Respawn");
						p.respawn();
						self.state = GameState::WaitForStart;
					},
					GameState::WaitForStart => {
						if wuc.is_space_pressed {
							self.state = GameState::Playing;
						}
					},
					GameState::Playing => {
						if wuc.is_space_pressed {
							p.turn_down();
						} else {
							p.turn_up();
						};
						if wuc.was_key_pressed( 'k' as u8 ) {
							p.kill();
						}
						if !p.is_alive() {
							self.state = GameState::Die;
						}
					},
					GameState::Die => {
						for e in self.entity_manager.iter_mut() {
							// :TODO: fade out or something
							e.kill();
						}
						self.zone_manager.clear_zone();
						self.state = GameState::Dead;
					},
					GameState::Dead => {
						if p.can_respawn() {
							self.state = GameState::None;
						}
					},
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

		if wuc.was_key_pressed( 'p' as u8 ) {
			self.toggle_pause();
		}

		if !self.is_paused {
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
				self.collide_with_obstacles( &euc );
				self.collect_pickups( &euc );
			}
		} else {
			// :HACK: for visualising collisions even when paused
			/* :TODO: enable via hotkey
			self.collide_with_obstacles( &euc );
			if wuc.mouse_buttons[ 0 ] {
				let pos = auc.cursor_pos();
				for p in self.fishes.iter_mut() {
					if p.name() == "fish" {
						p.set_pos( &pos );
					}
				}
			}
			*/
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

	pub fn is_playing( &self ) -> bool {
		self.state == GameState::Playing
	}
	pub fn is_paused( &self ) -> bool {
		self.is_paused
	}

	pub fn toggle_pause( &mut self ) {
		self.is_paused = !self.is_paused;
	}
}
