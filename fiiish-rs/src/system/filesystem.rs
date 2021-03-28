
use crate::system::filesystem_stream::FilesystemStream;


pub trait Filesystem {

	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream >;

	fn name( &self ) -> &str;
	fn filesystem_type( &self ) -> &str;
}

impl std::fmt::Debug for dyn Filesystem {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!( f,"[Trait] Filesystem: {} [{}]", self.name(), self.filesystem_type() )
	}
}
