

use crate::system::filesystem_stream::FilesystemStream;

pub struct FilesystemStreamMemory {
	name: String,
	data: Vec< u8 >,
	pos: usize,
}

impl FilesystemStreamMemory {
	pub fn open( name: &str, data: &Vec< u8 > ) -> Self {
		Self {
			name: name.to_owned(),
			data: data.clone(), // :TODO: optimize out copy
			pos: 0,
		}
	}
}

impl FilesystemStream for FilesystemStreamMemory {
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
				println!( "Warning: Failed to get u8 for MemoryStream" );
				0
			},
			Some( b ) => {
				self.pos += 1;
				*b
			},
		}
	}
	fn is_valid( &self ) -> bool {
		true
	}
	fn eof( &self ) -> bool {
		self.pos >= self.data.len()
	}
	fn name( &self ) -> &str {
		&self.name
	}
	fn filesystem_stream_type( &self ) -> &str {
		"Memory"
	}

}
