
use crate::math::Vector2;
use crate::math::Matrix32;

use crate::renderer::{
	Debug,
	gl,
};
use oml_game::system::System;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Texture {
	name: String,
	hwid: gl::types::GLuint,
	width: u32,
	height: u32,
	canvas: Option< Vec<u32> >,
	mtx: Matrix32,
}

impl Texture {
	pub fn create( system: &mut System, name: &str ) -> Self {
		let mut t = Texture::new( name );
		if !t.load( system, name ) {
			println!( "Warning: Failed loading texture {}", &name );
		}
		t
	}

	pub fn create_canvas( name: &str, size: u32 ) -> Self {
		let mut t = Texture::new( name );
		t.make_canvas( size );
		t.update_canvas();
		t
	}

	pub fn create_from_atlas( name: &str, mtx: &Matrix32, atlas: &Texture  ) -> Self {
		Self {
			name: name.to_string(),
			hwid: atlas.hwid() as u32,
			width: 0,	// :TODO:
			height: 0,	// :TODO:
			canvas: None,
			mtx: *mtx,
		}
	}

	pub fn new( name: &str ) -> Self {
		let mut hwid = 0xffff;
		unsafe {
			gl::GenTextures( 1, &mut hwid );
			gl::BindTexture( gl::TEXTURE_2D, hwid );

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
		}

		Self {
			name: name.to_string(),
			hwid: hwid,
			width: 0,
			height: 0,
			canvas: None,
			mtx: Matrix32::identity(),
		}
	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn hwid( &self ) -> u16 {
		self.hwid as u16
	}

	pub fn mtx( &self ) -> &Matrix32 {
		&self.mtx
	}

	pub fn bind( &self ) {
		// :TODO: support texture channels
		unsafe {
			gl::ActiveTexture( gl::TEXTURE0 );
			gl::BindTexture( gl::TEXTURE_2D, self.hwid );
		}
	}

	pub fn clear( &mut self ) {
		if let Some( c ) = &mut self.canvas {
			c.fill( 0 );
		}		
	}
	pub fn set_texel( &mut self, pos: &Vector2, color: u32 ) {
		if let Some( c ) = &mut self.canvas {
			let p = ( self.width * pos.y as u32 + pos.x as u32 ) as usize;
			if p < c.len() {
				c[ p ] = color;
			}
		}		
	}

	pub fn update_canvas( &mut self ) {
		if let Some( c ) = &self.canvas {
			unsafe {
				gl::BindTexture( gl::TEXTURE_2D, self.hwid );
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0, // mimap
					gl::RGBA8 as i32,
					self.width as i32,
					self.height as i32,
					0, // border
					gl::RGBA,
					gl::UNSIGNED_BYTE,
					//std::ptr::null(),
					// dummy_buf.as_ptr() as *const _,
					//i.into_raw().as_ptr() as *const _,
					c.as_ptr() as *const _,
				);

				Debug::check_gl_error( std::file!(), std::line!() );

//				gl::GenerateMipmap( gl::TEXTURE_2D );
			}
		}
	}
	fn make_canvas( &mut self, size: u32 ) {
		self.width = size;
		self.height = size;
		let buf_size = ( size*size ) as usize;
		let mut c = Vec::with_capacity( buf_size );
		unsafe {
			c.set_len( buf_size );
		}
		c.fill( 0x00000000 );
//		c[ 0 ] = 0xff0000ff;
		self.canvas = Some( c );
	}

	fn load( &mut self, system: &mut System, name: &str ) -> bool {
//		let extensions = [ ".jpg", ".png" ];
		let extensions = [ /*".jpg", */ ".png" ];

		for e in extensions.iter() {
			let filename = format!("{}{}", name, e);
			let mut f = system.default_filesystem_mut().open( &filename );
			if f.is_valid() {
				println!("Loading {} from {} ({}).", &name, &filename, &f.name());
				let mut buf = Vec::new();
				while !f.eof() {
					let c = f.read_u8();
					buf.push( c );
				}

				let buf: &[u8] = buf.as_slice();
//				let dummy_buf = [ 0xffffffffu32, 0x0, 0xffffffff, 0x0 ];

				match image::load_from_memory( buf ) {
					Ok( i ) => {
						let i = i.to_rgba8();
						let w = i.width();
						let h = i.height();

						self.width = w;
						self.height = h;

						unsafe {
							gl::BindTexture( gl::TEXTURE_2D, self.hwid );
							gl::TexImage2D(
								gl::TEXTURE_2D,
								0, // mimap
								gl::RGBA8 as i32,
								w as i32,
								h as i32,
								0, // border
								gl::RGBA,
								gl::UNSIGNED_BYTE,
								//std::ptr::null(),
								// dummy_buf.as_ptr() as *const _,
								i.into_raw().as_ptr() as *const _,
							);

							Debug::check_gl_error( std::file!(), std::line!() );

//							gl::GenerateMipmap( gl::TEXTURE_2D );
						}

						return true;

					},
					Err( e ) => {
						println!("Error: '{}' loading image from {}", &e, &filename );
					},
				}

			}
		}

		false
	}

	pub fn width( &self ) -> u32 {
		self.width
	}

	pub fn height( &self ) -> u32 {
		self.height
	}

}
