use oml_game::math::Vector2;
use oml_game::window::window_update_context::WindowUpdateContext;

#[derive(Debug, Copy, Clone)]
pub struct AppUpdateContext {
	time_step:  f64,
	cursor_pos: Vector2,
	wuc:        Option<WindowUpdateContext>,
}

impl AppUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step:  0.0,
			cursor_pos: Vector2::zero(),
			wuc:        None,
		}
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}

	pub fn set_time_step(mut self, time_step: f64) -> Self {
		self.time_step = time_step;
		self
	}

	pub fn cursor_pos(&self) -> &Vector2 {
		&self.cursor_pos
	}

	pub fn set_cursor_pos(mut self, cursor_pos: &Vector2) -> Self {
		self.cursor_pos = *cursor_pos;
		self
	}

	pub fn wuc(&self) -> Option<WindowUpdateContext> {
		self.wuc
	}

	pub fn set_wuc(mut self, wuc: &WindowUpdateContext) -> Self {
		self.wuc = Some(*wuc);
		self
	}
}
