
use crate::system::filesystem_stream::{ FilesystemStream, FilesystemStreamMode };


#[derive(Debug)]
enum SerializerMode {
	Read,
	Write,
}

#[derive(Debug)]
pub struct Serializer {
	file: Box< dyn FilesystemStream >,
	mode: SerializerMode,
	any_error: bool,
	byte_count: usize,
}

impl Serializer {
	pub fn new( file: Box< dyn FilesystemStream > ) -> Self {
		let mode = match file.mode() {
			FilesystemStreamMode::Read => SerializerMode::Read,
			FilesystemStreamMode::Write => SerializerMode::Write,
		};
		Self {
			file,
			mode,
			any_error: false,
			byte_count: 0,
		}
	}

	pub fn serialize_u8( &mut self, data: &mut u8 ) {
		self.byte_count += 1;
		match self.mode {
			SerializerMode::Read => {
				*data = self.file.read_u8( );

			},
			SerializerMode::Write => {
				self.file.write_u8( *data );
			},
		}
	}

	pub fn serialize_u16( &mut self, data: &mut u16 ) {
		let mut h = ( ( *data >> 8 ) & 0xff ) as u8;
		let mut l = ( ( *data >> 0 ) & 0xff ) as u8;

		self.serialize_u8( &mut h );
		self.serialize_u8( &mut l );

		*data = ( ( h as u16 ) <<8 ) | ( l as u16 );
	}

	pub fn serialize_u32( &mut self, data: &mut u32 ) {
		let mut a = ( ( *data >> 24 ) & 0xff ) as u8;
		let mut b = ( ( *data >> 16 ) & 0xff ) as u8;
		let mut c = ( ( *data >>  8 ) & 0xff ) as u8;
		let mut d = ( ( *data >>  0 ) & 0xff ) as u8;

		self.serialize_u8( &mut a );
		self.serialize_u8( &mut b );
		self.serialize_u8( &mut c );
		self.serialize_u8( &mut d );

		*data =
			  ( ( a as u32 ) << 24 )
			| ( ( b as u32 ) << 16 )
			| ( ( c as u32 ) <<  8 )
			| (   d as u32         );
	}

	pub fn serialize_bool( &mut self, value: &mut bool ) {
		let mut v: u8 = if *value {
			1
		} else {
			0
		};

		self.serialize_u8( &mut v );

		*value = if v > 0 {
			true
		} else {
			false
		}
	}

}
