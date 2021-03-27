
use crate::renderer::{
	gl,
	Vertex,
};

#[derive(Debug)]
pub struct Material {
	vertices: Vec<Vertex>,
	buffer: gl::types::GLuint,
	vao: gl::types::GLuint,

	// :HACK:
	vs: gl::types::GLuint,
	fs: gl::types::GLuint,
	prog: gl::types::GLuint,
}

const VS_SRC:&'static [u8] = b"
#version 330 core
layout (location = 0) in vec3 aPos;
out vec2 screen_pos;
void main()
{
	screen_pos = aPos.xy;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
\0";

const FS_SRC:&'static [u8] = b"
#version 330 core
out vec4 FragColor;
in vec2 screen_pos;
void main()
{
    FragColor = vec4(1.0f, 0.5f, abs(screen_pos.y*2.0), 1.0f);
} 
\0";


fn check_gl_error( line: u32 ) {
	unsafe {
		match gl::GetError() {
			gl::NO_ERROR => {},
			e => {
				println!("GL Error in line {}: {}", line,
					match e {
						gl::INVALID_VALUE => "INVALID_VALUE".to_string(),
						gl::INVALID_OPERATION => "INVALID_OPERATION".to_string(),
						e => format!("{}",e ),
					}
				);
			},
		}
	}
}

impl Material {

	pub fn new() -> Self {
		let mut s = Self {
			vertices: Vec::new(),
			buffer: 0xffffffff,
			vao: 0xffffffff,
			vs: 0xffffffff,
			fs: 0xffffffff,
			prog: 0xffffffff,
		};

		unsafe {
			gl::GenVertexArrays( 1, &mut s.vao );
			gl::GenBuffers( 1, &mut s.buffer );

			s.vs = gl::CreateShader( gl::VERTEX_SHADER );
			gl::ShaderSource( s.vs, 1, [ VS_SRC.as_ptr() as *const _ ].as_ptr(), std::ptr::null() );
			gl::CompileShader( s.vs );

			s.fs = gl::CreateShader( gl::FRAGMENT_SHADER );
			gl::ShaderSource( s.fs, 1, [ FS_SRC.as_ptr() as *const _ ].as_ptr(), std::ptr::null() );
			gl::CompileShader( s.fs );

			// :TODO: actually do the error handling, and reporting
			let mut log_length = 0;
			gl::GetShaderiv( s.fs, gl::INFO_LOG_LENGTH, &mut log_length );
			dbg!(&log_length);


			s.prog = gl::CreateProgram();
			gl::AttachShader( s.prog, s.fs );
			gl::AttachShader( s.prog, s.vs );
			gl::LinkProgram( s.prog );

			let mut status = 0;
			gl::GetProgramiv( s.prog, gl::LINK_STATUS, &mut status );
			dbg!(&status);

			check_gl_error( std::line!() );
		}

		s
	}

	/*
GLuint vaoId = 0;
glGenVertexArrays(1, &vaoId);
glBindVertexArray(vaoId);
	*/	

	pub fn clear( &mut self ) {
		self.vertices.clear();
	}

	pub fn add_vertex( &mut self, vertex: &Vertex ) {
		self.vertices.push( *vertex );
	}

	pub fn render( &mut self ) {
		unsafe {
			check_gl_error( std::line!() );
			gl::Enable( gl::BLEND );
			gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA );

			gl::Disable( gl::BLEND );
			gl::BlendFunc( gl::ONE, gl::ONE );

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
			check_gl_error( std::line!() );

			let attrib_index = 0;

			gl::EnableVertexAttribArray( attrib_index );
			check_gl_error( std::line!() );
			gl::VertexAttribPointer( attrib_index, 3, gl::FLOAT, gl::FALSE, vertex_size as i32, 0 as *const _ );
			check_gl_error( std::line!() );

			gl::UseProgram( self.prog );
			check_gl_error( std::line!() );
//			gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
//			let number_of_triangles = ( vertex_count / 3 ) as i32;
			gl::DrawArrays( gl::TRIANGLES, 0, vertex_count as i32 );
//			println!("Rendering {} vertices", vertex_count);
			check_gl_error( std::line!() );
		}
//		dbg!(&self);
	}
}

