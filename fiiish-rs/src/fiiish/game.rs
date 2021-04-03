
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
use crate::fiiish::EntityUpdateContext;
use crate::fiiish::Zone;

#[derive(Debug)]
pub struct Game {
	players: Vec<Player>,
	entity_manager: EntityManager,
	zone: Zone,
}

impl Game {
	pub fn new() -> Self {
		Self {
			players: Vec::new(),
			entity_manager: EntityManager::new(),
			zone: Zone::new(),
		}
	}

	pub fn setup(&mut self, system: &mut System, renderer: &mut Renderer) {
		// load texture
		AnimatedTexture::register_all( system, renderer, "fish_swim", 4 );
		AnimatedTexture::register_all( system, renderer, "fish_die", 2 );
		renderer.register_texture( Texture::create( system, "background" ) );
		renderer.register_texture( Texture::create( system, "background_grad" ) );

		self.entity_manager.setup();

		// load zone(s)
		self.zone.load( system, "0000_ILoveFiiish" );

//		dbg!(&self.zone);
//		todo!("die");

/*
#define	CRC_PICKUPCOIN	0xe4c651aa
#define	CRC_PICKUPRAIN	0x06fd4c5a
#define	CRC_PICKUPEXPLOSION	0xf75fd92f
#define	CRC_PICKUPMAGNET	0x235a41dd
#define	CRC_ROCKA	0xd058353c
#define	CRC_ROCKB	0x49516486
#define	CRC_ROCKC	0x3e565410
#define	CRC_ROCKD	0xa032c1b3
#define	CRC_ROCKE	0xd735f125
#define	CRC_ROCKF	0x4e3ca09f

#define	CRC_SEAWEEDA	0x6fe93bef
#define	CRC_SEAWEEDB	0xf6e06a55
#define	CRC_SEAWEEDC	0x81e75ac3
#define	CRC_SEAWEEDD	0x1f83cf60
#define	CRC_SEAWEEDE	0x6884fff6
#define	CRC_SEAWEEDF	0xf18dae4c
#define	CRC_SEAWEEDG	0x868a9eda

*/
		// :HACK:
		for l in self.zone.layer_iter() {
			for o in l.object_iter() {
				match o.crc {
					0xe4c651aa
					| 0x06fd4c5a
					| 0xf75fd92f
					| 0x235a41dd
					=> {
						//println!("Coin {:?}", &o );
						let mut c = Coin::new( &o.pos, 0, o.crc );
						c.setup( "coin" );

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
						r.setup( "rock" );
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
		b.setup( "backround" );
		self.entity_manager.add( Box::new( b ) );

/*
		let mut rf = Obstacle::new( &Vector2::new( 200.0, -300.0 ) );
		rf.setup( "rock-f" );
		self.entity_manager.add( Box::new( rf ) );

		let mut rf = Obstacle::new( &Vector2::new( 400.0, 300.0 ) );
		rf.setup( "rock-f" );
		rf.set_rotation( 180.0 );
		self.entity_manager.add( Box::new( rf ) );
*/
/*
		for i in 0..32 {
			let r = 80.0+ (i as f32 * 8.0 );
			let fi = 2.2*( i as f32 )*( 3.14*2.0 )/32.0;
			let x = fi.sin() * r;
			let y = fi.cos() * r;
			let mut c = Coin::new( &Vector2::new( x, y ), i as u16 );
			c.setup( "coin" );

			self.entity_manager.add( Box::new( c ) );
		}
*/		
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
