
use crate::math::Vector2;
use crate::ui::UiRenderer;

pub trait UiElement {
	fn update( &mut self, time_step: f64 );
	fn render( &self, ui_renderer: &mut UiRenderer);
	fn layout( &mut self, pos: &Vector2 ){}
	fn size( &self ) -> &Vector2;
	fn pos( &self ) -> &Vector2;
}

impl std::fmt::Debug for dyn UiElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!( f,"[Trait] UiElement: {}x{} @ {}, {}", self.size().x, self.size().y, self.pos().x, self.pos().y )
	}
}
