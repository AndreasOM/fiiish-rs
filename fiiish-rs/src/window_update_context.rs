
#[derive(Debug)]
pub struct WindowUpdateContext {
	pub is_escaped_pressed: bool,
	pub is_space_pressed: bool,
}

impl WindowUpdateContext {

	pub fn new() -> Self {
		Self {
			is_escaped_pressed: false,
			is_space_pressed: false,
		}
	}
}
