
use crate::math::Vector2;
use crate::system::System;
use crate::system::filesystem_stream::FilesystemStream;

#[derive(Debug)]
pub struct Object {
	pub id: u16,
	pub crc: u32,
	pub pos: Vector2,
	pub rotation: f32,
}

#[derive(Debug)]
pub struct Layer {
	name: String,
	objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Zone {
	name: String,
	difficulty: u16,
	size: Vector2,
	layers: Vec< Layer >,
}

impl Object {
	pub fn new() -> Self {
		Self {
			id: 0,
			crc: 0xffffffff,
			pos: Vector2::zero(),
			rotation: 0.0,
		}
	}

	pub fn load(&mut self, f: &mut Box< dyn FilesystemStream > ) -> bool {
		self.id = f.read_u16();
		self.crc = f.read_u32();
		self.pos.x = f.read_f32();
		self.pos.y = f.read_f32();
		self.rotation = f.read_f32();
		true
	}
}

impl Layer {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			objects: Vec::new(),
		}
	}
	// impl Iterator<Item = T>

	pub fn object_iter( &self ) -> std::slice::Iter<'_, Object> {
		self.objects.iter()
	}

	pub fn load(&mut self, f: &mut Box< dyn FilesystemStream > ) -> bool {
		self.name = f.read_as_fixed_string( 16 );
		let first_zero = self.name.find( "\u{0}" ).unwrap_or( self.name.len() );
		self.name.truncate( first_zero );

		let object_count = f.read_u16();

		for _o in 0..object_count {
			let mut object = Object::new();
			object.load( f );
			self.objects.push( object );
		}
		true
	}	
}

impl Zone {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			difficulty: 0,
			size: Vector2::zero(),
			layers: Vec::new(),
		}
	}

	pub fn layer_iter( &self ) -> std::slice::Iter<'_, Layer> {
		self.layers.iter()
	}

	pub fn size( &self ) -> &Vector2 {
		&self.size
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> bool {
		let filename = format!("{}.nzne", name);
		let mut f = system.default_filesystem_mut().open( &filename );
		if !f.is_valid() {
			println!("Error: Couldn't open zone {}", &filename);
			return false;
		}

		// 53 4f 01 00 46 49 53 48  4e 5a 4e 45 02 00 00 00

		let so_magic = f.read_u16();
		if so_magic != 0x4f53 {
			println!("Got broken magic expected 0x4f53 got {:X}", so_magic);
			return false
		}

		let so_version = f.read_u16();
		if so_version != 1 {
			println!("Version {} not supported", so_version);
			return false;
		}

		let chunk_magic = [ 0x46u8, 0x49, 0x53, 0x48, 0x4e, 0x5a, 0x4e, ];
		for m in &chunk_magic {
			let b = f.read_u8();
			if b != *m {
				println!("Got broken chunk magic expected {:X} got {:X}", m, b);
				return false;
			}
		}

		let flags = f.read_u8();
		if flags != 'E' as u8 {
			println!("Compression/flags '{}' not supported.", flags);
			return false;
		}

		let version = f.read_u32();
		if version != 2 {
			println!("Version {} not supported for zone", version);
		}

		self.name = f.read_as_fixed_string( 64 );
		let first_zero = self.name.find( "\u{0}" ).unwrap_or( self.name.len() );
		self.name.truncate( first_zero );
		self.difficulty = f.read_u16();
		self.size.x = f.read_f32();
		self.size.y = f.read_f32();

		println!("Reading version {:X} zone {}. Difficulty {}, Size {}x{}",&version, &self.name, self.difficulty, self.size.x, self.size.y);

		let layer_count = f.read_u16();

		for _l in 0..layer_count {
			let mut layer = Layer::new();
			layer.load( &mut f );
			self.layers.push( layer );
		}

		true
	}
}
