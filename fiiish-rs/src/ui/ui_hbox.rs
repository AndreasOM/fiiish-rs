
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
pub struct UiHbox {
	base: UiElementBase,
	padding: f32,
}

impl UiHbox {
	pub fn new( ) -> Self {
		Self {
			base: UiElementBase::new(),
			padding: 0.0,
		}
	}

	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
		self.recalculate_size();
	}

	pub fn add_child( &mut self, child: Box< dyn UiElement > ) {
		self.borrow_base_mut().children.push( child );
		self.recalculate_size();
	}

	fn recalculate_size( &mut self ) {
		let mut total_size = Vector2::zero();

		for c in self.borrow_base().children.iter() {
			let cs = c.size();
			total_size.x += cs.x + self.padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
		}
		total_size.x -= self.padding;

		self.borrow_base_mut().size = total_size;
	}

}

impl UiElement for UiHbox {
	fn layout( &mut self, pos: &Vector2 ) {
		let mut total_size = Vector2::zero();
		let mut c_positions_x = Vec::new();
		let padding = self.padding;

		let mut w1 = 0.0;
		let mut w0 = 0.0;

		for c in self.borrow_base().children.iter() {
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

		for (i, c ) in self.borrow_base_mut().children.iter_mut().enumerate() {
			let x = c_positions_x[ i ];
			cpos.x += x + padding;
			c.layout( &cpos );
		}

		self.borrow_base_mut().pos = *pos;
		self.borrow_base_mut().size = total_size;
	}

	fn borrow_base( &self ) -> &UiElementBase {
		&self.base
	}

	fn borrow_base_mut( &mut self ) -> &mut UiElementBase {
		&mut self.base
	}
	
}
