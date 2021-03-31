


#[derive(Copy,Clone)]
pub struct Matrix44 {
	m: [f32;16],
}

impl Matrix44 {
	pub fn new() -> Self {
		Self {
			m: [0.0;16],
		}
	}

	pub fn identity() -> Self {
		Self {
			m: [
				1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, 0.0,
				0.0, 0.0, 0.0, 1.0,
			]
		}
	}

	pub fn ortho( left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32 ) -> Self {
		let rpl = right + left;
		let rml = right - left;
		let tpb = top + bottom;
		let tmb = top - bottom;
		let fpn = far + near;
		let fmn = far - near;

		Self {
			m: [
				 2.0 / rml, 0.0       , 0.0       , 0.0,
				0.0       ,  2.0 / tmb, 0.0       , 0.0,
				0.0       , 0.0       ,  2.0 / fmn, 0.0,
				-rpl / rml, -tpb / tmb, -fpn / fmn, 1.0,
			]
		}
	}


	pub fn as_ptr(&self) -> *const f32 {
		self.m.as_ptr()
	}
}

impl std::fmt::Debug for Matrix44 {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		let m = &self.m;
		writeln!( f,"Matrix44:\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}",
			m[  0 ], m[  1 ], m[  2 ], m[  3 ],
			m[  4 ], m[  5 ], m[  6 ], m[  7 ],
			m[  8 ], m[  9 ], m[ 10 ], m[ 11 ],
			m[ 12 ], m[ 13 ], m[ 14 ], m[ 15 ],
		)
	}
}
