
use std::cell::RefCell;

use crate::fiiish::{ Shape, SubShape };
use oml_game::math::{
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

	fn get_side( s: &Vector2, e: &Vector2, p: &Vector2, debug_renderer: &Option < RefCell< DebugRenderer >  > ) -> Side {
		let sv = e.sub( &s );
		let sp = p.sub( &s );

		let cp = sp.cross( &sv );
		let is_left = cp.x > 0.0;
		if let Some( debug_renderer ) = debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			let color_l = Color::green();
			let color_r = Color::red();

			let color = Color::green();
			let mut color_dim = color;
			color_dim.a *= 0.25;

			let off = s;

			debug_renderer.add_line( &Vector2::zero().add( &off ), &sv.add( &off ), 5.0, &color );
			if is_left {
				debug_renderer.add_line( &Vector2::zero().add( &off ), &sp.add( &off ), 5.0, &color_l );
			} else {
				debug_renderer.add_line( &Vector2::zero().add( &off ), &sp.add( &off ), 5.0, &color_r );							
			}
			debug_renderer.add_line( &e, &sv.add( &off ), 1.0, &color_dim );
			debug_renderer.add_line( &p, &sv.add( &off ), 1.0, &color_dim );
		}

		if is_left { Side::Left } else { Side::Right }
	}

	// :TODO: fix name to "is any point in a on the same side of all segments in b"" ... or something like that
	fn any_points_in_polygon( points_a: &Vec< Segment >, polygon_b: &Vec< Segment >, _debug_renderer: &Option < RefCell< DebugRenderer >  > ) -> bool {
		let mut any_inside = false;

		for sa in points_a.iter() {
			let p = sa.start;
			let mut last_side = Side::None;

			let mut all_same_side = true;
			for sb in polygon_b.iter() {
				let side = OverlapChecker::get_side( &sb.start, &sb.end, &p, &None /*debug_renderer*/ );
				if last_side != Side::None {
					if last_side != side {
						all_same_side = false;
						// break;
					}
				};
				last_side = side;
			}
			if all_same_side {
				any_inside = true;
			}
		}

		any_inside
	}

	fn do_sub_shapes_overlap( a: &OverlapCheckerSubItem, b: &OverlapCheckerSubItem, debug_renderer: &Option < RefCell< DebugRenderer >  > ) -> bool {
		let segments_a = OverlapChecker::get_segments( &a );
		let segments_b = OverlapChecker::get_segments( &b );

		let any_inside = OverlapChecker::any_points_in_polygon( &segments_a, &segments_b, debug_renderer )
							|| OverlapChecker::any_points_in_polygon( &segments_b, &segments_a, debug_renderer );

		if let Some( debug_renderer ) = debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();

			let result_color = if any_inside {
				Color::from_rgba( 0.8, 0.2, 0.8, 0.5 )
//				Color::white()
			} else {
				Color::from_rgba( 0.2, 0.8, 0.2, 0.5 )
//				Color::blue()
			};

			for s in segments_a.iter() {
				debug_renderer.add_line( &s.start, &s.end, 15.0, &result_color );
			}
			for s in segments_b.iter() {
				debug_renderer.add_line( &s.start, &s.end, 15.0, &result_color );
			}
		}

		any_inside
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
				let maybe_dr = if n < 100 {
					debug_renderer
				} else {
					&None
				};

				if OverlapChecker::do_sub_shapes_overlap( &sa, &sb, maybe_dr ) {
					any_overlap = true;
					break;
					// :TODO: we could early out here
				}

				n += 1;
			}
			if any_overlap {
				break;
			}
		}
		any_overlap
	}

}
