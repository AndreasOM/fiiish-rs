
use crate::math::Vector2;

use crate::ui::{
	UiElement,
	UiElementFadeState
};

#[derive(Debug)]
pub struct UiElementBase {
	pub pos: Vector2,
	pub size: Vector2,
	pub fade_state: UiElementFadeState,
	pub children: Vec< Box< dyn UiElement > > ,
}

impl UiElementBase {
	pub fn new() -> Self {
		Self {
			pos: Vector2::zero(),
			size: Vector2::zero(),
			fade_state: UiElementFadeState::FadedIn,
			children: Vec::new(),
		}
	}
}
