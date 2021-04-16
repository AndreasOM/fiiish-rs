use crate::math::{
	Vector2,
	Matrix22,
	Matrix33,
};

#[derive(Copy,Clone)]
pub struct Matrix32 {
	pub rot: Matrix22,
	pub pos: Vector2,
}

impl Matrix32 {
	pub fn identity() -> Self {
		Self {
			rot: Matrix22::identity(),
			pos: Vector2::zero(),
		}
	}

	pub fn scaling( scale: f32 ) -> Self {
		Self {
			rot: Matrix22::scaling(scale),
			pos: Vector2::zero(),
		}
	}
	pub fn translation( t: &Vector2 ) -> Self {
		Self {
			rot: Matrix22::identity(),
			pos: *t,
		}
	}
	pub fn scaling_xy( x: f32, y: f32 ) -> Self {
		Self {
			rot: Matrix22::scaling_xy( x, y ),
			pos: Vector2::zero(),
		}
	}

	// :UNTESTED:
	pub fn from_matrix33( m: &Matrix33 ) -> Self {
		Self {
			rot: Matrix22::new(
				&Vector2::new( m.x.x, m.x.y ),
				&Vector2::new( m.y.x, m.y.y ),
			),
			pos: Vector2::new( m.x.z, m.y.z ),
		}	
	}

	pub fn mul_vector2( &self, v: &Vector2 ) -> Vector2 {
        let x = v.x;
        let y = v.y;

        Vector2::new(
	        self.rot.x.x * x + self.rot.y.x * y + self.pos.x,
	        self.rot.x.y * x + self.rot.y.y * y + self.pos.y,
        )
	}

}

impl From<[f32;6]> for Matrix32 {
	fn from(m: [f32;6]) -> Self { 
		Self {
			rot: Matrix22::new(
				&Vector2::new( m[ 0 ], m[ 3 ] ),
				&Vector2::new( m[ 1 ], m[ 4 ] ),
			),
			pos: Vector2::new( m[ 2 ], m[ 5 ] ),
		}
	}
}

impl std::fmt::Debug for Matrix32 {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!( f,"Matrix32:\n{} {} {}\n{} {} {}",
			self.rot.x.x, self.rot.x.y, self.pos.x,
			self.rot.y.x, self.rot.y.y, self.pos.y,
		)
	}
}

