
use crate::math::Vector2;

#[derive(Debug)]
pub struct UiElementBase {
	pub pos: Vector2,
	pub size: Vector2,
}

impl UiElementBase {
	pub fn new() -> Self {
		Self {
			pos: Vector2::zero(),
			size: Vector2::zero(),
		}
	}
}
