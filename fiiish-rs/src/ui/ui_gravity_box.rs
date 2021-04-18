
use crate::math::Vector2;
use crate::renderer::{
//	Color,
	Renderer,
//	Texture,
};

use crate::ui::{
	UiElement,
	UiElementContainer,
	UiElementContainerData,
};

#[derive(Debug)]
struct Child {
	pub element: Box< dyn UiElement >,
	pub gravity: Vector2,
}

#[derive(Debug)]
pub struct UiGravityBox {
	padding: f32,
	gravity: Vector2,
	children_gravities: Vec< Vector2 >
}

impl UiGravityBox {
	pub fn new( ) -> Self {
		Self {
			padding: 0.0,
			gravity: Vector2::zero(),
			children_gravities: Vec::new(),
		}
	}

	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
	}

	pub fn set_gravity( &mut self, gravity: &Vector2 ) {
		self.gravity = *gravity;
	}

}

impl UiElement for UiGravityBox {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn add_child( &mut self, _child: &mut UiElementContainerData ) {
		self.children_gravities.push( self.gravity );
	}

	fn layout( &mut self, container: &mut UiElementContainerData, pos: &Vector2 ) {
		let ws = container.size.sub( &Vector2::new( 2.0*self.padding, 2.0*self.padding ) );
		for ( g, c ) in self.children_gravities.iter().zip( container.borrow_children_mut().iter_mut() ) {
			let mut c = c.borrow_mut();
			let cs = c.size();
			let cpos = ws.sub( &cs ).scaled( 0.5 ).scaled_vector2( &g );
			c.layout( &cpos );
		}

		container.set_pos( pos );
	}
}
