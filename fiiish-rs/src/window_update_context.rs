
use crate::math::Vector2;

#[derive(Debug)]
pub struct WindowUpdateContext {
	pub time_step: f64,
	pub is_escaped_pressed: bool,
	pub is_space_pressed: bool,
	pub mouse_pos: Vector2,
	pub mouse_buttons: [bool;3],	// left middle right
	pub is_key_pressed: [bool;256],
	pub window_size: Vector2,

	previous_mouse_buttons: [bool;3],
	previous_keys_pressed: [bool;256],
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
			window_size: Vector2::zero(),

			previous_mouse_buttons: [false,false,false],
			previous_keys_pressed: [false;256],
		}
	}

	pub fn update( &mut self ) {
//		dbg!(&self);
		self.previous_mouse_buttons = self.mouse_buttons;
		self.previous_keys_pressed = self.is_key_pressed;
//		for i in 0..self.is_key_pressed.len() {
//			self.previous_keys_pressed[ i ] = self.is_key_pressed[ i ];
//		}
	}

	pub fn was_mouse_button_pressed( &self, button_index: usize ) -> bool {
		self.mouse_buttons[ button_index ] && !self.previous_mouse_buttons[ button_index ]
	}

	pub fn was_key_pressed( &self, key: u8 ) -> bool {
		self.is_key_pressed[ key as usize ] && !self.previous_keys_pressed[ key as usize ]
	}
}
