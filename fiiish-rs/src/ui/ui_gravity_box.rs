
use crate::math::Vector2;
use crate::renderer::{
//	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiRenderer,
};

#[derive(Debug)]
struct Child {
	pub element: Box< dyn UiElement >,
	pub gravity: Vector2,
}

#[derive(Debug)]
pub struct UiGravityBox {
	pos: Vector2,
	size: Vector2,
	padding: f32,
	children: Vec< Child > ,
}

impl UiGravityBox {
	pub fn new( ) -> Self {
		Self {
			pos: Vector2::zero(),
			size: Vector2::zero(),
			padding: 0.0,
			children: Vec::new(),
		}
	}

	pub fn set_size( &mut self, size: &Vector2 ) {
		self.size = *size;
	}
	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
	}

	pub fn add_child( &mut self, element: Box< dyn UiElement >, gravity: &Vector2 ) {
		self.children.push( Child { element, gravity: *gravity } );
	}
}

impl UiElement for UiGravityBox {
	fn update( &mut self, _time_step: f64 ) {
	}
	fn render( &self, ui_renderer: &mut UiRenderer) {
		ui_renderer.push_translation( &self.pos );
		for c in self.children.iter() {
			c.element.render( ui_renderer );
		}
		ui_renderer.pop_transform();
	}
	fn layout( &mut self, pos: &Vector2 ) {
		let ws = self.size.sub( &Vector2::new( 2.0*self.padding, 2.0*self.padding ) );
		for c in self.children.iter_mut() {
			let cs = c.element.size();
			let cpos = ws.sub( &cs ).scaled( 0.5 ).scaled_vector2( &c.gravity );
			c.element.layout( &cpos );
		}

		self.pos = *pos;
	}
	fn size( &self ) -> &Vector2 {
		&self.size
	}

	fn pos( &self ) -> &Vector2 {
		&self.pos
	}

}
