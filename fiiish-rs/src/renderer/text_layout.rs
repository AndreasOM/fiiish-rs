

use crate::math::Vector2;
use crate::math::Matrix32;
use crate::renderer::{ Color, Font };
use crate::debug_renderer;

#[derive(Debug)]
pub struct TextLayoutQuad {
	pub pos: Vector2,
	pub size: Vector2,
	pub tex_mtx: Matrix32,
//	pub vertices: [Vector2;4],
//	tex_coords: [Vector2;4],
}
#[derive(Debug)]
pub struct TextLayout {
	quads: Vec< TextLayoutQuad >,
	size: Vector2,
}

impl TextLayout {
	pub fn new() -> Self {
		Self {
			quads: Vec::new(),
			size: Vector2::zero(),
		}
	}

	pub fn layout( &mut self, font: &Font, pos: &Vector2, text: &str ) {
		let mut pos = *pos;
		/*
		if let Some( g ) = font.find_glyph( 'a' as u8 ) {
			dbg!(g);
		}

		if let Some( g ) = font.find_glyph( 'b' as u8 ) {
			dbg!(g);
		}

		todo!("die");
		*/
		let mut bottom_y = f32::MAX;
		let mut top_y = f32::MIN;
		for c in text.bytes() {
			if let Some( g ) = font.find_glyph( c as u8 ) {
//				println!("{} -> {:?}", c, g);
				// :TODO: use y_offset

/*
				let xl = pos.x + g.x as f32;
				let xr = xl + g.width as f32;
				let yb = pos.y + g.y as f32;// + g.y_offset;
				let yt = yb + g.height as f32;
*/
				let s = Vector2::new( g.width as f32, g.height as f32 );
				let y_offset = g.y_offset as f32;// * 260.0*5.0;
				let q = TextLayoutQuad {
//					pos: pos.add( &Vector2::new( g.x as f32, g.y as f32 ) ),
//					pos: pos.add( &Vector2::new( 0.0, g.y_offset as f32 * -260.0 ) ).sub( &s.scaled( 0.5 ) ),
					pos: Vector2::new( pos.x + 0.5*s.x, pos.y + 0.5*s.y - y_offset ),
					size: s,
					tex_mtx: g.matrix.into(),
					/*
					vertices: [
							Vector2::new( xl, yt ),
							Vector2::new( xl, yb ),
							Vector2::new( xr, yb ),
							Vector2::new( xr, yt ),
							]
					*/
				};
				/*
				debug_renderer::debug_renderer_add_frame(
					&q.pos,
					&q.size,
					3.0,
					&Color::red(),
				);
				*/
//				dbg!(&q);
//				dbg!(&pos);
				if q.pos.y < bottom_y {
					bottom_y = q.pos.y;
				}
				if q.pos.y + q.size.y > top_y {
					top_y = q.pos.y + q.size.y;
				}
				self.quads.push( q );
				pos.x += g.advance as f32;

				self.size.x = pos.x;
			}
		}
		// :TODO: fixe for center line
		self.size.y = top_y - bottom_y;
//		todo!("die");
	}

	pub fn quads( &self ) -> &Vec< TextLayoutQuad > {
		&self.quads
	}

	pub fn size( &self ) -> &Vector2 {
		&self.size
	}

}
