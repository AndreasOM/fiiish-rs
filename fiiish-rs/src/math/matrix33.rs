use crate::math::Vector3;

#[derive(Debug)]
pub struct Matrix33 {
	pub x: Vector3,
	pub y: Vector3,
	pub z: Vector3,
}

impl Matrix33 {
	pub fn identity() -> Self {
		Self {
			x: Vector3::new( 1.0, 0.0, 0.0 ),
			y: Vector3::new( 0.0, 1.0, 0.0 ),
			z: Vector3::new( 0.0, 0.0, 1.0 ),
		}
	}

	pub fn z_rotation( a: f32 ) -> Self {
		let s = a.sin();
		let c = a.cos();
		Self {
			x: Vector3::new(   c,  -s, 0.0 ),
			y: Vector3::new(   s,   c, 0.0 ),
			z: Vector3::new( 0.0, 0.0, 1.0 ),
		}
	}

	pub fn mul_vector3( &self, v: &Vector3 ) -> Vector3 {
        let x = v.x;
        let y = v.y;
        let z = v.z;
        
        Vector3::new(
        	self.x.x * x + self.y.x * y + self.z.x * z,
        	self.x.y * x + self.y.y * y + self.z.y * z,
        	self.x.z * x + self.y.z * y + self.z.z * z,
        )
	}
}

