
use std::collections::HashMap;

use crate::math::{ 
	Matrix22,
	Matrix32,
	Matrix33,
	Matrix44,
	Vector2
};
use crate::system::System;
use crate::window::Window;

//use material::Material;

#[derive(Debug)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {
	pub fn red() -> Self {
		Self {
			r: 1.0,
			g: 0.0,
			b: 0.0,
			a: 1.0,
		}
	}
	pub fn green() -> Self {
		Self {
			r: 0.0,
			g: 1.0,
			b: 0.0,
			a: 1.0,
		}
	}
	pub fn blue() -> Self {
		Self {
			r: 0.0,
			g: 0.0,
			b: 1.0,
			a: 1.0,
		}
	}
	pub fn from_rgba( r: f32, g: f32, b: f32, a: f32 ) -> Self {
		Self {
			r,
			g,
			b,
			a,
		}
	}

	pub fn as_rgba8( &self ) -> u32 {
		let r = ( self.r * 255.0 ) as u32;
		let g = ( self.g * 255.0 ) as u32;
		let b = ( self.b * 255.0 ) as u32;
		let a = ( self.a * 255.0 ) as u32;
		  ( r << 24 )
		| ( g << 16 )
		| ( b <<  8 )
		| ( a <<  0 )
	}
	pub fn as_abgr8( &self ) -> u32 {
		let r = ( self.r * 255.0 ) as u32;
		let g = ( self.g * 255.0 ) as u32;
		let b = ( self.b * 255.0 ) as u32;
		let a = ( self.a * 255.0 ) as u32;
		  ( r <<  0 )
		| ( g <<  8 )
		| ( b << 16 )
		| ( a << 24 )
	}
}

#[derive(Debug,Copy,Clone)]
pub struct Vertex {
	pos: [f32;3],
	tex_coords: [f32;2],
	color: [f32;4],
}

impl Vertex {
	pub fn from_xyz( x: f32, y: f32, z: f32 ) -> Self {
		Self {
			pos: [ x, y, z ],
			tex_coords: [ 0.0, 0.0 ],
			color: [ 1.0, 1.0, 1.0, 1.0 ],
		}
	}
	pub fn from_pos_with_tex_coords( pos: &Vector2, tex_coords: &Vector2 ) -> Self {
		Self {
			pos: [ pos.x, pos.y, 0.0 ],
			tex_coords: [ tex_coords.x, tex_coords.y ],
			color: [ 1.0, 1.0, 1.0, 1.0 ],
		}
	}
}

const MAX_TEXTURE_CHANNELS: usize = 4;
#[derive(Debug)]
pub struct Renderer {
	frame: u64,
	material_manager: Manager<Material>,
	texture_manager: Manager<Texture>,
	vertices: Vec<Vertex>,
	effects: HashMap< u16, Effect >,
	default_effect_id: u16,
	active_effect_id: u16,
	active_layer_id: u8,

	active_textures: [Option< u16 >; MAX_TEXTURE_CHANNELS],

	tex_coords: Vector2,

	mvp_matrix: Matrix44,
	tex_matrix: Matrix32,

	size: Vector2,
	viewport_pos: Vector2,
	viewport_size: Vector2,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			frame: 0,
			material_manager: Manager::new(),
			texture_manager: Manager::new(),
			vertices: Vec::new(),		// :TODO: pre allocate size? or maybe even a fixed size array
			effects: HashMap::new(),
			default_effect_id: 0,
			active_effect_id: 0,
			active_layer_id: 0,
			active_textures: [None; MAX_TEXTURE_CHANNELS],

			tex_coords: Vector2::zero(),
			mvp_matrix: Matrix44::identity(),
			tex_matrix: Matrix32::identity(),

