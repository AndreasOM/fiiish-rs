
//use std::fs::File;
use std::rc::Rc;
//use std::io::prelude::*;
use std::collections::HashMap;

use crate::system::filesystem::Filesystem;
use crate::system::filesystem_stream::FilesystemStream;
use crate::system::filesystem_stream_archive::FilesystemStreamArchive;

pub struct Entry {
	pub crc: u32,
	pub pos: u32,
	pub size: u32,
	pub data: Vec< u8 >,
}

impl Entry {
	pub fn new() -> Self {
		Self {
			crc: 0,
			pos: 0,
			size: 0,
			data: Vec::new(),
		}
	}

	pub fn initialize_from_file( &mut self, file: &mut Box< dyn FilesystemStream > ) -> bool {
		self.crc = file.read_u32();
		self.pos = file.read_u32();
		self.size = file.read_u32();

		true
	}

	pub fn load_from_file( &mut self, file: &mut Box< dyn FilesystemStream > ) -> bool {
		for _n in 0..self.size {
			let b = file.read_u8();
			self.data.push( b );
		}

		true
	}

}
pub struct FilesystemArchive {
	name: String,
	entries: HashMap<u32, Entry>,
	data: Rc< Vec< u8 > >,
}

impl FilesystemArchive {
	// :TODO: optimize out extra copying of data
	pub fn new_from_file( name: &str, file: &mut Box< dyn FilesystemStream > ) -> Self {
		let mut s = Self {
			name: name.to_owned(),
			entries: HashMap::new(),
			data: Rc::new( Vec::new() ),
		};

		s.initialize_from_file( file );

		s
	}

	fn initialize_from_file( &mut self, file: &mut Box< dyn FilesystemStream > ) -> bool {
		let magic = [ 0x4fu8, 0x4d, 0x41, 0x52 ];
		for m in &magic {
			let b = file.read_u8();
			if b != *m {
				println!("Warning: Broken magic for .omar. Got {:X} expected {:X}.", b, m);
				return false;
			}
		}

		let v = file.read_u8();
		if v != 2 {
			println!("Warning:Wrong version" );
			return false;
		}

		let flags = file.read_u8();
		if flags != 0 {
			println!("Warning: :TODO: Flags not implemented" );
			return false;
		}

		for _reserved in 0..2 {
			let r = file.read_u8();
			if r != 0 {
				println!("Warning: :TODO: Reserved field not zero" );
				return false;
			}
		}

		let number_of_files = file.read_u32();
		println!("Reading {:?} files from archive {}", number_of_files, &self.name );

		for _e in 0..number_of_files {
			let mut entry = Entry::new();
			if !entry.initialize_from_file( file ) {
				println!("Warning: Failed to read entry from file");
				return false;
			}
			self.entries.insert( entry.crc, entry );
		}
//		let l = self.entries.len();
		let data_start = file.pos();
		for ( _i, entry ) in self.entries.values_mut().enumerate() {
//			println!("{}/{}", i, l);
			// adjust file position
			let entry_start = data_start + entry.pos as usize;
			file.set_pos( entry_start );
			(*entry).load_from_file( file );
		}
		true
	}

	fn calc_crc_for_name( name: &str ) -> u32 {
		// :DANGER: this if for ruby converter compatibility

		// :TODO: calculate actual CRC name
		let downcase_name = name.to_lowercase();
		// Ruby: .gsub( /\W\./, ' ' ) // should be 'a-zA-Z0-9_', but actual code behaves differently
		let clean_name: String = downcase_name.chars().map(|c| match c {
			'0'..='9' => c,
			'a'..='z' => c,
	//			'A'..='Z' => c,	// already downcase
			'!'..='@' => c,
			'['..='`' => c,
			'{'..='~' => c,
	//		0x7f => c,			// ignore DEL
			_ => ' '
		}).collect();

		const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
		let crc = CRC32.checksum( clean_name.as_bytes() );

//		println!("CRC: {:?} -> {:?} crc: {:?} {:#10X}\n", name, clean_name, crc, crc );
//		panic!("CRC");

		crc
	}
}

impl Filesystem for FilesystemArchive {
	fn open( &mut self, name: &str ) -> Box< dyn FilesystemStream > {
		let crc = FilesystemArchive::calc_crc_for_name( name );
		match self.entries.get( &crc ) {
			None => {
				let mut stream = FilesystemStreamArchive::open( crc, &Vec::new() );
				stream.mark_invalid();
				Box::new( stream )				
			},
			Some( entry ) => {
				let stream = FilesystemStreamArchive::open( entry.crc, &entry.data );
				Box::new( stream )
			},
		}
	}

	fn exists( &self, name: &str ) -> bool {
		let crc = FilesystemArchive::calc_crc_for_name( name );
		if self.entries.contains_key( &crc ) {
			true
		} else {
//			dbg!(crc, &self.entries.contains_key( &crc ));
//			dbg!(&self.entries);
			false
		}
	}

	fn name( &self ) -> &str {
		&self.name
	}
	
	fn filesystem_type( &self ) -> &str {
		"Archive"
	}
}

impl std::fmt::Debug for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!( f, "0x{:08X} {} {} {}", self.crc, self.pos, self.size, self.data.len() )
	}
}

impl std::fmt::Debug for FilesystemArchive {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!( f, "FilesystemArchive:\n\tsize: {}", self.data.len() ).unwrap();

		for ( k, e ) in self.entries.iter() {
			writeln!( f, "\t0x{:08X}\t{:?}", k, &e ).unwrap();
		}

		writeln!(f, "")
	}
}
