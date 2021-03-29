// Depreacted, for now

todo!("remove");

use crate::renderer::Material;
use crate::system::System;

pub struct MaterialBuilder {
	name: String,
//	vertex_shader: String,
//	fragment_shader: String,
}

impl MaterialBuilder {
	pub fn new() -> Self {
		Self {
			name: String::new(),
//			vertex_shader: String::new(),
//			fragment_shader: String::new(),
		}
	}

	pub fn with_name( mut self, name: &str ) -> Self {
		self.name = name.to_string();
		self
	}
/*
	pub fn with_vertex_shader( mut self, name: &str ) -> Self {
		self.vertex_shader = name.to_string();
		self
	}

	pub fn with_fragment_shader( mut self, name: &str ) -> Self {
		self.fragment_shader = name.to_string();
		self
	}
*/
	pub fn build_with_system( &self, system: &mut System ) -> Material {
		let mut m = Material::new();
/*
		let mut vsf = system.default_filesystem_mut().open( &self.vertex_shader );
		let vs = vsf.read_as_string();

		let mut fsf = system.default_filesystem_mut().open( &self.fragment_shader );
		let fs = fsf.read_as_string();

		m.add_vertex_shader( &vs );
		m.add_fragment_shader( &fs );
		m.link();
*/
		m
	}
}