use std::sync::Arc;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::math::Matrix22;
use crate::math::Vector2;

use crate::renderer::{
	Color,
	Renderer
};


#[derive(Debug)]
struct Line {
	start: Vector2,
	end: Vector2,
	width: f32,
	color: Color,
}

#[derive(Debug)]
pub struct DebugRenderer {
	layer: u8,
	effect: u16,
	lines: Vec< Line >,
}

//pub static mut DEFAULT_DEBUGRENDERER: Option< Arc< Mutex< DebugRenderer > > > = None;
//pub static DEFAULT_DEBUGRENDERER: Arc< Mutex < Option< DebugRenderer > > > = Arc::new( Mutex::new( None ) );

lazy_static! {
//    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
	pub static ref DEFAULT_DEBUGRENDERER: Arc< Mutex < Option< DebugRenderer > > > = Arc::new( Mutex::new( None ) );
}

// :TODO: make these macros that compiles to NOP
pub fn debug_renderer_toggle( layer_id: u8, effect_id: u16 ) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if dr.is_none() {
			**dr = Some( DebugRenderer::new( layer_id, effect_id ) );
		} else {
			**dr = None;
		}		
	}	
}

pub fn debug_renderer_add_line( start: &Vector2, end: &Vector2, width: f32, color: &Color ) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some( dr ) = &mut **dr {
			dr.add_line( start, end, width, color );
		}		
	}
}
pub fn debug_renderer_add_frame( pos: &Vector2, size: &Vector2, width: f32, color: &Color ) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some( dr ) = &mut **dr {
			dr.add_frame( pos, size, width, color );
		}		
	}
}

pub fn debug_renderer_begin_frame( ) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some( dr ) = &mut **dr {
			dr.begin_frame( );
		}		
	}
}

pub fn debug_renderer_render( renderer: &mut Renderer ) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some( dr ) = &mut **dr {
			dr.render( renderer );
		}
	}
}


// end of macros

impl DebugRenderer {
	pub fn new( layer: u8, effect: u16 ) -> Self {
		Self {
			layer,
			effect,
			lines: Vec::new(),
		}
	}


	pub fn begin_frame( &mut self ) {
		self.lines.clear();
	}
	pub fn end_frame( &mut self ) {
		
	}

	pub fn render( &self, renderer: &mut Renderer ) {
//		println!("Debug Render rendering");
//		println!("{} lines", self.lines.len());

		renderer.use_layer( self.layer );
		renderer.use_effect( self.effect );
		for l in &self.lines {
			let v0 = l.start;
			let v1 = l.end;
			let v01 = v1.sub( &v0 ).normalized();
			let vp = Vector2::new( -v01.y, v01.x );
//			let vp = Vector2::new( 0.0, 1.0 );
			let vl = vp.scaled( 0.5*l.width );
			let vr = vp.scaled( -0.5*l.width );

			let v0 = l.start.add( &vr );
			let v1 = l.end.add( &vr );

			let v2 = l.start.add( &vl );
			let v3 = l.end.add( &vl );

//			println!("{:?} {:?} \n{:?} {:?} {:?} {:?} \n{:?} {:?} {:?} ",&l.start, &l.end, &v0,&v1,&v2, &v3, &v01, &vl, &vr);
//			println!("{} + {} = {}", l.start.y, vl.y, v2.y );

			renderer.set_color( &l.color );

			let v0 = renderer.add_vertex( &v0 );
			let v1 = renderer.add_vertex( &v1 );
			let v2 = renderer.add_vertex( &v2 );
			let v3 = renderer.add_vertex( &v3 );

			renderer.add_triangle( v0, v1, v2 );
			renderer.add_triangle( v2, v1, v3 );

		}
	}

	pub fn add_line( &mut self, start: &Vector2, end: &Vector2, width: f32, color: &Color ) {
		let line = { Line { start: *start, end: *end, width, color: *color } };
		self.lines.push( line );
	}
	pub fn add_frame( &mut self, pos: &Vector2, size: &Vector2, width: f32, color: &Color ) {
		let half_size = size.scaled_vector2( &Vector2::new( -0.5, 0.5 ) );
		let top_left = pos.add( &half_size );
		let bottom_right = pos.sub( &half_size );

		self.add_line( &top_left, &bottom_right, width, color );
		self.add_line( &Vector2::new( bottom_right.x, top_left.y ), &Vector2::new( top_left.x, bottom_right.y ), width, color );
		self.add_line( &Vector2::new( top_left.x, top_left.y ), &Vector2::new( top_left.x, bottom_right.y ), width, color );
		self.add_line( &Vector2::new( bottom_right.x, top_left.y ), &Vector2::new( bottom_right.x, bottom_right.y ), width, color );
		self.add_line( &Vector2::new( top_left.x, top_left.y ), &Vector2::new( bottom_right.x, top_left.y ), width, color );
		self.add_line( &Vector2::new( top_left.x, bottom_right.y ), &Vector2::new( bottom_right.x, bottom_right.y ), width, color );
	}
	pub fn add_circle( &mut self, pos: &Vector2, radius: f32, width: f32, color: &Color ) {
		let mut vr = Vector2::new( radius, 0.0 );

		// we could add some code to decide on the number of segments here
		let n = 24;
		let r_step = 360.0 / n as f32;
		// rotate
		let mtx = Matrix22::z_rotation( r_step * 0.01745329252); // DEG to RAD


		let mut vertices = Vec::new();

		for _ in 0..n {
			let v = pos.add( &vr );
			vertices.push( v );

			vr = mtx.mul_vector2( &vr );
		}

		for i in 0..vertices.len() {
			let v0 = vertices[ i ];
			let v1 = vertices[ ( i + 1 ) % vertices.len() ];
			self.add_line( &v0, &v1, width, color );
		} 

	}
}
