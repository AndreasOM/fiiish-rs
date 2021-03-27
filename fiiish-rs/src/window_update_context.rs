
#[derive(Debug)]
pub struct WindowUpdateContext {
	pub time_step: f64,
	pub is_escaped_pressed: bool,
	pub is_space_pressed: bool,
}

impl WindowUpdateContext {

	pub fn new() -> Self {
		Self {
			time_step: 0.0,
			is_escaped_pressed: false,
			is_space_pressed: false,
		}
	}
}
