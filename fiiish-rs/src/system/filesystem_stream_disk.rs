
use std::fs::{ File, OpenOptions };
use std::io::{BufReader,SeekFrom};
use std::io::prelude::*;

use crate::system::filesystem_stream::{ FilesystemStream, FilesystemStreamMode };

pub struct FilesystemStreamDisk {
	filename: String,	// only needed for better debugging
	file: Option< BufReader< File > >,
	file_write: Option< File >,
//	file: Option< File >,
	size: usize,
	pos: usize,
	mode: FilesystemStreamMode,
}

impl FilesystemStreamDisk {
	pub fn open( filename: &str ) -> Self {
		let mut s = Self {
			filename: filename.to_string(),
			file: None,
			file_write: None,
			size: 0,
			pos: 0,
			mode: FilesystemStreamMode::Read,
		};

		if let Ok( mut f ) = File::open( &s.filename ) {
			if let Ok( p ) =  f.seek(SeekFrom::End(0)) {
				f.seek( SeekFrom::Start( 0 ) ).unwrap();
				s.size = p as usize
			} else {
			}
			let f = BufReader::new(f);

			s.file = Some( f );
		};


		s
	}

	pub fn create( filename: &str, overwrite: bool ) -> Self {
		let mut s = Self {
			filename: filename.to_string(),
			file: None,
			file_write: None,
			size: 0,
			pos: 0,
			mode: FilesystemStreamMode::Write,
		};

		let file = OpenOptions::new()
					.write( true )
					.truncate( true )
					.create( overwrite )
					.create_new( !overwrite )
					.open( &s.filename );
//		dbg!(&file);
		match file {
			Ok( mut f ) => {
				s.size = 0;
				/*
				let f = BufReader::new(f);
				*/
				s.file_write = Some( f );
			},
			Err( e ) => {
				println!("Error creating >{}< -> {:?}", &filename, &e );
			}
		}

		s
	}
}

impl Drop for FilesystemStreamDisk {
    fn drop(&mut self) {
    	if let Some( f ) = &mut self.file_write {
    		f.sync_all().unwrap();
    		self.file_write = None;
    	}
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
				f.seek( SeekFrom::Start( pos as u64 ) ).unwrap();
				if let Ok( p ) = f.stream_position() {
					self.pos = p as usize;
				} else {

				}
			},
			None => {},
		}
	}

	fn mode( &self ) -> FilesystemStreamMode {
		self.mode
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
	fn write_u8( &mut self, data: u8 ) {
		match &mut self.file_write {
			Some( f ) => {
				let mut buf = [ data ];
				match f.write( &mut buf ) {
					Ok( _ ) => {
						self.pos += 1;
					},
					Err( _ ) => {},
				}

			},
			None => {
			},
		}
	}

	fn is_valid( &self ) -> bool {
		match self.mode {
			FilesystemStreamMode::Read => {
				self.file.is_some()
			},			
			FilesystemStreamMode::Write => {
				self.file_write.is_some()
			},
		}
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
