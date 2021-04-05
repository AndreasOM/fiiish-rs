
use std::collections::HashMap;

use crate::math::Matrix44;
use crate::renderer::{
//	Debug,
	Effect,
	gl,
//	Program,
//	ShaderType,
	Texture,
	Uniform,
	Vertex,
};

//#[derive(Debug)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Material {
	#[derivative(Debug="ignore")]
	vertices: Vec<Vertex>,
	buffer: gl::types::GLuint,
	vao: gl::types::GLuint,

	layer_id: u8,
	effect_id: u16,
	texture_hwids: Vec<u16>,

	effect_name: String,
	texture_name: String,

	key: u128,

	uniforms: HashMap< String, Uniform >,

	mvp_matrix: Matrix44,
}

impl Material {

	pub fn new( layer_id: u8, effect: &Effect, textures: Vec< &Texture > ) -> Self {
		let texture_hwids = textures.iter().map( |&t| t.hwid() ).collect();
		let key = Material::calculate_key( layer_id, effect.id(), &texture_hwids );
		let	texture_name = textures.iter().map( |&t| t.name().to_owned() ).collect::<Vec<_>>().join(" ");
		let mut s = Self {
			vertices: Vec::new(),
			buffer: 0xffffffff,
			vao: 0xffffffff,

			layer_id: layer_id,
			effect_id: effect.id(),
			texture_hwids: texture_hwids,

			effect_name: effect.name().to_string(),
			texture_name: texture_name,

			key: key,

			uniforms: HashMap::new(),

			mvp_matrix: Matrix44::identity(),
		};

		unsafe {
			gl::GenVertexArrays( 1, &mut s.vao );
			gl::GenBuffers( 1, &mut s.buffer );
		}

		s
	}

	pub fn calculate_key( layer_id: u8, effect_id: u16, texture_hwids: &Vec< u16 > ) -> u128 {
		// old fiiish: 
		// 00##llll pppppppp rrrrtttt tttttttt

		// .. .. .. .. .. .. .. .. .. .. .. .. .. .. .. ..
		// .. .. .. .. .. .. .. .. .. .. .. .. .l rr tt tt
		// with multi texture support
		// .. .. .. .. .. .. .l rr t3 t3 t2 t2 t1 t1 tt tt

		if texture_hwids.len() > 4 {
			panic!("Too many texture channels. {}", texture_hwids.len() );
		}
		if effect_id > 0xff {
			panic!("Too many effects. Got id {}", &effect_id );
		}

		let mut r = 0u128;
		let mut shift = 0;
		for i in 0..4 {
			if let Some( texture_hwid ) = texture_hwids.get( i ) {
				/* :TODO: reenable if we change type, or extend maximum number of textures
				if *texture_hwid > 0xffff {
					panic!("Too many textures. Got id {}", &texture_hwid );
				}
				*/
				r |= ( *texture_hwid as u128 & 0xffff ) << shift;
			}
			shift += 16;
		}

		  r
		| ( ( effect_id    as u128 &   0xff ) <<  ( shift   ) )
		| ( ( layer_id     as u128 & 0x000f ) <<  ( 8+shift ) )
	}

	pub fn can_render( &self, key: u128 ) -> bool {
//		self.effect_id == effect_id
		self.key == key
	}

	pub fn key( &self ) -> u128 {
		self.key
	}

	pub fn layer_id( &self ) -> u8 {
		self.layer_id
	}

	pub fn effect_id( &self ) -> u16 {
		self.effect_id
	}
/*
	pub fn texture_hwid( &self ) -> u16 {
		self.texture_hwid
	}
*/
	pub fn effect_name( &self ) -> &str {
		&self.effect_name
	}

	pub fn texture_name( &self ) -> &str {
		&self.texture_name
	}

	pub fn clear( &mut self ) {
		self.vertices.clear();
	}

	pub fn add_vertex( &mut self, vertex: &Vertex ) {
		self.vertices.push( *vertex );
	}

