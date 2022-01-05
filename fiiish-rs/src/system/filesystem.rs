
use crate::system::filesystem_stream::FilesystemStream;

use crate::system::filesystem_stream_empty::FilesystemStreamEmpty;

pub trait Filesystem {

	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream >;
	fn create( &mut self, name: &str, overwrite: bool ) -> Box< dyn FilesystemStream > {
		let stream = FilesystemStreamEmpty::open( name );

		Box::new( stream )		
	}
	fn exists( &self, _name: &str ) -> bool {
		false
	}

	fn writable( &self ) -> bool {
		false
	}

	fn name( &self ) -> &str;
	fn filesystem_type( &self ) -> &str;

	fn format( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!( f,"[Trait] Filesystem: {} [{}]", self.name(), self.filesystem_type() )
	}
}

impl std::fmt::Debug for dyn Filesystem {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		self.format( f )
	}
}
