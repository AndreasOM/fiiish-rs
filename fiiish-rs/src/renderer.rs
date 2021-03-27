
use crate::window::Window;

use material::Material;

#[derive(Debug)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {
	pub fn from_rgba( r: f32, g: f32, b: f32, a: f32 ) -> Self {
		Self {
			r,
			g,
			b,
			a,
		}
	}
}

#[derive(Debug,Copy,Clone)]
pub struct Vertex {
	pos: [f32;3],
}

impl Vertex {
	pub fn from_xyz( x: f32, y: f32, z: f32 ) -> Self {
		Self {
			pos: [ x, y, z ],
		}
	}
}

#[derive(Debug)]
pub struct Renderer {
	materials: Vec<Material>,
	vertices: Vec<Vertex>,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			materials: Vec::new(),
			vertices: Vec::new(),		// :TODO: pre allocate size? or maybe even a fixed size array
		}
	}

	pub fn setup( &mut self, window: &Window ) -> anyhow::Result<()> {
		gl::load_with(|s| window.get_proc_address(s) as *const _); // :TODO: maybe use CFBundleGetFunctionPointerForName directly
		// :HACK: create one material
		self.materials.push( Material::new() );
		unsafe {
			let s = gl::GetString( gl::VERSION );
			let s = String::from_utf8( std::ffi::CStr::from_ptr( s as *const _ ).to_bytes().to_vec() )?;
			println!("GL Version: {}", s );
		}
		Ok(())
	}

	pub fn teardown( &mut self ) {

	}

	pub fn begin_frame( &mut self ) {
		self.vertices.clear();
		for material in self.materials.iter_mut() {
			material.clear();
		}
	}

	pub fn end_frame( &mut self ) {
		// just to avoid ghost
		unsafe {
//			gl::Disable(gl::CULL_FACE);
			gl::Enable(gl::CULL_FACE);
			gl::Disable(gl::DEPTH_TEST);
		}

		// :TODO: fix rendering order
		for material in self.materials.iter_mut() {
			material.render();
		}

		// glFlush or glFinish
		unsafe {
			gl::Flush();
		}
	}

	// rendering functions

	pub fn clear( &mut self, color: &Color ) {
//		println!("clear with {:?}", &color );
		// glClearColor and glClear
		unsafe {
			gl::ClearColor( color.r, color.g, color.b, color.a );
			gl::Clear( gl::COLOR_BUFFER_BIT ); // :TODO: clear other buffers?
		}
	}

	pub fn add_vertex( &mut self, x: f32, y: f32 ) -> u32 {
		let v = Vertex::from_xyz( x, y, 0.0 );
		self.vertices.push( v );
		self.vertices.len() as u32 - 1
	}

	pub fn add_triangle( &mut self, v0: u32, v1: u32, v2: u32 ) {
		match self.materials.get_mut( 0 ) { // 0 == active material
			Some( material ) => {
				for v in [v0, v1, v2].iter() {
					match self.vertices.get( *v as usize ) {
						Some( v ) => {
							material.add_vertex( v );
						},
						None => {
							// :TODO: shout loud
						},
					}
				}				
			},
			None => {},
		}
	}
}

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod material;
