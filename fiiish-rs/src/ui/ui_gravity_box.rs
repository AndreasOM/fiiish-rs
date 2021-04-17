
use crate::math::Vector2;
use crate::renderer::{
//	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiElementBase,
	UiElementFadeState,
	UiRenderer,
};

#[derive(Debug)]
struct Child {
	pub element: Box< dyn UiElement >,
	pub gravity: Vector2,
}

#[derive(Debug)]
pub struct UiGravityBox {
	base: UiElementBase,
	padding: f32,
	children_gravities: Vec< Vector2 >
}

impl UiGravityBox {
	pub fn new( ) -> Self {
		Self {
			base: UiElementBase::new(),
			padding: 0.0,
			children_gravities: Vec::new(),
		}
	}

	pub fn set_size( &mut self, size: &Vector2 ) {
		self.borrow_base_mut().size = *size;
	}
	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
	}

	pub fn add_child( &mut self, element: Box< dyn UiElement >, gravity: &Vector2 ) {
		self.borrow_base_mut().children.push( element );
		self.children_gravities.push( *gravity );
	}
}

impl UiElement for UiGravityBox {
	fn layout( &mut self, pos: &Vector2 ) {
		let ws = self.borrow_base().size.sub( &Vector2::new( 2.0*self.padding, 2.0*self.padding ) );
		for ( g, c ) in self.children_gravities.iter().zip( self.base.children.iter_mut() ) {
			let cs = c.size();
			let cpos = ws.sub( &cs ).scaled( 0.5 ).scaled_vector2( &g );
			c.layout( &cpos );
		}

		self.borrow_base_mut().pos = *pos;
	}

	fn borrow_base( &self ) -> &UiElementBase {
		&self.base
	}

	fn borrow_base_mut( &mut self ) -> &mut UiElementBase {
		&mut self.base
	}
}