			size: Vector2::zero(),
			viewport_pos: Vector2::zero(),
			viewport_size: Vector2::zero(),
		}
	}

	pub fn register_effect( &mut self, mut effect: Effect ) {
		if self.effects.len() == 0 { 
			self.default_effect_id = effect.id();
		}
		self.effects.insert(effect.id(), effect);
	}

	pub fn register_texture( &mut self, mut texture: Texture ) {
		let index = self.texture_manager.add( texture );
		if self.texture_manager.len() == 1 {
//			self.texture_manager.set_active( index );
			self.active_textures[ 0 ] = Some( index as u16 );
		}
	}

	fn get_default_effect(&self) -> &Effect {
		match self.effects.get( &self.default_effect_id ) {
			Some( e ) => e,
			None => panic!("No default render Effect")
		}
	}

	fn get_active_effect(&self) -> &Effect {
		match self.effects.get( &self.active_effect_id ) {
			Some( e ) => e,
			None => {
				println!("No active render Effect -> using default");
				self.get_default_effect()
			}
		}
	}

	pub fn setup( &mut self, window: &Window, system: &mut System ) -> anyhow::Result<()> {
		gl::load_with(|s| window.get_proc_address(s) as *const _); // :TODO: maybe use CFBundleGetFunctionPointerForName directly

		// :HACK: create one effect
		/*
		let e = Effect::create( system, "Default", "default_vs.glsl", "default_fs.glsl" );
		self.default_effect_name = e.name().to_string();
		self.active_effect_name = e.name().to_string();
		self.register_effect( e );

		let e = Effect::create( system, "White", "default_vs.glsl", "white_fs.glsl" );
		self.register_effect( e );
		*/
		// :HACK: create one material

		/*
		let e = self.get_mut_default_effect();
		let m = Material::new( e );
		self.material_manager.add( m );
		*/

		unsafe {
			let s = gl::GetString( gl::VERSION );
			let s = String::from_utf8( std::ffi::CStr::from_ptr( s as *const _ ).to_bytes().to_vec() )?;
			println!("GL Version: {}", s );
		}

		// ensure we have one texture
		self.register_texture( Texture::create_canvas( system, "[]", 2 ) );

		Ok(())
	}

	pub fn teardown( &mut self ) {

	}

	pub fn begin_frame( &mut self ) {
		self.vertices.clear();
		for material in self.material_manager.iter_mut() {
			material.clear();
		}
		// ensure we have at least one material, and it is active
		if self.material_manager.len() == 0 {
			let mut textures = Vec::new();
			for i in 0..MAX_TEXTURE_CHANNELS {
				let ti = self.active_textures[ i ].unwrap_or( 0 );
				textures.push( self.texture_manager.get( ti as usize ).unwrap() );
			}
			let m = Material::new( self.active_layer_id, &self.get_default_effect(), textures );
			let i = self.material_manager.add( m );
			self.material_manager.set_active( i );			
		}
//		let default_effect_name = self.default_effect_name.clone();
//		self.use_effect( &default_effect_name );

		unsafe {
			let p = &self.viewport_pos;
			let s = &self.viewport_size;
			gl::Viewport( p.x as i32, p.y as i32, s.x as i32, s.y as i32 );
//			gl::Scissor( self.pos.x as i32, self.pos.y as i32, self.size.x as i32, self.size.y as i32 );
//			gl::Enable( gl::SCISSOR_TEST );
		}
	}

	pub fn end_frame( &mut self ) {
		let mut total_vertices = 0;
		let mut total_materials = 0;
		let mut total_materials_with_vertices = 0;

		let debug = self.frame % 500 == 0;
		// just to avoid ghost
		unsafe {
//			gl::Disable(gl::CULL_FACE);
			gl::Enable(gl::CULL_FACE);
			gl::Disable(gl::DEPTH_TEST);
		}

//		println!("---");
		// :TODO: fix rendering order
		let mut material_indices = Vec::new();
		for i in 0..self.material_manager.len() {
			material_indices.push( i );
		}

		material_indices.sort_unstable_by(|a, b|{
			let a = self.material_manager.get( *a ).unwrap().key();
			let b = self.material_manager.get( *b ).unwrap().key();

			a.partial_cmp(&b).unwrap()
		});

		for i in material_indices {
			let material = self.material_manager.get_mut( i ).unwrap();

//			println!("SortKey: 0x{:016X}", material.key() );
			// :TODO: ask material for effect
			let effect_id = material.effect_id();
			let e = match self.effects.get_mut( &effect_id ) {
				Some( e ) => e,
				None => match self.effects.get_mut( &self.default_effect_id ) {
					Some( e ) => e,
					None => panic!("No default render Effect")
				}
			};
			material.set_mvp_matrix( &self.mvp_matrix );
			let vc = material.render( e );
			total_vertices += vc;
			total_materials += 1;
			if vc > 0 {
				total_materials_with_vertices += 1;
			}
			if debug {
//				println!("Rendered {} vertices for material {:?} with effect {:?}", vc, &material, &e );
			}
		}

		// glFlush or glFinish
		unsafe {
			gl::Flush();
		}

		if debug {
//			dbg!(&self.material_manager);
			println!("Render Stats: {} {} {}", total_vertices, total_materials_with_vertices, total_materials);
		}
		self.frame += 1;
	}

	// rendering functions

	pub fn clear( &mut self, color: &Color ) {
//		println!("clear with {:?}", &color );
		// glClearColor and glClear
		unsafe {
			gl::ClearColor( color.r, color.g, color.b, color.a );
			gl::Clear( gl::COLOR_BUFFER_BIT ); // :TODO: clear other buffers?
		}
	}

	pub fn aspect_ratio( &self ) -> f32 {
		self.size.x / self.size.y
	}

	pub fn size( &self ) -> &Vector2 {
		&self.size
	}

	pub fn set_size( &mut self, size: &Vector2 ) {
		self.size = *size;
	}

	pub fn set_viewport( &mut self, pos: &Vector2, size: &Vector2 ) {
		self.viewport_pos = *pos;
		self.viewport_size = *size;
	}

	pub fn set_mvp_matrix( &mut self, mvp_matrix: &Matrix44 ) {
		self.mvp_matrix = *mvp_matrix;
	}

	pub fn set_tex_matrix( &mut self, tex_matrix: &Matrix32 ) {
		self.tex_matrix = *tex_matrix;
	}

	pub fn set_uniform_float( &mut self, name: &str, value: f32 ) {
		// :HACK:
		let mut m = self.material_manager.get_mut_active();
		m.set_uniform( name, &Uniform::F32( value ) );
	}
	
	fn switch_active_material_if_needed( &mut self ) {
//		println!("switch_active_material_if_needed active_effect_name {}", &self.active_effect_name);
		let lid = self.active_layer_id;
		let eid = self.get_active_effect().id();
		let mut textures = Vec::new();
		for i in 0..MAX_TEXTURE_CHANNELS {
			let ti = self.active_textures[ i ].unwrap_or( 0 );
			textures.push( self.texture_manager.get( ti as usize ).unwrap() );
		}
		let tids = textures.iter().map( |&t| t.hwid() ).collect::<Vec<_>>().to_vec();
		let key = Material::calculate_key( lid, eid, &tids );
		let can_render = {
			let m = self.material_manager.get_active();
			m.can_render( key )
		};

		if !can_render {
			let found_material = self.material_manager.select_active(|m: &Material|{
				m.can_render( key )
			});
			if !found_material {
				println!(
					"Didn't find material for layer id {} effect id {} active_effect_id {}",
					lid,
					eid,
					&self.active_effect_id
				);
				let mut textures = Vec::new();
				for i in 0..MAX_TEXTURE_CHANNELS {
					let ti = self.active_textures[ i ].unwrap_or( 0 );
					textures.push( self.texture_manager.get( ti as usize ).unwrap() );
				}
				let m = Material::new( self.active_layer_id, &self.get_active_effect(), textures );
				let i = self.material_manager.add( m );
				self.material_manager.set_active( i );
			}
		}

	}
	pub fn use_effect( &mut self, effect_id: u16 ) {
		self.active_effect_id = effect_id;
		self.switch_active_material_if_needed();
	}

	pub fn use_layer( &mut self, layer_id: u8 ) {
		self.active_layer_id = layer_id;
		self.switch_active_material_if_needed();
	}

	pub fn use_texture( &mut self, name: &str ) {
		self.use_texture_in_channel( name, 0 );
		/*
		let current_active_texture = self.texture_manager.get_active();
		if name != current_active_texture.name() {
//			println!("Switching active texture from {} to {}", &current_active_texture.name(), &name );

			let found_texture = self.texture_manager.select_active(|t: &Texture|{
				t.name() == name
			});

			if !found_texture {
				println!("Warning: Texture {} not found using default", &name);
				self.texture_manager.set_active( 0 );
			}
			self.switch_active_material_if_needed();
		}
		*/
	}

	pub fn disable_texture_for_channel( &mut self, channel: u8 ) {
		if self.active_textures[ channel as usize ].is_none() {
			// :TODO: is this ok?
			return;
		}

		self.active_textures[ channel as usize ] = None;

		self.switch_active_material_if_needed();
	}

	pub fn use_texture_in_channel( &mut self, name: &str, channel: u8 ) {
		// :TODO: avoid changing texture when it is already active
//		dbg!(&self.texture_manager);
		let found_texture = match self.texture_manager.find_index(|t: &Texture|{
//			dbg!(&t.name(), &name);
			t.name() == name
		}) {
			None => todo!("Texture not found {}. User error?", &name),
			Some( i ) => {
				self.active_textures[ channel as usize ] = Some ( i as u16 );
				self.switch_active_material_if_needed();
			},
		};
	}


	pub fn set_tex_coords( &mut self, tex_coords: &Vector2 ) {
		self.tex_coords = *tex_coords;
	}
	pub fn add_vertex( &mut self, pos: &Vector2 ) -> u32 {
		let v = Vertex::from_pos_with_tex_coords( pos, &self.tex_coords );
		self.vertices.push( v );
		self.vertices.len() as u32 - 1
	}

	pub fn add_triangle( &mut self, v0: u32, v1: u32, v2: u32 ) {
		let material = self.material_manager.get_mut_active();
		for v in [v0, v1, v2].iter() {
			match self.vertices.get( *v as usize ) {
				Some( v ) => {
					material.add_vertex( v );
				},
				None => {
					// :TODO: shout loud
				},
			}
		}				
	}

	pub fn render_quad( &mut self, pos: &Vector2, size: &Vector2 ) {
		let mut hs = *size;	// hs => half size
		hs.x = 0.5 * hs.x;
		hs.y = 0.5 * hs.y;

		let tl = Vector2::new( -hs.x + pos.x,  hs.y + pos.y );
		let bl = Vector2::new( -hs.x + pos.x, -hs.y + pos.y );
		let br = Vector2::new(  hs.x + pos.x, -hs.y + pos.y );
		let tr = Vector2::new(  hs.x + pos.x,  hs.y + pos.y );

		let v0 = self.add_vertex( &tl );
		let v1 = self.add_vertex( &bl );
		let v2 = self.add_vertex( &br );
		let v3 = self.add_vertex( &tr );

		self.add_triangle( v0, v1, v2 ); // TopLeft, BottomLeft, BottomRight
		self.add_triangle( v2, v3, v0 ); // BottomRight, TopRight, TopLeft		
	}

	pub fn render_textured_fullscreen_quad( &mut self ) {
		let size = self.size;
//		dbg!(&size);
		self.render_textured_quad( &Vector2::zero(), &size );
	}

	pub fn render_textured_quad( &mut self, pos: &Vector2, size: &Vector2 ) {
		self.render_textured_quad_with_rotation( pos, size, 0.0 );
	}

	pub fn render_textured_quad_with_rotation( &mut self, pos: &Vector2, size: &Vector2, angle: f32 ) {
		let angle = angle * 0.01745329252;

		let mtx = Matrix22::z_rotation( angle );

		let positions = [
			Vector2::new( -0.5,  0.5 ),
			Vector2::new( -0.5, -0.5 ),
			Vector2::new(  0.5, -0.5 ),
			Vector2::new(  0.5,  0.5 ),
		];

		let tex_coords = [
			Vector2::new( 0.0, 0.0 ),
			Vector2::new( 0.0, 1.0 ),
			Vector2::new( 1.0, 1.0 ),
			Vector2::new( 1.0, 0.0 ),
		];

//		let tex_mtx = Matrix32::identity();
		let ti = self.active_textures[ 0 ].unwrap_or( 0 );
		let at = self.texture_manager.get( ti as usize ).unwrap_or(
			self.texture_manager.get( 0 ).unwrap()
		);

		let tex_mtx = *at.mtx();
		let user_tex_mtx = self.tex_matrix;

		let mut v = [0u32;4];

		// :TODO: future optimization once we have full matrix implementation
//		let mtx_tr = Matrix32::translation( pos.x, pos.y );
//		let mtx = mtx_r.mul_matrix( &mtx );
		for i in 0..4 {
			let p = positions[ i ];
			let p = p.scale_vector2( &size );
			let p = mtx.mul_vector2( &p ).add( &pos );

			// :TODO: decide if we might want to move this calculation to set_tex_coords
			let t = tex_coords[ i ];
			let t = user_tex_mtx.mul_vector2( &t );
			let t = tex_mtx.mul_vector2( &t );
		
			self.set_tex_coords( &t );
			v[ i ] = self.add_vertex( &p );
		}

		self.add_triangle( v[ 0 ], v[ 1 ], v[ 2 ] );
		self.add_triangle( v[ 2 ], v[ 3 ], v[ 0 ] );
	}

	pub fn find_texture_mut( &mut self, name: &str ) -> Option< &mut Texture > {
		self.texture_manager.find_mut( |t|{
			t.name() == name
		})
	}

}

