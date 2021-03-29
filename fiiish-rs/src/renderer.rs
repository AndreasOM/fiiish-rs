
use std::collections::HashMap;

use crate::math::Vector2;
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
	pub fn from_rgba( r: f32, g: f32, b: f32, a: f32 ) -> Self {
		Self {
			r,
			g,
			b,
			a,
		}
	}
}

#[derive(Debug,Copy,Clone)]
pub struct Vertex {
	pos: [f32;3],
}

impl Vertex {
	pub fn from_xyz( x: f32, y: f32, z: f32 ) -> Self {
		Self {
			pos: [ x, y, z ],
		}
	}
}

#[derive(Debug)]
pub struct Renderer {
	frame: u64,
	material_manager: MaterialManager,
	vertices: Vec<Vertex>,
	effects: HashMap< u16, Effect >,
	default_effect_id: u16,
	active_effect_id: u16,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			frame: 0,
			material_manager: MaterialManager::new(),
			vertices: Vec::new(),		// :TODO: pre allocate size? or maybe even a fixed size array
			effects: HashMap::new(),
			default_effect_id: 0,
			active_effect_id: 0,
		}
	}

	pub fn register_effect( &mut self, mut effect: Effect ) {
		if self.effects.len() == 0 { 
			self.default_effect_id = effect.id();
		}
		self.effects.insert(effect.id(), effect);
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
			None => panic!("No active render Effect")
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
		Ok(())
	}

	pub fn teardown( &mut self ) {

	}

	pub fn begin_frame( &mut self ) {
		self.vertices.clear();
		for material in self.material_manager.iter_mut() {
			material.clear();
		}
		// enusre we have at least one material, and it is active
		if self.material_manager.len() == 0 {
			let m = Material::new( &self.get_default_effect() );
			let i = self.material_manager.add( m );
			self.material_manager.set_active( i );			
		}
//		let default_effect_name = self.default_effect_name.clone();
//		self.use_effect( &default_effect_name );
	}

	pub fn end_frame( &mut self ) {
		let debug = self.frame % 500 == 0;
		// just to avoid ghost
		unsafe {
//			gl::Disable(gl::CULL_FACE);
			gl::Enable(gl::CULL_FACE);
			gl::Disable(gl::DEPTH_TEST);
		}

		// :TODO: fix rendering order
		for material in self.material_manager.iter_mut() {
			// :TODO: ask material for effect
			let effect_id = material.effect_id();
			let e = match self.effects.get_mut( &effect_id ) {
				Some( e ) => e,
				None => match self.effects.get_mut( &self.default_effect_id ) {
					Some( e ) => e,
					None => panic!("No default render Effect")
				}
			};
			let vc = material.render( e );
			if debug {
				println!("Rendered {} vertices for material {:?} with effect {:?}", vc, &material, &e );
			}
		}

		// glFlush or glFinish
		unsafe {
			gl::Flush();
		}

		if debug {
			dbg!(&self.material_manager);
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

	fn switch_active_material_if_needed( &mut self ) {
//		println!("switch_active_material_if_needed active_effect_name {}", &self.active_effect_name);
		let eid = self.get_active_effect().id();
		let can_render = {
			let m = self.material_manager.get_active();
			m.can_render( eid )
		};

		if !can_render {
			let found_material = self.material_manager.select_active(|m: &Material|{
				m.can_render( eid )
			});
			if !found_material {
				println!("Didn't find material for effect id {} active_effect_id {}", eid, &self.active_effect_id );
				let m = Material::new( &self.get_active_effect() );
				let i = self.material_manager.add( m );
				self.material_manager.set_active( i );
			}
		}

	}
	pub fn use_effect( &mut self, effect_id: u16 ) {
		self.active_effect_id = effect_id;
		self.switch_active_material_if_needed();
	}

	pub fn add_vertex( &mut self, x: f32, y: f32 ) -> u32 {
		let v = Vertex::from_xyz( x, y, 0.0 );
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

		let v0 = self.add_vertex( -hs.x + pos.x,  hs.y + pos.y ); // TopLeft
		let v1 = self.add_vertex( -hs.x + pos.x, -hs.y + pos.y ); // BottomLeft
		let v2 = self.add_vertex(  hs.x + pos.x, -hs.y + pos.y ); // BottomRight
		let v3 = self.add_vertex(  hs.x + pos.x,  hs.y + pos.y ); // TopRight
		self.add_triangle( v0, v1, v2 ); // TopLeft, BottomLeft, BottomRight
		self.add_triangle( v2, v3, v0 ); // BottomRight, TopRight, TopLeft		
	}

}

#[derive(Debug)]
struct MaterialManager {
	materials: Vec<Material>,
	active_index: usize,
}

impl MaterialManager {
	pub fn new() -> Self {
		Self {
			materials: Vec::new(),
			active_index: 0,
		}
	}

	pub fn set_active( &mut self, index: usize ) {
		self.active_index = index;
	}

	pub fn select_active<F>( &mut self, f: F) -> bool
		where F: Fn( &Material ) -> bool
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
	pub fn add( &mut self, material: Material ) -> usize {
		let i = self.materials.len();
		self.materials.push(material);
		i
	}

	pub fn iter_mut( &mut self ) -> std::slice::IterMut<'_, Material> {
		self.materials.iter_mut()
	}
	pub fn get_mut_active( &mut self ) -> &mut Material {
		match self.materials.get_mut( self.active_index ) {
			Some( m ) => m,
			None => panic!("No active Material"),
		}
	}
	pub fn get_active( &self ) -> &Material {
		match self.materials.get( self.active_index ) {
			Some( m ) => m,
			None => panic!("No active Material"),
		}
	}

}


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
