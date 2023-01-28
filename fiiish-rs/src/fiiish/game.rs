use oml_audio::AudioBackend;
use std::rc::Rc;
use std::cell::RefCell;

use oml_audio::{
	Audio,
};

pub use oml_audio::fileloader::{FileLoader, FileLoaderFile};
//use crate::audio::audio_fileloader_system::*;

use oml_game::math::Vector2;
use oml_game::renderer::{
//	AnimatedTexture,
	Color,
	Renderer,
	Texture,
};
use oml_game::system::System;
use oml_game::window::window_update_context::WindowUpdateContext;

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
use crate::fiiish::Player;
use crate::fiiish::ShapeCache;
use crate::fiiish::ZoneManager;

use oml_game::renderer::debug_renderer::DebugRenderer;

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
struct TimedTransfer {
	total_time: f32,
	duration: f32,
	delay: f32,
	per_second: f32,
	amount: f32,
}

impl TimedTransfer {
	pub fn new( duration: f32, delay: f32 ) -> Self {
		Self {
			total_time: 0.0,
			duration,
			delay,
			per_second: 0.0,
			amount: 0.0,
		}
	}

	pub fn reset( &mut self, total_amount: u32 ) {
		self.per_second = total_amount as f32 / self.duration;
		self.amount = 0.0;
		self.total_time = 0.0;
	}

	pub fn tick( &mut self, time_step: f32 ) -> u32 {
		self.total_time += time_step;
		if self.total_time > self.delay {
			let to_transfer = self.amount + self.per_second * time_step;
			let full_to_transfer = to_transfer.floor() as u32;
			let leftover_to_transfer = to_transfer - full_to_transfer as f32;
			self.amount = leftover_to_transfer;
			full_to_transfer
		} else {
			0
		}
	}
}
#[derive(Debug)]
pub struct Game {
	fishes: Vec<Fish>,
	entity_manager: EntityManager,
	entity_configuration_manager: EntityConfigurationManager,
	zone_manager: ZoneManager,
	shape_cache: ShapeCache,
	state: GameState,
	time_in_state: f32,
	is_paused: bool,
	is_music_enabled: bool,
	is_sound_enabled: bool,

	debug_renderer: Rc < Option < RefCell< DebugRenderer >  > >,

	coins: u32,
	distance: f32,
	pixels_per_meter: f32,

	player: Player,			// since the game always exists (for Fiiish!) we can :HACK: it this way
	coin_transfer:		TimedTransfer,
	distance_transfer:	TimedTransfer,


	audio:				Box<dyn AudioBackend<oml_game::system::System>>,
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
			time_in_state:					0.0,
			is_paused:						false,
			is_music_enabled:				true,
			is_sound_enabled:				true,
			debug_renderer:					Rc::new( None ),
			coins:							0,
			distance:						0.0,
			pixels_per_meter:				100.0,
			player:							Player::new(),
			coin_transfer:					TimedTransfer::new( 1.0, 2.0 ),
			distance_transfer:				TimedTransfer::new( 2.0, 3.0 ),
			audio:							Audio::create_default(),
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

		tracing::info!("Audio Backend: {}", self.audio.backend_type() );

		if !self.audio.load_music_native( system, "theme-00" ) {
			println!("Error loading music");
		}

		// load sounds
//		self.sound.load( system, "picked_coin", 10 );
//		self.sound.load( system, "fiish_death", 1 );

		self.audio.load_sound_bank( system, "default.omsb" );

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

		if !self.player.load( system ) {
//			todo!("Handle broken savegame");
		}

		self.is_music_enabled = self.player.music_enabled();
		self.is_sound_enabled = self.player.sound_enabled();
		if self.is_music_enabled {
			self.audio.play_music();
		}
//		self.player.save( system ); // :HACK:

		dbg!(&self.player);

