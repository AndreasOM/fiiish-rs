
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
pub struct UiVbox {
	base: UiElementBase,
	padding: f32,
}

impl UiVbox {
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
			total_size.y += cs.y + self.padding;
			if total_size.x < cs.x {
				total_size.x = cs.x;
			}
		}
		total_size.y -= self.padding;

		self.borrow_base_mut().size = total_size;
	}
}

impl UiElement for UiVbox {
	fn update( &mut self, time_step: f64 ) {
		self.update_fade_state( time_step );
		for c in self.borrow_base_mut().children.iter_mut() {
			c.update( time_step );
		}
	}
	fn layout( &mut self, pos: &Vector2 ) {
		let mut total_size = Vector2::zero();
		let mut c_positions_y = Vec::new();
		let padding = self.padding;

		let mut w1 = 0.0;
		let mut w0;

		for c in self.borrow_base().children.iter() {
			let cs = *c.size();
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

		for (i, c ) in self.borrow_base_mut().children.iter_mut().enumerate() {
			let y = c_positions_y[ i ];
			cpos.y += y + padding;
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
