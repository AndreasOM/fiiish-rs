
use std::collections::HashMap;

//use crate::renderer::Program;
use crate::renderer::{
	Debug,
	gl,
};

#[derive(Debug,PartialEq,Eq,Hash)]
pub enum ShaderType {
	Vertex,
	Fragment,
}

#[derive(Debug)]
pub struct Program {

//	shader_ids: HashMap<ShaderType, gl::types::GLuint>,
	shader_ids: Vec< (ShaderType, gl::types::GLuint) >,
	program_id: gl::types::GLuint,
}

impl Program {
	pub fn new() -> Self {
		Self {
//			shader_ids: HashMap::new(),
			shader_ids: Vec::new(),
			program_id: 0xffffffff,
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
			}

//			self.shader_ids.insert( shader_type, id );
			self.shader_ids.push( ( shader_type, id ) );
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
				println!("Warning: Failed linking shaders into program")
			}
			dbg!(&status);


			self.program_id = id;
		}
		Debug::check_gl_error( std::file!(), std::line!() );
	}

	// :TODO: find better name, `use` is rust keyword :(
	pub fn r#use( &mut self ) {
		unsafe {
			gl::UseProgram( self.program_id );
		}
	}
}
