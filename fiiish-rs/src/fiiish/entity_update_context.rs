

#[derive(Debug)]
pub struct EntityUpdateContext {
	time_step: f64,
}

impl EntityUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step: 0.0,
		}
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}

	pub fn set_time_step( mut self, time_step: f64 ) -> Self {
		self.time_step = time_step;
		self
	}
}