
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum FilesystemStreamMode {
	Read,
	Write
}

pub trait FilesystemStream {
	fn size( &self, ) -> usize;
	fn pos( &self,  ) -> usize;
	fn set_pos( &mut self, pos: usize );
	fn read_u8( &mut self ) -> u8;
	fn write_u8( &mut self, data: u8 ){}
	fn is_valid( &self ) -> bool;
	fn eof( &self ) -> bool;
	fn name( &self ) -> &str;
	fn filesystem_stream_type( &self ) -> &str;

	fn mode( &self ) -> FilesystemStreamMode {
		FilesystemStreamMode::Read
	}

	fn read_as_string( &mut self ) -> String {
		let mut s = String::new();
		while !self.eof() {
			let c = self.read_u8() as char;
			s.push( c );
		}
		s
	}

	fn read_as_fixed_string(&mut self, size: u16) -> String {
		let mut s = String::new();
		for _n in 0..size {
			let c = self.read_u8() as char;
			s.push( c );
		}
		s
	}

	fn read_u16( &mut self ) -> u16 {
		let a = self.read_u8() as u16;
		let b = self.read_u8() as u16;

		  ( b << 8 )
		| ( a << 0 )
	}

	fn read_u32( &mut self ) -> u32 {
		let a = self.read_u8() as u32;
		let b = self.read_u8() as u32;
		let c = self.read_u8() as u32;
		let d = self.read_u8() as u32;

		  ( d << 24 )
		| ( c << 16 )
		| ( b <<  8 )
		| ( a <<  0 )
	}

	fn read_f32( &mut self ) -> f32 {
		let f = self.read_u32();

		unsafe { std::mem::transmute( f ) }
	}

}


impl std::fmt::Debug for dyn FilesystemStream {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!(
			f, "[Trait] FilesystemStream: {} [{}] {}",
			self.name(),
			self.filesystem_stream_type(),
			if self.is_valid() {
				"[VALID]"
			} else {
				"[INVALID]"
			}
		)
	}
}