	pub fn set_uniform( &mut self, name: &str, value: &Uniform ) {
		self.uniforms.insert( name.to_owned(), *value );
	}

	pub fn set_mvp_matrix( &mut self, mvp_matrix: &Matrix44 ) {
		self.mvp_matrix = *mvp_matrix;
	}

	pub fn render( &mut self, effect: &mut Effect ) -> u32 {
		let vertex_count = self.vertices.len();
		if vertex_count == 0 {
			return 0;
		}

		unsafe {
			gl::Enable( gl::BLEND );
			gl::BlendFunc( gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA );

//			gl::Disable( gl::BLEND );
//			gl::BlendFunc( gl::ONE, gl::ONE );

			gl::BindVertexArray( self.vao );
			gl::BindBuffer( gl::ARRAY_BUFFER, self.buffer );

//			let vertex_size = ( core::mem::size_of<gl::type::GLFloat>() * 3 ) as i32;
			let vertex_size = ( core::mem::size_of::<f32>( ) * ( 3 + 2 + 4 ) ) as isize; // :HACK:
			// :TODO: we might want to reuse this
			gl::BufferData(
				gl::ARRAY_BUFFER,
				vertex_size * vertex_count as isize,
				self.vertices.as_ptr() as *const core::ffi::c_void,
				gl::STATIC_DRAW		 									//maybe STREAM?
			);

			let attrib_pos_index = 0;
			let attrib_tex_coords_index = 1;

			gl::EnableVertexAttribArray( attrib_pos_index );
			gl::VertexAttribPointer( attrib_pos_index, 3, gl::FLOAT, gl::FALSE, vertex_size as i32, 0 as *const _ );

			// :TODO: only enable when needed
			gl::EnableVertexAttribArray( attrib_tex_coords_index );
			gl::VertexAttribPointer( attrib_tex_coords_index, 2, gl::FLOAT, gl::FALSE, vertex_size as i32, ( 3 * 4 ) as *const _ );

			effect.r#use();
			// :HACK:
			for ( n, v ) in self.uniforms.iter() {
				match effect.program().lookup_uniform( &n ) {
					Some( l ) => {
//						println!("Location for {} is {}", &n, l );
						match v {
							Uniform::F32( v ) => {
								gl::Uniform1f( l, *v );
							},
//							_ => todo!("Uniform type not supported. {:?}", v ),

						}
					},
					None => {
						println!("Warning: Location of {} not found {:?}", &n, &effect );
					},
				}
			}
			for ( n, l ) in effect.program().uniforms_iter() {
//				println!("{} -> {}", &n, &l );
				match n.as_str() {
					"texture0\0" => {
						gl::Uniform1i( *l, 0 );	// always use channel 0 for texture0
					},
					"texture1\0" => {
						gl::Uniform1i( *l, 1 );	// always use channel 1 for texture1
					},
					"modelViewProjectionMatrix\0" => {
						gl::UniformMatrix4fv( *l, 1, 0, self.mvp_matrix.as_ptr() as *const _ );

					}
					_ => {
						todo!("handle uniform {:?}", &n);
					},
				}
			}
//			gl::Uniform1i( 0, 0 );

			for ( i, hwid ) in self.texture_hwids.iter().enumerate() {
				gl::ActiveTexture( gl::TEXTURE0+i as u32 );
				gl::BindTexture( gl::TEXTURE_2D, *hwid as u32 );
			}
			// :TODO: decide if we really cleanup in case somebody else expects defaults
			gl::ActiveTexture( gl::TEXTURE0 );

//			dbg!(&self.vertices);
//			gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
			gl::DrawArrays( gl::TRIANGLES, 0, vertex_count as i32 );
//			println!("Rendering {} vertices", vertex_count);
			vertex_count as u32
		}
//		dbg!(&self);
	}
}
/*
impl std::fmt::Debug for Material {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!(
			f, "Material: {} {} effect: {} [{}]",
			self.buffer,
			self.vao,
			&self.effect_name,
			self.effect_id,
		)
	}
}
*/
