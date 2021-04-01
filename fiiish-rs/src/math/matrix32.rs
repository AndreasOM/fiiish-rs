use crate::math::Vector2;
use crate::math::Matrix22;

#[derive(Debug,Copy,Clone)]
pub struct Matrix32 {
	rot: Matrix22,
	pos: Vector2,
}

impl Matrix32 {
	pub fn identity() -> Matrix32 {
		Self {
			rot: Matrix22::identity(),
			pos: Vector2::zero(),
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

