
use crate::math::{
	Matrix32,
	Vector2,
};

use crate::renderer::Renderer;

pub struct UiRenderer<'a> {
	renderer: &'a mut Renderer,
	transform_stack: Vec< Matrix32 >,
	transform_mtx: Matrix32,
}

impl <'a> UiRenderer<'a> {
	pub fn new( renderer: &'a mut Renderer ) -> Self {
		Self {
			renderer,
			transform_stack: Vec::new(),
			transform_mtx: Matrix32::identity(),
		}
	}

	pub fn push_translation( &mut self, t: &Vector2 ) {
		self.transform_stack.push( self.transform_mtx );
		self.transform_mtx.pos = self.transform_mtx.pos.add( &t );
	}

	pub fn pop_transform( &mut self ) {
		// :TODO: protect against mismatched push/pop
		self.transform_mtx = self.transform_stack.pop().unwrap();
	}

	pub fn use_texture( &mut self, name: &str ) {
		self.renderer.use_texture( name );
	}

	pub fn render_textured_quad( &mut self, pos: &Vector2, size: &Vector2 ) {
		let transformed_pos = self.transform_mtx.mul_vector2( &pos );
		self.renderer.render_textured_quad( &transformed_pos, size );
	}
}
