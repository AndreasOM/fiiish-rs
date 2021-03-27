
pub struct FiiishApp {
	count: isize,
	is_done: bool,
}

impl FiiishApp {
	pub fn new() -> Self {
		Self {
			count: 1000,
			is_done: false,
		}
	}

	pub fn setup( &mut self ) {

	}
	pub fn teardown( &mut self ) {
		// Note: teardown is currently not called
		// implement Drop if you really need cleanup, or just do it before returning true from is_done
	}

	pub fn is_done( &self ) -> bool {
//		println!("is_done {} <= 0", &self.count );
		self.is_done
	}

	pub fn update( &mut self ) {
//		println!("Update {} - 1", &self.count );
		self.count -= 1;
		if self.count <= 0 {
			// Note: Do all cleanup here
			self.is_done = true;
		}
	}

	pub fn render( &mut self ) {
//		println!("Render {}", &self.count );
	}
}
