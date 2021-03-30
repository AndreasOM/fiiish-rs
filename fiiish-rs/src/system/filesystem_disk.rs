
use crate::system::filesystem::Filesystem;
use crate::system::filesystem_stream::FilesystemStream;
use crate::system::filesystem_stream_disk::FilesystemStreamDisk;

pub struct FilesystemDisk {
	basedir: String,
}

impl FilesystemDisk {
	pub fn new( basedir: &str ) -> Self {
		Self {
			basedir: basedir.to_string(),
		}
	}
}

impl Filesystem for FilesystemDisk {
	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream > {
		let fullname = format!("{}/{}", &self.basedir, &name);
		let mut stream = FilesystemStreamDisk::open( &fullname );

		Box::new( stream )
	}
	fn exists( &mut self, name: &str ) -> bool {
		let fullname = format!("{}/{}", &self.basedir, &name);
		std::path::Path::new(&fullname).exists()
	}

	fn name( &self ) -> &str {
		""
	}
	
	fn filesystem_type( &self ) -> &str {
		"Disk"
	}
}