#[derive(Debug)]
struct Manager<T> {
	materials: Vec<T>,
	active_index: usize,
}

impl <T>Manager<T> {
	pub fn new() -> Self {
		println!("Creating manager for {}", std::any::type_name::<T>());

		Self {
			materials: Vec::new(),
			active_index: 0,
		}
	}

	pub fn set_active( &mut self, index: usize ) {
		self.active_index = index;
	}

	pub fn select_active<F>( &mut self, f: F) -> bool
		where F: Fn( &T ) -> bool
	{
		for (i,m) in self.materials.iter().enumerate() {
			if f( m ) {
				self.active_index = i;
				return true;
			}
		}
		false
	}

	pub fn len( &self ) -> usize {
		self.materials.len()
	}
	pub fn add( &mut self, material: T ) -> usize {
		let i = self.materials.len();
		self.materials.push(material);
		i
	}

	pub fn get( &self, index: usize ) -> Option< &T > {
		self.materials.get( index )
	}

	pub fn get_mut( &mut self, index: usize ) -> Option< &mut T > {
		self.materials.get_mut( index )
	}

	pub fn find_index<F>( &mut self, f: F) -> Option< usize >
		where F: Fn( &T ) -> bool
	{
		for (i,m) in self.materials.iter().enumerate() {
			if f( m ) {
				return Some( i );
			}
		}
		None
	}

