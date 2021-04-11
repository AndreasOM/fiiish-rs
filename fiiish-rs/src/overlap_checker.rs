
use std::cell::RefCell;

use crate::fiiish::{ Shape, SubShape };
use crate::math::{
	Matrix22,
	Vector2
};
use crate::renderer::Color;
use crate::DebugRenderer;

#[derive(Debug)]
pub struct OverlapCheckerItem<'a> {
	pub shape: &'a Shape,
	pub pos: &'a Vector2,
	pub offset: &'a Vector2,
	pub rotation: f32,
}

#[derive(Debug)]
pub struct OverlapChecker {

}

#[derive(Debug)]
struct OverlapCheckerSubItem<'a> {
	pub sub_shape: &'a SubShape,
	pub pos: &'a Vector2,
	pub offset: &'a Vector2,
	pub rotation: f32,
}

#[derive(Debug)]
struct Segment {
	start: Vector2,
	end: Vector2,
}

#[derive(Debug,PartialEq,Eq)]
enum Side {
	None = 0,
	Left = -1,
	Right = 1,
}

impl OverlapChecker {

	fn get_segments( a: &OverlapCheckerSubItem ) -> Vec< Segment > {
		let mut segments_a = Vec::new();
		for i in 0..a.sub_shape.vertices.len() {
			let v0 = a.sub_shape.vertices[ i ];
			let v1 = a.sub_shape.vertices[ (i+1) % a.sub_shape.vertices.len() ];

			let v0 = v0.add( &a.offset );
			let v1 = v1.add( &a.offset );
			
			let mtx = Matrix22::z_rotation( a.rotation * 0.0174 );

			let v0 = mtx.mul_vector2( &v0 );
			let v1 = mtx.mul_vector2( &v1 );

			let v0 = a.pos.add( &v0 );
			let v1 = a.pos.add( &v1 );

			let s = Segment {
				start: v0,
				end: v1,
			};

			segments_a.push( s );
		}

		segments_a		
	}

	fn do_sub_shapes_overlap( a: &OverlapCheckerSubItem, b: &OverlapCheckerSubItem, debug_renderer: &Option < RefCell< DebugRenderer >  > ) -> bool {
		let segments_a = OverlapChecker::get_segments( &a );
		let segments_b = OverlapChecker::get_segments( &b );

		let color_l = Color::green();
		let color_r = Color::red();

		if let Some( debug_renderer ) = debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			let color = Color::green();
			let mut color_dim = color;
			color_dim.a *= 0.25;

			let mut n = 0;
			let mut any_inside = false;

			for sa in segments_a.iter() {
				let p = sa.start;
				let mut last_side = Side::None;

				for sb in segments_b.iter() {
					// check if p is left or right of sb
					let sv = sb.end.sub( &sb.start );
					let sp = p.sub( &sb.start );

					let cp = sp.cross( &sv );
					let is_left = cp.x > 0.0;

					if is_left && last_side == Side::Right {
						any_inside = true;
//						break;
					} else if !is_left && last_side == Side::Left {
						any_inside = true;
//						break;
					}

					last_side = if is_left { Side::Left } else { Side::Right };

					if n < 5 {
						let off = Vector2::new( 350.0, 0.0 ).scaled( n as f32 - 2.5 );

						debug_renderer.add_line( &Vector2::zero().add( &off ), &sv.add( &off ), 5.0, &color );
						if is_left {
							debug_renderer.add_line( &Vector2::zero().add( &off ), &sp.add( &off ), 5.0, &color_l );
						} else {
							debug_renderer.add_line( &Vector2::zero().add( &off ), &sp.add( &off ), 5.0, &color_r );							
						}
						debug_renderer.add_line( &sb.end, &sv.add( &off ), 1.0, &color_dim );
					}

					n += 1;
				}
			}

			let result_color = if any_inside {
				Color::from_rgba( 0.8, 0.2, 0.8, 0.5 )
			} else {
				Color::from_rgba( 0.2, 0.8, 0.2, 0.5 )
			};

			for s in segments_a.iter() {
				debug_renderer.add_line( &s.start, &s.end, 2.0, &result_color );
			}
			for s in segments_b.iter() {
				debug_renderer.add_line( &s.start, &s.end, 2.0, &result_color );
			}
		}

		false
	}

	pub fn do_shapes_overlap( a: &OverlapCheckerItem, b: &OverlapCheckerItem, debug_renderer: &Option < RefCell< DebugRenderer >  > ) -> bool {
		let mut any_overlap = false;

		let mut n = 0;
		for ssa in a.shape.sub_shape_iter() {
			let sa = OverlapCheckerSubItem {
				sub_shape: &ssa,
				pos: a.pos,
				offset: a.offset,
				rotation: a.rotation,
			};
			for ssb in b.shape.sub_shape_iter() {
				let sb = OverlapCheckerSubItem {
					sub_shape: &ssb,
					pos: b.pos,
					offset: b.offset,
					rotation: b.rotation,
				};
				let maybe_dr = if n < 1 {
					debug_renderer
				} else {
					&None
				};

				if OverlapChecker::do_sub_shapes_overlap( &sa, &sb, maybe_dr ) {
					any_overlap = true;
					// :TODO: we could early out here
				}

				n += 1;
			}			
		}
		false
	}

}
