

#[derive(Copy,Clone)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}


impl Vector2 {
	pub fn new( x: f32, y: f32 ) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn zero() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
		}
	}

	pub fn normalized( &self ) -> Self {
		let l = self.length();
		Self {
			x: self.x / l,
			y: self.y / l,
		}
	}

	pub fn scaled( &self, factor: f32 ) -> Self {
		Self {
			x: self.x * factor,
			y: self.y * factor,
		}
	}

	pub fn cross( &self, other: &Vector2 ) -> Self {
		Self {
			x: self.y*other.x - self.x*other.y,
			y: self.x*other.y - self.y*other.x,
		}	
	}

	// :TODO: seems to be duplicated from scale_vector2
	pub fn scaled_vector2( &self, factor: &Vector2 ) -> Self {
		Self {
			x: self.x * factor.x,
			y: self.y * factor.y,
		}
	}


	pub fn add( &self, o: &Vector2 ) -> Self {
		Self{
			x: self.x + o.x,
			y: self.y + o.y,
		}
	}

	pub fn sub( &self, o: &Vector2 ) -> Self {
		Self{
			x: self.x - o.x,
			y: self.y - o.y,
		}
	}

	pub fn length( &self ) -> f32 {
		let sql = self.x * self.x + self.y * self.y;
		sql.sqrt()
	}

	pub fn scale_vector2( &self, o: &Vector2 ) -> Self {
		Self {
			x: self.x * o.x,
			y: self.y * o.y,
		}
	}
}

impl From< ( f32, f32 ) > for Vector2 {
	fn from( t: ( f32, f32 ) ) -> Self {
		Self {
			x: t.0,
			y: t.1,
		}
	}
}

impl std::fmt::Debug for Vector2 {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!( f,"Vector2:\n{} {}",
			self.x, self.y,
		)
	}
}
