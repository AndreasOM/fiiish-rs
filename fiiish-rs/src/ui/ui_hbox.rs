
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
pub struct UiHbox {
	pos: Vector2,
	size: Vector2,
	padding: f32,
	children: Vec< Box< dyn UiElement > > ,
}

impl UiHbox {
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
			total_size.x += cs.x + self.padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
		}
		total_size.x -= self.padding;

		self.size = total_size;
	}

}

impl UiElement for UiHbox {
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
		let mut c_positions_x = Vec::new();

		let mut w1 = 0.0;
		let mut w0 = 0.0;

		for c in self.children.iter() {
			let cs = c.size();
			total_size.x += cs.x + self.padding;
			if total_size.y < cs.y {
				total_size.y = cs.y;
			}
			w0 = w1;
			w1 = 0.5 * cs.x;
			c_positions_x.push( w0 +  w1 );
		}
		total_size.x -= self.padding;

		c_positions_x.push( 0.0 );

		let mut cpos = Vector2::new( -0.5*total_size.x, 0.0 );

		for (i, c ) in self.children.iter_mut().enumerate() {
			c.layout( &cpos );
			let x = c_positions_x[ i+1 ];
			cpos.x += x + self.padding;
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
