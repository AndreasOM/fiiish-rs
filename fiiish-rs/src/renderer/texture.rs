

use crate::renderer::{
	Debug,
	gl,
};
use crate::system::System;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Texture {
	name: String,
	hwid: gl::types::GLuint,
}

impl Texture {
	pub fn create( system: &mut System, name: &str ) -> Self {
		let mut t = Texture::new( name );
		t.load( system, name );
		t
	}

	pub fn new( name: &str ) -> Self {
		let mut hwid = 0xffff;
		unsafe {
			gl::GenTextures( 1, &mut hwid );
		}

		Self {
			name: name.to_string(),
			hwid: hwid,
		}
	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn hwid( &self ) -> u16 {
		self.hwid as u16
	}

	pub fn bind( &self ) {
		// :TODO: support texture channels
		unsafe {
			gl::ActiveTexture( gl::TEXTURE0 );
			gl::BindTexture( gl::TEXTURE_2D, self.hwid );
		}
	}

	fn load( &mut self, system: &mut System, name: &str ) -> bool {
		let extensions = [ ".jpg", ".png" ];

		for e in extensions.iter() {
			let filename = format!("{}{}", name, e);
			let mut f = system.default_filesystem_mut().open( &filename );
			if f.is_valid() {
				println!("Loading {} from {}.", &name, &filename);
				let mut buf = Vec::new();
				while !f.eof() {
					let c = f.read_u8();
					buf.push( c );
				}
				let buf: &[u8] = buf.as_slice();
				let dummy_buf = [ 0xffffffffu32, 0x0, 0xffffffff, 0x0 ];

				match image::load_from_memory( buf ) {
					Ok( i ) => {
						let i = i.to_rgba8();
						let w = i.width();
						let h = i.height();

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

							gl::GenerateMipmap( gl::TEXTURE_2D );
/*
	glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, m_width, m_height, 0, GL_RGBA, GL_UNSIGNED_BYTE, pData);

	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

	glGenerateMipmap(GL_TEXTURE_2D);
*/							
						}
						dbg!(&h,&w);
//						dbg!(&i);
//						todo!("die");

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

}
