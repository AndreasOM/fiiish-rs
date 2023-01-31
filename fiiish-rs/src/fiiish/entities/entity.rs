use oml_game::renderer::Renderer;

use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::entities::EntityType;
use crate::fiiish::EntityUpdateContext;

#[derive(Debug)]
pub enum EntityState {
	Alive,
	Dieing { time: f64 },
	Dead,
}

#[derive(Debug)]
pub struct EntityData {
	pub state:       EntityState,
	pub dieing_time: f64,
}

const DIEING_TIME: f64 = 1.0;

impl EntityData {
	pub fn default() -> Self {
		Self {
			state:       EntityState::Alive,
			dieing_time: DIEING_TIME,
		}
	}

	pub fn update(&mut self, time_step: f64) {
		match self.state {
			EntityState::Dieing { time } => {
				let time = time + time_step;
				if time < self.dieing_time {
					self.state = EntityState::Dieing { time };
				} else {
					self.state = EntityState::Dead;
				}
			},
			_ => {},
		}
	}

	pub fn deadness(&self) -> f64 {
		match self.state {
			EntityState::Alive => 0.0,
			EntityState::Dead => 1.0,
			EntityState::Dieing { time } => time / self.dieing_time,
		}
	}
}

pub trait Entity {
	fn data(&self) -> &EntityData;

	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

	fn setup(&mut self, _ec: &EntityConfiguration) {}
	fn teardown(&mut self);
	fn update(&mut self, euc: &mut EntityUpdateContext);
	fn render(&mut self, renderer: &mut Renderer);

	fn name(&self) -> &str;
	fn entity_type(&self) -> EntityType;
	fn is_alive(&self) -> bool {
		match self.data().state {
			EntityState::Alive => true,
			_ => false,
		}
	}
	fn is_dead(&self) -> bool {
		match self.data().state {
			EntityState::Dead => true,
			_ => false,
		}
	}
	fn kill(&mut self) {}
}

impl std::fmt::Debug for dyn Entity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(
			f,
			"[Trait] Entity: {} [{:?}] {}",
			self.name(),
			self.entity_type(),
			if self.is_alive() { "[ALIVE]" } else { "[DEAD]" }
		)
	}
}
