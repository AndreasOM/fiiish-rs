
use crate::system::filesystem::Filesystem;
use crate::system::filesystem_stream::FilesystemStream;
use crate::system::filesystem_stream_disk::FilesystemStreamDisk;

pub struct FilesystemDisk {
	basedir: String,
	writable: bool,
}

impl FilesystemDisk {
	pub fn new( basedir: &str ) -> Self {
		Self {
			basedir: basedir.to_string(),
			writable: false,
		}
	}

	pub fn enable_write( &mut self ) {
		self.writable = true;
	}
}

impl Filesystem for FilesystemDisk {
	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream > {
		let fullname = format!("{}/{}", &self.basedir, &name);
		let stream = FilesystemStreamDisk::open( &fullname );

		Box::new( stream )
	}
	fn create( &mut self, name: &str, overwrite: bool ) -> Box< dyn FilesystemStream > {
		let fullname = format!("{}/{}", &self.basedir, &name);
		let stream = FilesystemStreamDisk::create( &fullname, overwrite );

		Box::new( stream )
	}

	fn exists( &mut self, name: &str ) -> bool {
		let fullname = format!("{}/{}", &self.basedir, &name);
		std::path::Path::new(&fullname).exists()
	}

	fn writable( &self ) -> bool {
		self.writable
	}

	fn name( &self ) -> &str {
		""
	}
	
	fn filesystem_type( &self ) -> &str {
		"Disk"
	}

	fn format( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!( f,"Filesystem: {} [{}] -> {}", self.name(), self.filesystem_type(), &self.basedir )
	}	
}

impl std::fmt::Debug for FilesystemDisk {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		self.format( f )
	}
}

