

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
		let initial_pos = pos;
		let mut pos = *pos;
		/*
		if let Some( g ) = font.find_glyph( 'j' as u8 ) {
			dbg!(g);
		}

		if let Some( g ) = font.find_glyph( 'b' as u8 ) {
			dbg!(g);
		}

		todo!("die");
		*/
		let mut bottom_y = f32::MAX;
		let mut top_y = f32::MIN;
		self.size.y = font.size();
		let mut line_count = 1;
		for c in text.bytes() {
			if c as u8 == 0x0a {
				pos.x = initial_pos.x;
				pos.y -= font.size();
				self.size.y += font.size();
				line_count += 1;
				continue;
			}
			if let Some( g ) = font.find_glyph( c as u8 ) {
//				println!("{} -> {:?}", c, g);
				let s = Vector2::new( g.width as f32, g.height as f32 );
				let y_offset = g.y_offset as f32;// * 260.0*5.0;
				let q = TextLayoutQuad {
					pos: Vector2::new( pos.x + 0.5*s.x, pos.y + 0.5*s.y - y_offset ),
					size: s,
					tex_mtx: g.matrix.into(),
				};
				self.quads.push( q );
				pos.x += g.advance as f32;

				if self.size.x < pos.x {
					self.size.x = pos.x;
				}
			}
		}

		if line_count > 1 {
			let y_fix = ( line_count - 1 ) as f32 * font.size();
			for q in self.quads.iter_mut() {
				q.pos.y += y_fix;
			}
		}
	}

	pub fn quads( &self ) -> &Vec< TextLayoutQuad > {
		&self.quads
	}

	pub fn size( &self ) -> &Vector2 {
		&self.size
	}

}
