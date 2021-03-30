
use crate::system::filesystem::Filesystem;

use crate::system::filesystem_stream::FilesystemStream;

pub struct FilesystemLayered {
	filesystems: Vec< Box< dyn Filesystem > >,
}

impl FilesystemLayered {
	pub fn new( ) -> Self {
		Self {
			filesystems: Vec::new(),
		}
	}

	pub fn add_filesystem( &mut self, filesystem: Box< dyn Filesystem > ) {
		self.filesystems.push( filesystem );
	}
}

impl Filesystem for FilesystemLayered {
	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream > {
		for fs in self.filesystems.iter_mut().rev() {
			if fs.exists( name ) {
				return fs.open( name );
			}
		}

		if let Some( fs ) = self.filesystems.get_mut( 0 ) {
			fs.open( name )
		} else {
			panic!( "Error: FilesystemLayered tried to open file without any filesystem" );
		}
	}

	fn name( &self ) -> &str {
		""
	}
	
	fn filesystem_type( &self ) -> &str {
		"Layered"
	}
}
