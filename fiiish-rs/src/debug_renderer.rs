
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
}
