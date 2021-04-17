
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
pub struct UiVbox {
	pos: Vector2,
	size: Vector2,
	padding: f32,
	children: Vec< Box< dyn UiElement > > ,
}

impl UiVbox {
	pub fn new( ) -> Self {
		Self {
			pos: Vector2::zero(),
			size: Vector2::zero(),
			padding: 0.0,
			children: Vec::new(),
		}
	}

	pub fn set_padding( &mut self, padding: f32 ) {
		self.padding = padding;
		self.recalculate_size();
	}

	pub fn add_child( &mut self, child: Box< dyn UiElement > ) {
		self.children.push( child );
		self.recalculate_size();
	}

	fn recalculate_size( &mut self ) {
		let mut total_size = Vector2::zero();

		for c in self.children.iter() {
			let cs = c.size();
			total_size.y += cs.y + self.padding;
			if total_size.x < cs.x {
				total_size.x = cs.x;
			}
		}
		total_size.y -= self.padding;

		self.size = total_size;
	}
}

impl UiElement for UiVbox {
	fn update( &mut self, _time_step: f64 ) {
	}
	fn render( &self, ui_renderer: &mut UiRenderer) {
		ui_renderer.push_translation( &self.pos );
		for c in self.children.iter() {
			c.render( ui_renderer );
		}
		ui_renderer.pop_transform();
	}
	fn layout( &mut self, pos: &Vector2 ) {
		let mut total_size = Vector2::zero();
		let mut c_positions_y = Vec::new();

		let mut w1 = 0.0;
		let mut w0 = 0.0;

		for c in self.children.iter() {
			let cs = *c.size();
			total_size.y += cs.y + self.padding;
			if total_size.x < cs.x {
				total_size.x = cs.x;
			}
			w0 = w1;
			w1 = 0.5 * cs.y;
			c_positions_y.push( w0 +  w1 );
		}
		total_size.y -= self.padding;

		c_positions_y.push( 0.0 );

		let mut cpos = Vector2::new( 0.0, -0.5*total_size.y - self.padding );

		for (i, c ) in self.children.iter_mut().enumerate() {
			let y = c_positions_y[ i ];
			cpos.y += y + self.padding;
			c.layout( &cpos );
		}

		self.pos = *pos;
		self.size = total_size;
	}
	fn size( &self ) -> &Vector2 {
		&self.size
	}

	fn pos( &self ) -> &Vector2 {
		&self.pos
	}

}
