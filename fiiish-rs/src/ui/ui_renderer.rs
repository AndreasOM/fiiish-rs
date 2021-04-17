
use crate::math::{
	Matrix32,
	Vector2,
};

use crate::renderer::{
	Color,
	Renderer,
};

pub struct UiRenderer<'a> {
	renderer: &'a mut Renderer,
	transform_stack: Vec< Matrix32 >,
	transform_mtx: Matrix32,
	opacity_stack: Vec< f32 >,
	opacity: f32,
}

impl <'a> UiRenderer<'a> {
	pub fn new( renderer: &'a mut Renderer ) -> Self {
		Self {
			renderer,
			transform_stack: Vec::new(),
			transform_mtx: Matrix32::identity(),
			opacity_stack: Vec::new(),
			opacity: 1.0,
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

	pub fn push_opacity( &mut self, opacity: f32 ) {
		self.opacity_stack.push( self.opacity );
		self.opacity *= opacity;
	}

	pub fn pop_opacity( &mut self ) {
		self.opacity_stack.pop();
	}

	pub fn use_texture( &mut self, name: &str ) {
		self.renderer.use_texture( name );
	}

	pub fn render_textured_quad( &mut self, pos: &Vector2, size: &Vector2 ) {
		let transformed_pos = self.transform_mtx.mul_vector2( &pos );
		let mut color = Color::white();
		color.a = self.opacity;
		self.renderer.set_color( &color );
		self.renderer.render_textured_quad( &transformed_pos, size );
	}
}
