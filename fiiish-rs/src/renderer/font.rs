

use crate::system::System;

#[derive(Debug,Copy,Clone)]
pub struct Glyph {
	pub codepoint:	u8,
	pub width:		u32,
	pub height:		u32,
	pub x:			u32,
	pub y:			u32,
	pub advance:	u16,
	pub y_offset:	f32,
	pub matrix:		[f32;6],
}

impl Glyph {
	pub fn new( codepoint: u8, width: u32, height: u32 ) -> Glyph {
		Glyph {
			codepoint: codepoint,
			width: width,
			height: height,
			x: 0,
			y: 0,
			advance: 0,
			y_offset: 0.0,
			matrix: [0.0;6],
		}
	}
	fn recalc_from_matrix( &mut self, texsize: u32 ) {
		self.width	= ( self.matrix[ 0*3 + 0 ] * texsize as f32 ).trunc() as u32;
		self.height	= ( self.matrix[ 1*3 + 1 ] * texsize as f32 ).trunc() as u32;
		self.x		= ( self.matrix[ 0*3 + 2 ] * texsize as f32 ).trunc() as u32;
		self.y		= ( self.matrix[ 1*3 + 2 ] * texsize as f32 ).trunc() as u32;

		// :HACK: since omt-font might be broken
		if self.codepoint != 32 { // leave SPACE alone
			self.advance = self.width as u16;
		}
		self.y_offset = self.y_offset * texsize as f32;
	}

}


#[derive(Debug)]
pub struct Font {
	glyphs: Vec<Glyph>,
	size: u16,
	name: String,
}

impl Font {
	pub fn create( system: &mut System, name: &str ) -> Self {
		let mut f = Font::new( name );
		f.load( system, name );

		f
	}
	pub fn recalc_from_matrix( &mut self, texsize: u32 ) {
		let mut hi_code = 0;
		let mut hi_h = 0;
		for g in &mut self.glyphs {
			g.recalc_from_matrix( texsize );
			let y = ( g.height as f32 - g.y_offset ) as u32;
			if y > hi_h {
				hi_h = y;
				hi_code = g.codepoint;
//				dbg!(&g);
			}
		}

//		dbg!(&hi_code, &hi_h, self.size);
		// :HACK: cheat, since the font converter doesn't give us what we need
		self.size = hi_h as u16;
//		todo!("die");
	}

	fn new( name: &str ) -> Self {
		Self {
			glyphs: Vec::new(),
			size: 0,
			name: name.to_owned(),
		}
	}

	fn load( &mut self, system: &mut System, name: &str ) -> bool {
		let filename = format!("{}{}", name, ".omfont" );
		let mut f = system.default_filesystem_mut().open( &filename );
		if f.is_valid() {
			println!("Loading font from {}", &filename );
		}
		let chunk_magic = [ 0x4fu8, 0x4d, 0x46, 0x4f, 0x4e, 0x54, ];
		for m in &chunk_magic {
			let b = f.read_u8();
			if b != *m {
				return false; //Err( OmError::Generic("Broken chunk magic".to_string() ) );
			}
		}
		let version = f.read_u32();
		if version != 2 {
			return false; //Err( OmError::Generic("Unsupported version".to_string() ) );
		}

		self.size = f.read_u16();
		let count = f.read_u16();

		let mut codepoints = Vec::new();

		for _ in 0..count {
			let codepoint = f.read_u32();
			codepoints.push( codepoint );
		}

		for c in 0..count {
			let codepoint = codepoints[ c as usize ];
			let mut glyph = Glyph::new( codepoint as u8, 0, 0 );
			for m in &mut glyph.matrix {
				*m = f.read_f32();
			}
			glyph.advance = f.read_f32() as u16;
			if codepoint == 32 { //SPACE	// :HACK: since space is written with an advance of 0
				glyph.advance = self.size / 4;
				dbg!(&glyph);
			}
			glyph.y_offset = f.read_f32();
			self.glyphs.push( glyph );
		}


		true
	}

	pub fn find_glyph( &self, codepoint: u8 ) -> Option< &Glyph > {
		for g in self.glyphs.iter() {
			if g.codepoint == codepoint {
				return Some( g );
			}
		}
		None
	}

	pub fn name( &self ) -> &str {
		&self.name
	}

	pub fn size( &self ) -> f32 {
		self.size as f32
	}
}
