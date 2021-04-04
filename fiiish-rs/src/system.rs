
use crate::system::filesystem::Filesystem;
use crate::system::filesystem_empty::FilesystemEmpty;
use crate::system::filesystem_stream::FilesystemStream;

#[derive(Debug)]
pub struct System {
	default_filesystem: Box<dyn Filesystem>,
}

impl System {
	pub fn new() -> Self {
		Self {
			default_filesystem: Box::new( FilesystemEmpty::new() ),
		}
	}

	pub fn set_default_filesystem(&mut self, fs: Box< dyn Filesystem > ) {
		self.default_filesystem = fs;
	}

	pub fn default_filesystem_mut( &mut self ) -> &mut Box< dyn Filesystem > {
		&mut self.default_filesystem
	}
}

pub mod filesystem;
pub mod filesystem_stream;

pub mod filesystem_archive;
pub mod filesystem_stream_archive;

pub mod filesystem_disk;
pub mod filesystem_stream_disk;

pub mod filesystem_empty;
pub mod filesystem_stream_empty;

pub mod filesystem_memory;
pub mod filesystem_stream_memory;

pub mod filesystem_layered;
