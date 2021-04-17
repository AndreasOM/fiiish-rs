
use crate::math::Vector2;

#[derive(Debug)]
pub enum UiEvent {
	MouseClick{
		pos: Vector2,
		button: u8,
	}
}
