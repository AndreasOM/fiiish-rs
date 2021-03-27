
use crate::window::Window;

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

#[derive(Debug)]
pub struct Renderer {

}

impl Renderer {
	pub fn new() -> Self {
		Self {

		}
	}

	pub fn setup( &mut self, window: &Window ) -> anyhow::Result<()> {
		gl::load_with(|s| window.get_proc_address(s) as *const _); // :TODO: maybe use CFBundleGetFunctionPointerForName directly
		Ok(())
	}

	pub fn teardown( &mut self ) {

	}

	pub fn begin_frame( &mut self ) {

	}

	pub fn end_frame( &mut self ) {
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
}

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
