
use crate::renderer::{
	AnimatedTexture,
	Renderer,
	Texture,
};
use crate::system::System;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::entities::Player;
use crate::fiiish::EntityUpdateContext;

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
}

impl Game {
	pub fn new() -> Self {
		Self {
			players: Vec::new(),
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		// load texture
		renderer.register_texture( Texture::create( system, "fish_swim0000" ) );
		renderer.register_texture( Texture::create( system, "fish_swim0021" ) );
		renderer.register_texture( Texture::create( system, "fish_die00" ) );
		AnimatedTexture::register_all( system, renderer, "fish_swim", 4 );
		AnimatedTexture::register_all( system, renderer, "fish_die", 2 );

		let mut p = Player::new();
		p.setup( "player" );
		self.players.push( p );
	}

	pub fn teardown( &mut self ) {
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
	}

	pub fn render( &mut self, renderer: &mut Renderer) {
		for p in self.players.iter_mut() {
			p.render( renderer );
		}
	}
}
