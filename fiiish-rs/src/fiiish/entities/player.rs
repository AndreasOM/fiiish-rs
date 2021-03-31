
use crate::fiiish::effect_ids::EffectId;
use crate::fiiish::EntityUpdateContext;
use crate::math::Vector2;
use crate::renderer::{
	Renderer,
};

#[derive(Debug,PartialEq,Eq)]
enum PlayerState {
	WaitForStart,
	Swimming,
	Dying,
	Dead,
}

#[derive(Debug)]
pub struct Player {
	name: String,
	spawn_pos: Vector2,
	pos: Vector2,
	size: Vector2,
	state: PlayerState,
	going_down: bool,
	speed: f32,
}

impl Player {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			spawn_pos: Vector2::new( -512.0, 0.0 ),
			pos: Vector2::zero(),
			size: Vector2::new( 128.0, 128.0 ),
			state: PlayerState::Dead,
			going_down: false,
			speed: 100.0,
		}
	}

	pub fn setup( &mut self, name: &str ) {
		self.name = name.to_owned();
	}

	pub fn teardown( &mut self ) {

	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn is_alive( &self ) -> bool {
		match self.state {
			PlayerState::Dead | PlayerState::Dying => false,
			PlayerState::WaitForStart | PlayerState::Swimming => true,
		}
	}

	pub fn can_respawn( &self ) -> bool {
		self.state == PlayerState::Dead
	}

	fn goto_state( &mut self, state: PlayerState ) {
		match state {
			PlayerState::WaitForStart => {
				self.pos = self.spawn_pos;
			},
			_ => {},
		}
		self.state = state;	// :TODO: handle transitions if needed
	}

	pub fn respawn( &mut self ) {
		match self.state {
			PlayerState::Dead => {
				self.goto_state( PlayerState::WaitForStart );
			},
			_ => {},
		}		
	}

	pub fn turn_down( &mut self ) {
		match self.state {
			PlayerState::WaitForStart => {
				self.goto_state( PlayerState::Swimming );
			},
			PlayerState::Swimming => {
				self.going_down = true;
			}
			_ => {},
		}
	}

	pub fn turn_up( &mut self ) {
		self.going_down = false;	
	}

	fn update_swimming( &mut self, euc: &mut EntityUpdateContext ) {
		if self.going_down {
			self.pos.y -= self.speed*euc.time_step() as f32;
			if self.pos.y < -512.0 {
				self.goto_state( PlayerState::Dying );
			}
		} else {
			self.pos.y += self.speed*euc.time_step() as f32;
			if self.pos.y > 512.0 {
				self.goto_state( PlayerState::Dying );
			}
		}		
	}
	fn update_dying( &mut self, euc: &mut EntityUpdateContext ) {
		self.pos.y += self.speed*euc.time_step() as f32;
		if self.pos.y > 512.0+128.0 {
			self.goto_state( PlayerState::Dead );
		}
	}

	pub fn update( &mut self, euc: &mut EntityUpdateContext ) {
		// :TODO: time step
		match self.state {
			PlayerState::Swimming => self.update_swimming( euc ),
			PlayerState::Dying => self.update_dying( euc ),
			_ => {},
		}
	}

	pub fn render( &mut self, renderer: &mut Renderer ) {
		if self.state == PlayerState::Dead {
			// dead means offscreen, nothing to be rendered
			return;
		}

		renderer.use_effect( EffectId::Textured as u16 );
		match self.state {
			PlayerState::Dying | PlayerState::Dead => renderer.use_texture( "fish_die00" ),
			_ => renderer.use_texture( "fish_swim0000" ),
		}
		renderer.render_textured_quad( &self.pos, &self.size );
	}
}