	pub fn find_mut<F>( &mut self, f: F ) -> Option< &mut T >
		where F: Fn( &T ) -> bool
	{
		for m in self.materials.iter_mut() {
			if f( &m ) {
				return Some( m );
			}
		}

		None
	}
	pub fn iter( &mut self ) -> std::slice::Iter<'_, T> {
		self.materials.iter()
	}
	pub fn iter_mut( &mut self ) -> std::slice::IterMut<'_, T> {
		self.materials.iter_mut()
	}
	pub fn get_mut_active( &mut self ) -> &mut T {
		match self.materials.get_mut( self.active_index ) {
			Some( m ) => m,
			None => panic!("No active {}", std::any::type_name::<T>()),
		}
	}
	pub fn get_active( &self ) -> &T {
		match self.materials.get( self.active_index ) {
			Some( m ) => m,
			None => panic!("No active {}", std::any::type_name::<T>()),
		}
	}

}


mod animated_texture;
	pub use animated_texture::AnimatedTexture as AnimatedTexture;

mod debug;
	pub use debug::Debug as Debug;
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod effect;
	pub use effect::Effect as Effect;
mod material;
	pub use material::Material as Material;
//mod material_builder;
//	pub use material_builder::MaterialBuilder as MaterialBuilder;
mod program;
	pub use program::Program as Program;
	pub use program::ShaderType as ShaderType;
mod texture;
	pub use texture::Texture as Texture;
mod texture_atlas;
	pub use texture_atlas::TextureAtlas as TextureAtlas;
mod uniform;
	pub use uniform::Uniform as Uniform;

