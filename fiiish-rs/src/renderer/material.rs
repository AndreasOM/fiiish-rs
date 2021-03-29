
use crate::renderer::{
	Debug,
	Effect,
	gl,
	Program,
	ShaderType,
	Vertex,
};

//#[derive(Debug)]
pub struct Material {
	vertices: Vec<Vertex>,
	buffer: gl::types::GLuint,
	vao: gl::types::GLuint,

	effect_id: u16,
	effect_name: String,
//	program: Program,
}

impl Material {

	pub fn new( effect: &Effect ) -> Self {
		let mut s = Self {
			vertices: Vec::new(),
			buffer: 0xffffffff,
			vao: 0xffffffff,
			effect_id: effect.id(),
			effect_name: effect.name().to_string(),
//			program: Program::new(),
		};

		unsafe {
			gl::GenVertexArrays( 1, &mut s.vao );
			gl::GenBuffers( 1, &mut s.buffer );
		}

		s
	}

	pub fn can_render( &self, effect_id: u16 ) -> bool {
		self.effect_id == effect_id
	}

	pub fn effect_name( &self ) -> &str {
		&self.effect_name
	}

	pub fn clear( &mut self ) {
		self.vertices.clear();
	}

	pub fn add_vertex( &mut self, vertex: &Vertex ) {
		self.vertices.push( *vertex );
	}

	pub fn render( &mut self, effect: &mut Effect ) -> u32 {
		unsafe {
			gl::Enable( gl::BLEND );
			gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA );

//			gl::Disable( gl::BLEND );
//			gl::BlendFunc( gl::ONE, gl::ONE );

			gl::BindVertexArray( self.vao );
			gl::BindBuffer( gl::ARRAY_BUFFER, self.buffer );

//			let vertex_size = ( core::mem::size_of<gl::type::GLFloat>() * 3 ) as i32;
			let vertex_size = ( core::mem::size_of::<f32>( ) * 3 ) as isize;
			let vertex_count = self.vertices.len();
			// :TODO: we might want to reuse this
			gl::BufferData(
				gl::ARRAY_BUFFER,
				vertex_size * vertex_count as isize,
				self.vertices.as_ptr() as *const core::ffi::c_void,
				gl::STATIC_DRAW		 									//maybe STREAM?
			);

			let attrib_index = 0;

			gl::EnableVertexAttribArray( attrib_index );
			gl::VertexAttribPointer( attrib_index, 3, gl::FLOAT, gl::FALSE, vertex_size as i32, 0 as *const _ );

			effect.r#use();
//			self.program.r#use();

//			gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
			gl::DrawArrays( gl::TRIANGLES, 0, vertex_count as i32 );
//			println!("Rendering {} vertices", vertex_count);
			vertex_count as u32
		}
//		dbg!(&self);
	}
}

impl std::fmt::Debug for Material {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!(
			f, "Material: {} {} effect: {} [{}]",
			self.buffer,
			self.vao,
			&self.effect_name,
			self.effect_id,
		)
	}
}

