

use crate::system::filesystem_stream::FilesystemStream;

pub struct FilesystemStreamEmpty {
	name: String,
}

impl FilesystemStreamEmpty {
	pub fn open( name: &str ) -> Self {
		Self {
			name: name.to_owned(),
		}
	}
}

impl FilesystemStream for FilesystemStreamEmpty {
	fn size( &self ) -> usize {
		0
	}
	fn pos( &self ) -> usize {
		0
	}
	fn set_pos( &mut self, _pos: usize ) {

	}
	fn read_u8( &mut self ) -> u8 {
		0
	}
	fn is_valid( &self ) -> bool {
		true
	}
	fn eof( &self ) -> bool {
		true
	}
	fn name( &self ) -> &str {
		&self.name
	}
	fn filesystem_stream_type( &self ) -> &str {
		"Empty"
	}

}
