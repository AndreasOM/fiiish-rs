
use crate::system::filesystem::Filesystem;
use crate::system::filesystem_stream::FilesystemStream;
use crate::system::filesystem_stream_memory::FilesystemStreamMemory;

pub struct FilesystemMemory {

}

impl FilesystemMemory {
	pub fn new() -> Self {
		Self {

		}
	}
	pub fn open_from_data( &mut self, name: &str, data: &Vec< u8 > ) -> Box< dyn FilesystemStream > {
		let stream = FilesystemStreamMemory::open( name, data );

		Box::new( stream )
	}
}

impl Filesystem for FilesystemMemory {
	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream > {
		let stream = FilesystemStreamMemory::open( name, &Vec::new() );

		Box::new( stream )
	}

	fn name( &self ) -> &str {
		""
	}
	
	fn filesystem_type( &self ) -> &str {
		"Memory"
	}
}
