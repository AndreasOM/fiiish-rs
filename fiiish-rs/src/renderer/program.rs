
use std::collections::HashMap;

//use crate::renderer::Program;
use crate::renderer::{
	Debug,
	gl,
};

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub enum ShaderType {
	Vertex,
	Fragment,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Program {

//	shader_ids: HashMap<ShaderType, gl::types::GLuint>,
	shader_ids: Vec< (ShaderType, gl::types::GLuint) >,
	program_id: gl::types::GLuint,

	uniforms: HashMap< String, i32 >,

	#[derivative(Debug="ignore")]
	// :TODO: make debug only
	shader_sources: Vec< (ShaderType, String) >,
}

impl Program {
	pub fn new() -> Self {
		Self {
//			shader_ids: HashMap::new(),
			shader_ids: Vec::new(),
			program_id: 0xffffffff,
			uniforms: HashMap::new(),

			shader_sources: Vec::new(),
		}
	}

	pub fn add_shader( &mut self, shader_type: ShaderType, source: &str ) {
		let gl_shader_type = match shader_type {
			ShaderType::Vertex => gl::VERTEX_SHADER,
			ShaderType::Fragment => gl::FRAGMENT_SHADER,
			_ => todo!("{:?}", &shader_type),
		};

		unsafe {
			let id = gl::CreateShader( gl_shader_type );
			gl::ShaderSource(
				id,
				1,
				[ source.as_ptr() as *const _].as_ptr() as *const _,
				//std::ptr::null(),
				[ source.len() ].as_ptr() as *const _,
			);
			gl::CompileShader( id );

			let mut log_length = 0;
			gl::GetShaderiv( id, gl::INFO_LOG_LENGTH, &mut log_length );
			if log_length > 0 {
				// :TODO: get actual log
				println!("Warning: LogLength {} for shader {:?}\n{}", &log_length, &shader_type, &source );

				let mut buf: Vec<u8> = Vec::with_capacity( log_length as usize );
				gl::GetShaderInfoLog( id, log_length, &mut log_length, buf.as_mut_ptr() as *mut _ );
				buf.set_len( log_length as usize );
				let s = String::from_utf8( buf ).unwrap();
				println!("Error Log: {}", &s );
/*

			GLchar *log = (GLchar *)malloc(logLength);
			glGetShaderInfoLog(*shader, logLength, &logLength, log);
*/				
			}

//			self.shader_ids.insert( shader_type, id );
			self.shader_ids.push( ( shader_type, id ) );
			self.shader_sources.push( ( shader_type, source.to_string() ) );
		}
		Debug::check_gl_error( std::file!(), std::line!() );
	}

	pub fn link( &mut self ) {
		unsafe {
			let id = gl::CreateProgram();
//			for s_id in self.shader_ids.values() {
			for ( _, s_id ) in self.shader_ids.iter() {
				gl::AttachShader( id, *s_id );
			}
			gl::LinkProgram( id );

			let mut status = 0;
			gl::GetProgramiv( id, gl::LINK_STATUS, &mut status );
			if status != 1 {
				println!("Warning: Failed linking shaders into program");
				for (_k, s) in self.shader_sources.iter() {
					println!("{}", &s);
				}
			}
			dbg!(&status);


			self.program_id = id;
		}
		Debug::check_gl_error( std::file!(), std::line!() );

		// lookup common uniforms
		unsafe {
			let uniform_names = [ "texture0\0" ];
			for un in uniform_names.iter() {
				let l = gl::GetUniformLocation( self.program_id, un.as_ptr() as *const _ );
				if l != -1 {
					println!("Got uniform {} at location {}", &un, l );
					self.uniforms.insert( un.to_string(), l );
				}
			}

		}
	}

	pub fn uniforms_iter( &self ) -> std::collections::hash_map::Iter<'_, String, i32> {
		self.uniforms.iter()
	}

	// :TODO: find better name, `use` is rust keyword :(
	pub fn r#use( &mut self ) {
		unsafe {
			gl::UseProgram( self.program_id );
		}
	}
}
