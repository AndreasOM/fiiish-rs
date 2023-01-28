
use oml_game::math::Vector2;


use crate::ui::*;

#[derive(Debug)]
pub struct UiHbox {
	padding: f32,
}

impl UiHbox {
	pub fn new( ) -> Self {
		Self {
			padding: 0.0,
		}
	}

	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
//		self.recalculate_size();	// :TODO:
	}

}

impl UiElement for UiHbox {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
	fn recalculate_size( &mut self, container: &mut UiElementContainerData ) {
		let mut total_size = Vector2::zero();

		for c in container.borrow_children().iter() {
			let c = c.borrow();
			let cs = c.size();
			total_size.x += cs.x + self.padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
		}
		total_size.x -= self.padding;

		container.set_size( &total_size );
	}


	fn layout( &mut self, container: &mut UiElementContainerData, pos: &Vector2 ) {
		let mut total_size = Vector2::zero();
		let mut c_positions_x = Vec::new();
		let padding = self.padding;

		let mut w1 = 0.0;
		let mut w0; // = 0.0;

		for c in container.borrow_children().iter() {
			let c = c.borrow();
			let cs = c.size();
			total_size.x += cs.x + padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
			w0 = w1;
			w1 = 0.5 * cs.x;
			c_positions_x.push( w0 +  w1 );
		}
		total_size.x -= padding;

		c_positions_x.push( 0.0 );

		let mut cpos = Vector2::new( -0.5*total_size.x - self.padding, 0.0 );

		for (i, c ) in container.borrow_children_mut().iter_mut().enumerate() {
			let x = c_positions_x[ i ];
			cpos.x += x + padding;
			c.borrow_mut().layout( &cpos );
		}

		container.set_pos( pos );
		container.set_size( &total_size );
	}

}
