
use crate::system::filesystem::Filesystem;
use crate::system::filesystem_stream::FilesystemStream;
use crate::system::filesystem_stream_empty::FilesystemStreamEmpty;

pub struct FilesystemEmpty {

}

impl FilesystemEmpty {
	pub fn new() -> Self {
		Self {

		}
	}
}

impl Filesystem for FilesystemEmpty {
	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream > {
		let stream = FilesystemStreamEmpty::open( name );

		Box::new( stream )
	}

	fn name( &self ) -> &str {
		""
	}
	
	fn filesystem_type( &self ) -> &str {
		"Empty"
	}
}
