

use crate::system::filesystem_stream::FilesystemStream;

#[derive(Debug)]
pub struct FilesystemStreamArchive {
	crc: u32,
	name: String,
	data: Vec< u8 >,
	pos: usize,
	valid: bool,
}

impl FilesystemStreamArchive {
	pub fn open( crc: u32, data: &Vec< u8 > ) -> Self {
		Self {
			crc: crc,
			name: format!( "0x{:08X}", crc ),
			data: data.clone(), // :TODO: optimize out copy
			pos: 0,
			valid: true,
		}
	}

	pub fn mark_invalid( &mut self ) {
		self.valid = false;
	}
}

impl FilesystemStream for FilesystemStreamArchive {
	fn size( &self ) -> usize {
		self.data.len()
	}
	fn pos( &self ) -> usize {
		self.pos
	}
	fn set_pos( &mut self, pos: usize ) {
		self.pos = pos;
	}
	fn read_u8( &mut self ) -> u8 {
		match self.data.get( self.pos ) {
			None => {
				println!( "Warning: Failed to get u8 for {}", &self.name );
				0
			},
			Some( b ) => {
				self.pos += 1;
				*b
			},
		}
	}
	fn is_valid( &self ) -> bool {
		self.valid
	}
	fn eof( &self ) -> bool {
		self.pos >= self.data.len()
	}
	fn name( &self ) -> &str {
		&self.name
	}
	fn filesystem_stream_type( &self ) -> &str {
		"Archive"
	}

}
