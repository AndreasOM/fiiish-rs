
use crate::renderer::{
	Color,
	Renderer,
};
use crate::window::Window;
use crate::window_update_context::WindowUpdateContext;

#[derive(Debug)]
pub struct FiiishApp {
	count: isize,
	total_time: f64,
	is_done: bool,
	renderer: Option< Renderer >,
}

impl FiiishApp {
	pub fn new() -> Self {
		Self {
			count: 0,
			total_time: 0.0,
			is_done: false,
			renderer: None,
		}
	}

	pub fn setup( &mut self, window: &mut Window ) -> anyhow::Result<()> {
		window.set_title("Fiiish! RS");
		let mut renderer = Renderer::new();
		renderer.setup( window )?;
		self.renderer = Some( renderer );
		Ok(())
	}

	pub fn teardown( &mut self ) {
		// Note: teardown is currently not called
		// implement Drop if you really need cleanup, or just do it before returning true from is_done

		self.renderer = None;
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
		std::thread::sleep( std::time::Duration::new(0, 4_000_000)); // 1_000_000_000 ns in 1s
	}

	pub fn render( &mut self ) {
//		println!("Render {}", &self.count );
		std::thread::sleep( std::time::Duration::new(0, 5_000_000)); // 1_000_000_000 ns in 1s
		match &mut self.renderer {
			Some( renderer ) => {
				renderer.begin_frame();
				let color = Color::from_rgba( 0.5 + 0.5*( self.total_time * 0.5 ).sin() as f32, 0.5, 0.5, 1.0 );
				renderer.clear( &color );
				let y = 0.2*self.total_time.sin() as f32;
				let x = 0.2*(self.total_time*1.1).sin() as f32;

				let v0 = renderer.add_vertex(  0.0 + x,  0.5 + y );
				let v1 = renderer.add_vertex( -0.5 + x, -0.5 + y );
				let v2 = renderer.add_vertex(  0.5 + x, -0.5 + y );
				renderer.add_triangle( v0, v1, v2 );

//				dbg!( &renderer );
				renderer.end_frame();
			},
			None => {},
		}
	}
}
