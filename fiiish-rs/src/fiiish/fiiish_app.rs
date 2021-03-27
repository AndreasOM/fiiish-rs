use crate::window_update_context::WindowUpdateContext;

#[derive(Debug)]
pub struct FiiishApp {
	count: isize,
	total_time: f64,
	is_done: bool,
}

impl FiiishApp {
	pub fn new() -> Self {
		Self {
			count: 0,
			total_time: 0.0,
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

	pub fn update( &mut self, wuc: &mut WindowUpdateContext ) {
//		println!("Update {}", &wuc.time_step );
		self.count += 1;
		self.total_time += wuc.time_step;

		if self.count % 180 == 0 {
			let fps = self.count as f64 / self.total_time;
			println!("fps: {}", fps);
		}

		if wuc.is_escaped_pressed || wuc.is_space_pressed {
			self.is_done = true;
			dbg!(&self);
		}
//		let next_frame_time = std::time::Instant::now() +
//        	std::time::Duration::from_nanos(4_000_000);	// use some time for update
		std::thread::sleep( std::time::Duration::new(0, 4_000_000)) // 1_000_000_000 ns in 1s
	}

	pub fn render( &mut self ) {
//		println!("Render {}", &self.count );
		std::thread::sleep( std::time::Duration::new(0, 5_000_000)) // 1_000_000_000 ns in 1s
	}
}
