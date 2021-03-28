
use crate::renderer::{
	Debug,
	gl,
	Program,
	ShaderType,
	Vertex,
};

#[derive(Debug)]
pub struct Material {
	vertices: Vec<Vertex>,
	buffer: gl::types::GLuint,
	vao: gl::types::GLuint,

	program: Program,
}

const VS_SRC: &str = "
#version 330 core
layout (location = 0) in vec3 aPos;
out vec2 screen_pos;

void main()
{
	screen_pos = aPos.xy;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const FS_SRC: &str = "
#version 330 core
out vec4 FragColor;
in vec2 screen_pos;

void main()
{
    FragColor = vec4(1.0f, abs(screen_pos.x*2.0), abs(screen_pos.y*2.0), 0.125f);
} 
";

impl Material {

	pub fn new() -> Self {
		let mut s = Self {
			vertices: Vec::new(),
			buffer: 0xffffffff,
			vao: 0xffffffff,
			program: Program::new(),
		};

		unsafe {
			gl::GenVertexArrays( 1, &mut s.vao );
			gl::GenBuffers( 1, &mut s.buffer );
		}
		s.program.add_shader( ShaderType::Vertex, &VS_SRC );
		s.program.add_shader( ShaderType::Fragment, &FS_SRC );
		s.program.link();

		s
	}

	pub fn clear( &mut self ) {
		self.vertices.clear();
	}

	pub fn add_vertex( &mut self, vertex: &Vertex ) {
		self.vertices.push( *vertex );
	}

	pub fn render( &mut self ) {
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

			self.program.r#use();

//			gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
			gl::DrawArrays( gl::TRIANGLES, 0, vertex_count as i32 );
//			println!("Rendering {} vertices", vertex_count);
		}
//		dbg!(&self);
	}
}