		self.audio.start();
//		todo!("die");
	}

	pub fn autosave( &mut self, system: &mut System ) -> bool {
		if self.player.is_dirty() {
			self.player.save( system )
		} else {
			false
		}
	}


	pub fn teardown( &mut self, system: &mut System ) {
		self.autosave( system );
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
									/*
									let a = OverlapCheckerItem {
										shape: &fish_shape,
										pos: &fp,
										offset: &Vector2::new( -64.0, -64.0 ),
										rotation: 0.0,
									};
									*/
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
										self.audio.play_sound( "FIISH_DEATH" );
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
//							p.kill();
							p.collect();
							// play pickup sound
//							dbg!(&self.sound_bank);
							self.audio.play_sound( "PICKED_COIN" );
							self.coins += 1;
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
		self.audio.update();
		self.time_in_state += auc.time_step() as f32;
		let mut fish_movement = Vector2::zero();
		for p in self.fishes.iter_mut() {
			if p.name() == "fish" {
				fish_movement = *p.movement();
				match self.state {
					GameState::None => {
						println!("Respawn");
						p.respawn();
						self.state = GameState::WaitForStart;
						self.time_in_state = 0.0;
						if self.coins > 0 {
//							let c = self.take_coins( self.coins );
							self.player.give_coins( self.coins );
							self.coins = 0;
						}
						if self.distance > 0.0 {
							self.player.add_to_last_distance( self.distance.floor() as u32 );
							self.distance = 0.0;
						}
//						self.coins = 0;
//						self.distance = 0.0;
					},
					GameState::WaitForStart => {
						if wuc.is_space_pressed {
							self.state = GameState::Playing;
							self.time_in_state = 0.0;
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
							self.time_in_state = 0.0;
						}
					},
					GameState::Die => {
						for e in self.entity_manager.iter_mut() {
							// :TODO: fade out or something
							e.kill();
						}
						self.zone_manager.clear_zone();
						self.state = GameState::Dead;
						self.time_in_state = 0.0;
						self.coin_transfer.reset( self.coins );
						self.distance_transfer.reset( self.distance.floor() as u32 );
						self.player.reset_last_distance();
						self.player.log_play( self.coins, self.distance.floor() as u32 );
					},
					GameState::Dead => {
						if p.can_respawn() {
//							if wuc.was_key_pressed( 'r' as u8 ) {
							if wuc.is_space_pressed {
								self.state = GameState::None;
								self.time_in_state = 0.0;
							}
						}
					},
				}
			}
		}

		match self.state {
			GameState::Dead => {
				let c = self.coin_transfer.tick( auc.time_step() as f32 );
				let c = self.take_coins( c );
				self.player.give_coins( c );
				let d = self.distance_transfer.tick( auc.time_step() as f32 );
				let d = self.take_distance( d );
				self.player.add_to_last_distance( d );
			},
			_ => {},
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
				self.distance -= euc.world_movement().x / self.pixels_per_meter;
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

	pub fn player( &self ) -> &Player {
		&self.player
	}

	pub fn coins( &self ) -> u32 {
		self.coins
	}

	fn take_coins( &mut self, count: u32 ) -> u32 {
		let c = if count > self.coins {
			self.coins
		} else {
			count
		};

		self.coins -= c;

		c
	}

	pub fn distance( &self ) -> u32 {
		self.distance.floor() as u32
	}

	fn take_distance( &mut self, amount: u32 ) -> u32 {
		let fd = self.distance.floor() as u32;
		let d = if amount > fd {
			fd
		} else {
			amount
		};

		self.distance -= d as f32;

		d
	}


	pub fn is_waiting_for_start( &self ) -> bool {
		self.state == GameState::WaitForStart
	}
	pub fn is_dead( &self ) -> bool {
		self.state == GameState::Dead
	}
	pub fn is_playing( &self ) -> bool {
		self.state == GameState::Playing
	}
	pub fn is_paused( &self ) -> bool {
		self.is_paused
	}
	pub fn is_music_enabled( &self ) -> bool {
		self.is_music_enabled
	}
	pub fn is_sound_enabled( &self ) -> bool {
		self.is_sound_enabled
	}

	pub fn toggle_pause( &mut self ) {
		self.is_paused = !self.is_paused;
	}
	pub fn toggle_music( &mut self ) {
		self.is_music_enabled = !self.is_music_enabled;
		self.player.set_music_enabled( self.is_music_enabled );
		if self.is_music_enabled {
			self.audio.play_music();
		} else {
			self.audio.pause_music();
		}
	}
	pub fn toggle_sound( &mut self ) {
		self.is_sound_enabled = !self.is_sound_enabled;
		self.player.set_sound_enabled( self.is_sound_enabled );
		if self.is_sound_enabled {
		} else {
		}
	}

	pub fn can_respawn( &self ) -> bool {
		if self.time_in_state > 3.0 {
			for p in self.fishes.iter() {
				if p.name() == "fish" {
					if p.can_respawn() {
						return true;
					}
				}				
			}
		}
		false
	}

	pub fn play( &mut self ) {
		if self.state == GameState::Dead {
			if self.can_respawn() {
				self.state =GameState::None;
			}
		}
	}
}
