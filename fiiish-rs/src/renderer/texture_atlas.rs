
use regex::Regex;

use crate::math::Matrix32;

use crate::renderer::{
	Renderer,
	Texture,
};
use crate::system::System;
use crate::system::filesystem_stream::FilesystemStream;

#[derive(Debug)]
pub struct Entry {
	name: String,
	mtx: Matrix32,
}

#[derive(Debug)]
pub struct TextureAtlas {
	entries: Vec< Entry >,
}

impl Entry {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			mtx: Matrix32::identity(),
		}
	}

	pub fn load( &mut self, f: &mut Box< dyn FilesystemStream > ) -> bool {
		let mut name_buffer = Vec::with_capacity( 128 );
		for _ in 0..128 {
			let b = f.read_u8();
			name_buffer.push( b );
		}

		let mut name = String::from_utf8( name_buffer ).unwrap();
		let first_zero = name.find( "\u{0}" ).unwrap_or( name.len() );
		name.truncate( first_zero );

		let mut matrix_buffer = [0f32;6];
		for m in &mut matrix_buffer {
			*m = f.read_f32();
		}

		self.name = name;
		self.mtx = matrix_buffer.into();
		true
	}
}
// :TODO: move somewhere more sane
fn simple_format_u32( f: &str, n: u32 ) -> String {
	let s = f.clone();
	let re = Regex::new(r"(%d)").unwrap();

//	println!("simple_format_u32 {:?} with {:?}", s, re );
	let s = re.replace_all(
		&s,
		|c: &regex::Captures| {
			let placeholder = c.get(1).map_or( "", |m| m.as_str() );
//			println!("Found {:?}", placeholder );
			match placeholder {
				"" => "".to_string(),
				"%d" => n.to_string(),
				x => {
					println!("simple_format_u32 got {:?}", x);
					x.to_string()
				},
			}
		}
	);

	s.to_string()
}

impl TextureAtlas {
	pub fn load_all( system: &mut System, renderer: &mut Renderer, template: &str ) -> usize {
		let fs = system.default_filesystem_mut();

		let mut to_load = Vec::new();
		let mut i = 0;
		loop {
			let name = simple_format_u32( template, i );
			let name_atlas = format!("{}.atlas", &name);
			let name_png = format!("{}.png", &name);

			if fs.exists( &name_atlas ) && fs.exists( &name_png ){
				to_load.push( name.to_owned() );
			} else {
				break;
			}
			i += 1;
		}

//		dbg!(&to_load);


		let mut total_textures_registered = 0;
		for name in to_load.iter() {
			let name_atlas = format!("{}.atlas", &name);
			let t = Texture::create( system, &name );

			let mut ta = TextureAtlas::new();
			ta.load( system, &name_atlas );

			for e in ta.entries.iter() {
				let mut name_wo_ext = e.name.clone();
				// :TODO: last dot might be better ;)
				let first_dot = name_wo_ext.find( "." ).unwrap_or( name_wo_ext.len() );
				name_wo_ext.truncate( first_dot );

				let te = Texture::create_from_atlas( &name_wo_ext, &e.mtx, &t );
//				println!("Registering atlas (sub) texture '{}' with renderer {:?}", &name_wo_ext, &te);
				renderer.register_texture( te );
				total_textures_registered += 1;
			}

			renderer.register_texture( t );
		}
		total_textures_registered
	}

	pub fn new() -> Self {
		Self {
			entries: Vec::new(),
		}
	}
	pub fn load( &mut self, system: &mut System, name: &str ) -> bool {
		let mut f = system.default_filesystem_mut().open( &name );
		if !f.is_valid() {
			return false;
		}

		println!("Loading atlas from {}", &name );

		let magic = f.read_u16();
		if magic != 0x4f53 {
			println!("Got broken magic expected 0x4f53 got {:X}", magic);
			return false
		}
		let v = f.read_u16();
		if v != 1 {
			println!("Version {} not supported", v);
			return false;
		}
		let chunk_magic = [ 0x4fu8, 0x4d, 0x41, 0x54, 0x4c, 0x41, 0x53, ];
		for m in &chunk_magic {
			let b = f.read_u8();
			if b != *m {
				println!("Got broken chunk magic expected {:X} got {:X}", m, b);
				return false;
			}
		}

		let flags = f.read_u8();
		if flags != 'S' as u8 {
			println!("Compression/flags '{}' not supported.", flags);
			return false;
		}

		let chunk_version = [ 0x01u8, 0x00, 0x00, 0x00 ];
		for m in &chunk_version {
			let b = f.read_u8();
			if b != *m {
				println!("Broken chunk version");
				return false;
			}
		}		

		let entry_count = f.read_u16();

		println!("Got {:?} entries", entry_count );

		for i in 0..entry_count {
			let mut e = Entry::new();
			if !e.load( &mut f ) {
				println!("Load of entry {}/{} failed.", i, entry_count);
				return false;
			}

			self.entries.push( e );
		}

//		dbg!(&self);

		true

	}

}
