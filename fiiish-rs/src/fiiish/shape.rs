
use std::cell::RefCell;
use crate::DebugRenderer;

use oml_game::math::Vector2;
use oml_game::math::Matrix22;

use crate::system::System;
//use crate::system::filesystem_stream::FilesystemStream;

use crate::renderer::Color;

#[derive(Debug)]
pub struct SubShape {	
	pub vertices: Vec< Vector2 >,
}

impl SubShape {
	pub fn new() -> Self {
		Self {
			vertices: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub struct Shape {
	name: String,
	offset: Vector2,
	sub_shapes: Vec< SubShape >
}

impl Shape {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			offset: Vector2::zero(),
			sub_shapes: Vec::new(),
		}
	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn load(&mut self, system: &mut System, name: &str) -> bool {
		let filename = format!("{}.shp", name);
		let mut f = system.default_filesystem_mut().open( &filename );
		if !f.is_valid() {
			println!("Error: Couldn't open shape {}", &filename);
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


		let chunk_magic = [ 0x46u8, 0x49, 0x53, 0x48, 0x53, 0x48, 0x50 ]; //0x46u8, 0x49, 0x53, 0x48, 0x4e, 0x5a, 0x4e, ];
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
		if version != 1 {
			println!("Version {} not supported for shape", version);
		}
		self.offset.x = f.read_f32();
		self.offset.y = f.read_f32();

		let vertice_count = f.read_u16();

		if vertice_count > 10000 {
			println!("{} vertices seems to be a bit much", vertice_count);
			return false;
		}

		let mut vertices = Vec::new();

		// :TODO: preallocate

		for _i in 0..vertice_count {
			let x = f.read_f32();
			let y = f.read_f32();

			vertices.push( Vector2::new( x, y ) );
		}

		let shape_count = f.read_u8();

		let mut offsets = Vec::new();
		for _i in 0..shape_count {
			let o = f.read_u16();
			offsets.push( o );
		}

		let mut lengths = Vec::new();
		for _i in 0..shape_count {
			let l = f.read_u16();
			lengths.push( l );
		}

		self.sub_shapes.clear();

		for i in 0..shape_count.into() {
			let o = offsets[ i ];
			let l = lengths[ i ];
			let e = o+l;

			let mut sub_shape = SubShape::new();

			for j in o..e {
				sub_shape.vertices.push( vertices[ j as usize ] );
			}

			self.sub_shapes.push( sub_shape );

		}

		true
	}

	pub fn sub_shape_iter( &self ) -> std::slice::Iter<'_, SubShape> {
		self.sub_shapes.iter()
	}

	pub fn debug_render( &self, debug_renderer: &Option < RefCell< DebugRenderer >  >, offset: &Vector2, pos: &Vector2, rotation: f32 ) {
		if let Some( dr ) = debug_renderer {
			let mut debug_renderer = dr.borrow_mut();
			let color = Color::from_rgba( 0.9, 0.5, 0.5, 0.8 );
			for ss in self.sub_shapes.iter() {
				for i in 0..ss.vertices.len() {
					let v0 = ss.vertices[ i ];
					let v1 = ss.vertices[ ( i+1 ) % ss.vertices.len() ];

					let v0 = v0.add( &offset );
					let v1 = v1.add( &offset );
					
					let mtx = Matrix22::z_rotation( rotation * 0.0174 );

					let v0 = mtx.mul_vector2( &v0 );
					let v1 = mtx.mul_vector2( &v1 );

					let v0 = pos.add( &v0 );
					let v1 = pos.add( &v1 );
					// :TODO: might want to add the offset & position
					debug_renderer.add_line( &v0, &v1, 3.0, &color );
				}
			}
		}
	}
}
