
use crate::math::Vector2;
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
	cursor_pos: Vector2,

	click_positions: Vec< Vector2 >,
}

impl FiiishApp {
	pub fn new() -> Self {
		Self {
			count: 0,
			total_time: 0.0,
			is_done: false,
			renderer: None,
			cursor_pos: Vector2::zero(),
			click_positions: Vec::new(),
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

		self.cursor_pos.x = 2.0*wuc.mouse_pos.x - 1.0;
		self.cursor_pos.y = 2.0*wuc.mouse_pos.y - 1.0;

		if wuc.was_mouse_button_pressed( 0 ) {
			println!("Left Mouse Button was pressed!");
			let cp = self.cursor_pos;

			self.click_positions.push( cp );
		}
		if wuc.mouse_buttons[ 1 ] {
			println!("Middle Mouse Button is pressed! -> {}", self.click_positions.len());
			let cp = self.cursor_pos;

			for _ in 0..1000 {
				self.click_positions.push( cp );
			}
		}

		if self.count % 180 == 0 {
			let fps = self.count as f64 / self.total_time;
			println!("fps: {}", fps);
		}

		if wuc.is_escaped_pressed || wuc.is_space_pressed {
			self.is_done = true;
//			dbg!(&self);
		}
//		let next_frame_time = std::time::Instant::now() +
//        	std::time::Duration::from_nanos(4_000_000);	// use some time for update
		std::thread::sleep( std::time::Duration::new(0, 4_000_000)); // 1_000_000_000 ns in 1s
	}

	pub fn render_quad( renderer: &mut Renderer, pos: &Vector2, size: &Vector2 ) {
//		let mut pos = pos;
//		pos.x -= 0.5*size.x;
//		pos.y -= 0.5*size.y;

		let mut hs = *size;	// hs => half size
		hs.x = 0.5 * hs.x;
		hs.y = 0.5 * hs.y;

		let v0 = renderer.add_vertex( -hs.x + pos.x,  hs.y + pos.y ); // TopLeft
		let v1 = renderer.add_vertex( -hs.x + pos.x, -hs.y + pos.y ); // BottomLeft
		let v2 = renderer.add_vertex(  hs.x + pos.x, -hs.y + pos.y ); // BottomRight
		let v3 = renderer.add_vertex(  hs.x + pos.x,  hs.y + pos.y ); // TopRight
		renderer.add_triangle( v0, v1, v2 ); // TopLeft, BottomLeft, BottomRight
		renderer.add_triangle( v2, v3, v0 ); // BottomRight, TopRight, TopLeft		
	}


	pub fn render( &mut self ) {
//		println!("Render {}", &self.count );
		std::thread::sleep( std::time::Duration::new(0, 5_000_000)); // 1_000_000_000 ns in 1s
		match &mut self.renderer {
			Some( renderer ) => {
				renderer.begin_frame();
				let color = Color::from_rgba( 0.5 + 0.5*( self.total_time * 0.5 ).sin() as f32, 0.5, 0.5, 1.0 );
				renderer.clear( &color );

				for i in 0..100 {
					let s = 0.125;
					let fi = i as f32;
					let t = self.total_time as f32 + fi*1.01;
					let y = 0.2*t.sin() as f32;
					let x = 0.2*(t + 3.14*0.5).sin() as f32;
					let x = 2.0 * x;
					let y = 2.0 * y;

					let pos = Vector2::new( x, y );
					let size = Vector2::new( 2.0*s, 2.0*s );
					FiiishApp::render_quad( renderer, &pos, &size );
				}

				for cp in &self.click_positions {
					FiiishApp::render_quad( renderer, &cp, &Vector2::new( 0.1, 0.1 ) );
				}
				FiiishApp::render_quad( renderer, &self.cursor_pos, &Vector2::new( 0.1, 0.1 ) );

//				dbg!( &renderer );
				renderer.end_frame();
			},
			None => {},
		}
	}
}
