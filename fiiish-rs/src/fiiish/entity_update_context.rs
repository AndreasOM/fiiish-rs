
use crate::math::Vector2;

use crate::fiiish::game::GameState;

#[derive(Debug)]
pub struct EntityUpdateContext {
	time_step: f64,
	world_movement: Vector2,
	change_background_state: bool,
	game_state: GameState,
}

impl EntityUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step: 0.0,
			world_movement: Vector2::zero(),
			change_background_state: false,
			game_state: GameState::None,
		}
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}

	pub fn set_time_step( mut self, time_step: f64 ) -> Self {
		self.time_step = time_step;
		self
	}

	pub fn world_movement(&self) -> &Vector2 {
		&self.world_movement
	}

	pub fn set_world_movement(&mut self, world_movement: &Vector2 ) {
		self.world_movement = *world_movement;
	}

	pub fn set_game_state(&mut self, game_state: &GameState ) {
		self.game_state = *game_state;
	}

	pub fn game_state( &self ) -> &GameState {
		&self.game_state
	}

	pub fn change_background_state( &self ) -> bool {
		self.change_background_state
	}
	pub fn enable_change_background_state( &mut self ) {
		self.change_background_state = true;
	}
}