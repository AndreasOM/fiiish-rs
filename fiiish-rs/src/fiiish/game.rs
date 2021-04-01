
use crate::math::Vector2;
use crate::renderer::{
	AnimatedTexture,
	Renderer,
	Texture,
};
use crate::system::System;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::entities::{
	Coin,
	Entity,
	EntityManager,
	Player
};
use crate::fiiish::EntityUpdateContext;

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
	entity_manager: EntityManager,
}

impl Game {
	pub fn new() -> Self {
		Self {
			players: Vec::new(),
			entity_manager: EntityManager::new(),
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		// load texture
		AnimatedTexture::register_all( system, renderer, "fish_swim", 4 );
		AnimatedTexture::register_all( system, renderer, "fish_die", 2 );

		let mut p = Player::new();
		p.setup( "player" );
		self.players.push( p );

		self.entity_manager.setup();

		for i in 0..32 {
			let r = 80.0+ (i as f32 * 8.0 );
			let fi = 2.2*( i as f32 )*( 3.14*2.0 )/32.0;
			let x = fi.sin() * r;
			let y = fi.cos() * r;
			let mut c = Coin::new( &Vector2::new( x, y ), i as u16 );
			c.setup( "coin" );

			self.entity_manager.add( Box::new( c ) );
		}
		
	}

	pub fn teardown( &mut self ) {
		self.entity_manager.teardown();
		for p in self.players.iter_mut() {
			p.teardown( );
		}
	}

	// :TODO: decide if we need the full WindowUpdateContext here
	pub fn update( &mut self, wuc: &mut WindowUpdateContext ) {

		for p in self.players.iter_mut() {
			if p.name() == "player" {
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
