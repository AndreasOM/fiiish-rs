

use crate::renderer::{
//	Debug,
//	gl,
	Program,
	ShaderType,
};
use crate::system::System;


#[derive(Debug)]
pub struct Effect {
	id: u16,
	name: String,
	program: Program,
}

impl Effect {
	pub fn create(
		system: &mut System,
		id: u16,
		name: &str,
		vertex_shader_name: &str,
		fragment_shader_name: &str,
	) -> Self {
		Effect::new(
			system,
			id,
			name,
			vertex_shader_name,
			fragment_shader_name,
		)
	}
	fn new(
		system: &mut System,
		id: u16,
		name: &str,
		vertex_shader_name: &str,
		fragment_shader_name: &str,		
	) -> Self {
		let mut program = Program::new();

		let mut vsf = system.default_filesystem_mut().open( &vertex_shader_name );
		let vs = vsf.read_as_string();

		let mut fsf = system.default_filesystem_mut().open( &fragment_shader_name );
		let fs = fsf.read_as_string();

		program.add_shader( ShaderType::Vertex, &vs );
		program.add_shader( ShaderType::Fragment, &fs );
		program.link();

		Self {
			id,
			name: name.to_string(),
			program,
		}
	}

	pub fn id( &self ) -> u16 {
		self.id
	}

	pub fn r#use( &mut self ) {
		self.program.r#use();
	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn program(&self) -> &Program { 
		&self.program
	}
}
