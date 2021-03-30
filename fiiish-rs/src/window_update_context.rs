
use crate::math::Vector2;

#[derive(Debug)]
pub struct WindowUpdateContext {
	pub time_step: f64,
	pub is_escaped_pressed: bool,
	pub is_space_pressed: bool,
	pub mouse_pos: Vector2,
	pub mouse_buttons: [bool;3],	// left middle right
	pub is_key_pressed: [bool;256],

	previous_mouse_buttons: [bool;3],
}

impl WindowUpdateContext {

	pub fn new() -> Self {
		Self {
			time_step: 0.0,
			is_escaped_pressed: false,
			is_space_pressed: false,
			mouse_pos: Vector2::zero(),
			mouse_buttons: [false,false,false],
			is_key_pressed: [false;256],

			previous_mouse_buttons: [false,false,false],
		}
	}

	pub fn update( &mut self ) {
//		dbg!(&self);
		self.previous_mouse_buttons = self.mouse_buttons;
	}

	pub fn was_mouse_button_pressed( &self, button_index: usize ) -> bool {
		self.mouse_buttons[ button_index ] && !self.previous_mouse_buttons[ button_index ]
	}
}
