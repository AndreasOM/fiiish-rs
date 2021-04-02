use crate::math::Vector2;

#[derive(Copy,Clone)]
pub struct Matrix22 {
	pub x: Vector2,
	pub y: Vector2,
}

impl Matrix22 {
	pub fn new(x: &Vector2, y: &Vector2) -> Self {
		Self {
			x: *x,
			y: *y,
		}
	}

	pub fn identity() -> Self {
		Self {
			x: Vector2::new( 1.0, 0.0 ),
			y: Vector2::new( 0.0, 1.0 ),
		}
	}

	pub fn scaling( scale: f32 ) -> Self {
		Self {
			x: Vector2::new( scale, 0.0 ),
			y: Vector2::new( 0.0, scale ),
		}		
	}

	pub fn scaling_xy( x: f32, y: f32 ) -> Self {
		Self {
			x: Vector2::new( x, 0.0 ),
			y: Vector2::new( 0.0, y ),
		}		
	}

	pub fn z_rotation( a: f32 ) -> Self {
		let s = a.sin();
		let c = a.cos();
		Self {
			x: Vector2::new(   c,  -s ),
			y: Vector2::new(   s,   c ),
		}
	}

	pub fn mul_vector2( &self, v: &Vector2 ) -> Vector2 {
        let x = v.x;
        let y = v.y;
        
        Vector2::new(
        	self.x.x * x + self.y.x * y,
        	self.x.y * x + self.y.y * y,
        )
	}
}

impl std::fmt::Debug for Matrix22 {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!( f,"Matrix22:\n{} {}\n{} {}",
			self.x.x, self.x.y,
			self.y.x, self.y.y,
		)
	}
}
