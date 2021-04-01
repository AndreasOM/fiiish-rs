

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

	pub fn add( &self, o: &Vector2 ) -> Self {
		Self{
			x: self.x + o.x,
			y: self.y + o.y,
		}
	}


	pub fn scale_vector2( &self, o: &Vector2 ) -> Self {
		Self {
			x: self.x * o.x,
			y: self.y * o.y,
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
