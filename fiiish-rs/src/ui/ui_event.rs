
use oml_game::math::Vector2;

#[derive(Debug)]
pub enum UiEvent {
	MouseClick{
		pos: Vector2,
		button: u8,
	}
}


// :TODO: move
#[derive(Debug)]
pub struct UiEventResponseButtonClicked {
	pub button_name: String,
}

impl UiEventResponseButtonClicked {
	pub fn new( button_name: &str ) -> Self {
		Self {
			button_name: button_name.to_owned(),
		}
	}
}

// :TODO: use derived Debug where available
impl UiEventResponse for UiEventResponseButtonClicked {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!( f, "UiEventResponseButtonClicked -> {}", self.button_name )
	}
}

pub trait UiEventResponse {
	fn as_any(&self) -> &dyn std::any::Any;
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!( f, "[Trait] UiEventResponse" )
	}
}

impl std::fmt::Debug for dyn UiEventResponse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		self.fmt( f )
	}
}