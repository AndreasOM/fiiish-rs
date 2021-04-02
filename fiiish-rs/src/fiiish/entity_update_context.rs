

#[derive(Debug)]
pub struct EntityUpdateContext {
	time_step: f64,
	change_background_state: bool,
}

impl EntityUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step: 0.0,
			change_background_state: false,
		}
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}

	pub fn set_time_step( mut self, time_step: f64 ) -> Self {
		self.time_step = time_step;
		self
	}

	pub fn change_background_state( &self ) -> bool {
		self.change_background_state
	}
	pub fn enable_change_background_state( &mut self ) {
		self.change_background_state = true;
	}
}