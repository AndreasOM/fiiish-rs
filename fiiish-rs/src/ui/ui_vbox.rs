
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
	UiElementFadeState,
	UiRenderer,
};

#[derive(Debug)]
pub struct UiVbox {
	padding: f32,
}

impl UiVbox {
	pub fn new( ) -> Self {
		Self {
			padding: 0.0,
		}
	}

	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
//		self.recalculate_size(); // :TODO:
	}

}

impl UiElement for UiVbox {
	fn recalculate_size( &mut self, container: &mut UiElementContainerData ) {
		let mut total_size = Vector2::zero();

		for c in container.borrow_children().iter() {
			let c = c.borrow();
			let cs = c.size();
			total_size.y += cs.y + self.padding;
			if total_size.x < cs.x {
				total_size.x = cs.x;
			}
		}
		total_size.y -= self.padding;

		container.set_size( &total_size );
	}


	fn layout( &mut self, container: &mut UiElementContainerData, pos: &Vector2 ) {
		let mut total_size = Vector2::zero();
		let mut c_positions_y = Vec::new();
		let padding = self.padding;

		let mut w1 = 0.0;
		let mut w0;

		for c in container.borrow_children().iter() {
			let cs = *c.borrow().size();
			total_size.y += cs.y + padding;
			if total_size.x < cs.x {
				total_size.x = cs.x;
			}
			w0 = w1;
			w1 = 0.5 * cs.y;
			c_positions_y.push( w0 +  w1 );
		}
		total_size.y -= padding;

		c_positions_y.push( 0.0 );

		let mut cpos = Vector2::new( 0.0, -0.5*total_size.y - self.padding );

		for (i, c ) in container.borrow_children_mut().iter_mut().enumerate() {
			let y = c_positions_y[ i ];
			cpos.y += y + padding;
			c.borrow_mut().layout( &cpos );
		}

		container.set_pos( pos );
		container.set_size( &total_size );
	}

}
