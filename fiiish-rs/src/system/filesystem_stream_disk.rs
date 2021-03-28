
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

use crate::system::filesystem_stream::FilesystemStream;

pub struct FilesystemStreamDisk {
	filename: String,	// only needed for better debugging
	file: Option< File >,
	size: usize,
	pos: usize,
}

impl FilesystemStreamDisk {
	pub fn open( filename: &str ) -> Self {
		let mut s = Self {
			filename: filename.to_string(),
			file: None,
			size: 0,
			pos: 0,
		};

		if let Ok( mut f ) = File::open( &s.filename ) {
			if let Ok( p ) =  f.seek(SeekFrom::End(0)) {
				f.seek( SeekFrom::Start( 0 ) );
				s.size = p as usize
			} else {
			}
			s.file = Some( f );
		};


		s
	}
}

impl FilesystemStream for FilesystemStreamDisk {
	fn size( &self ) -> usize {
		self.size
	}
	fn pos( &self ) -> usize {
		self.pos
	}
	fn set_pos( &mut self, pos: usize ) {
		match &mut self.file {
			Some( f ) => {
				f.seek( SeekFrom::Start( pos as u64 ) );
				if let Ok( p ) = f.stream_position() {
					self.pos = p as usize;
				} else {

				}
			},
			None => {},
		}
	}
	fn read_u8( &mut self ) -> u8 {

		match &mut self.file {
			Some( f ) => {
				let mut buf = [0];
				match f.read( &mut buf ) {
					Ok( _ ) => {
						self.pos += 1;
						buf[ 0 ]
					},
					Err( _ ) => 0,
				}

			},
			None => {
				0
			},
		}

	}
	fn is_valid( &self ) -> bool {
		self.file.is_some()
	}
	fn eof( &self ) -> bool {
		self.pos >= self.size
	}
	fn name( &self ) -> &str {
		&self.filename
	}
	fn filesystem_stream_type( &self ) -> &str {
		"Disk"
	}

}