
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
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
	EntityConfiguration,
	EntityConfigurationManager,
};
use crate::fiiish::EntityUpdateContext;
use crate::fiiish::Zone;

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
	entity_manager: EntityManager,
	entity_configuration_manager: EntityConfigurationManager,
	zone: Zone,
}

impl Game {
	pub fn new() -> Self {
		Self {
			players: 						Vec::new(),
			entity_manager:	 	 	 	 	EntityManager::new(),
			entity_configuration_manager:	EntityConfigurationManager::new(),
			zone:							Zone::new(),
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		// load configuration
		// :TODO: actually load from a file
		self.entity_configuration_manager.load( system, "entity_config.whatever" );

		// load texture
		AnimatedTexture::register_all( system, renderer, "fish_swim", 4 );
		AnimatedTexture::register_all( system, renderer, "fish_die", 2 );
		renderer.register_texture( Texture::create( system, "background" ) );
		renderer.register_texture( Texture::create( system, "background_grad" ) );

		self.entity_manager.setup();

		// load zone(s)
		self.zone.load( system, "0000_ILoveFiiish" );

		// :HACK:
		for l in self.zone.layer_iter() {
			for o in l.object_iter() {
				let ec = self.entity_configuration_manager.get_config( o.crc );
				dbg!(&ec);

				match o.crc {
					0xe4c651aa
					| 0x06fd4c5a
					| 0xf75fd92f
					| 0x235a41dd
					=> {
						//println!("Coin {:?}", &o );
						let mut c = Coin::new( &o.pos, 0, o.crc );
						c.setup( &ec );

						self.entity_manager.add( Box::new( c ) );
					},
					0xd058353c
					| 0x49516486
					| 0x3e565410
					| 0xa032c1b3
					| 0xd735f125
					| 0x4e3ca09f
					| 0x6fe93bef
					| 0xf6e06a55
					| 0x81e75ac3
					| 0x1f83cf60
					| 0x6884fff6
					| 0xf18dae4c
					| 0x868a9eda
					=> {
						//println!("Coin {:?}", &o );
						let mut r = Obstacle::new( &o.pos, o.crc );
//						let mut r = Obstacle::new_from_config( &ec );
						r.setup( &ec );
//						r.setup( "rock" );
						r.set_rotation( o.rotation );

						self.entity_manager.add( Box::new( r ) );
					},
					_ => {},
				}
			}
		}

		let mut p = Player::new();
		p.setup( "player" );
		self.players.push( p );

		let mut b = Background::new();
//		b.setup( "backround" );
		self.entity_manager.add( Box::new( b ) );

	}

	pub fn teardown( &mut self ) {
		self.entity_manager.teardown();
		for p in self.players.iter_mut() {
			p.teardown( );
		}
	}

	// :TODO: decide if we need the full WindowUpdateContext here
	pub fn update( &mut self, wuc: &mut WindowUpdateContext ) {

		let mut fish_movement = Vector2::zero();
		for p in self.players.iter_mut() {
			if p.name() == "player" {
				fish_movement = *p.movement();
				if p.is_alive() {
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
					}
				}
			}
		}



		let mut euc = EntityUpdateContext::new()
						.set_time_step( wuc.time_step );

		euc.set_world_movement( &Vector2::new( -fish_movement.x, 0.0 ) );
		// :HACK: for testing background state transitions
		if wuc.was_key_pressed( 'b' as u8 ) {
			euc.enable_change_background_state();
		}

		for p in self.players.iter_mut() {
			p.update( &mut euc );
		}

		for e in self.entity_manager.iter_mut() {
			e.update( &mut euc );
		}
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
