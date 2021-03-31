use super::effect_ids::EffectId;

use crate::math::{ Matrix44, Vector2 };
use crate::renderer::{
	Color,
	Effect,
	Renderer,
	Texture,
};
use crate::system::System;
use crate::system::filesystem_disk::FilesystemDisk;
use crate::system::filesystem_layered::FilesystemLayered;

use crate::window::Window;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::game::Game;

use super::mixel::Mixel;

#[derive(Debug)]
pub struct FiiishApp {
	count: isize,
	total_time: f64,
	is_done: bool,
	renderer: Option< Renderer >,
	cursor_pos: Vector2,

	click_positions: Vec< Vector2 >,
	system: System,

	size: Vector2,
	scaling: f32,

	game: Game,

	mixel: Mixel,
	mixel_enabled: bool,
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
			system: System::new(),

			size: Vector2::zero(),
			scaling: 1.0,

			game: Game::new(),

			mixel: Mixel::new(),
			mixel_enabled: false,
		}
	}

	pub fn setup( &mut self, window: &mut Window ) -> anyhow::Result<()> {
		// :TODO: make configurable via command line, or environment

		let cwd = std::env::current_dir()?;
		let cwd = cwd.to_string_lossy();
		let datadir = format!("{}/../fiiish-data", &cwd);
		let dfs1 = FilesystemDisk::new( &datadir );

		let datadir = format!("{}/../dummy-data", &cwd);
		let dfs2 = FilesystemDisk::new( &datadir );

		let mut lfs = FilesystemLayered::new();
		// Note: Filesystems will be searched last to first (lifo)
		lfs.add_filesystem( Box::new( dfs2 ) );
		lfs.add_filesystem( Box::new( dfs1 ) );

		self.system.set_default_filesystem( Box::new( lfs ) );

		// :TODO: load confiiguration

		let fs = self.system.default_filesystem_mut();
		dbg!(&fs);

//		let mut f = fs.open("test.txt");
		let mut f = fs.open("default_vs.glsl");
		println!("Testfile has size {}", f.size() );
		println!("Testfile has pos {}", f.pos() );
		f.set_pos( 10 );
		println!("Testfile has pos {}", f.pos() );
		f.set_pos( 0 );
		dbg!(&f);

		while !f.eof() {
			let c = f.read_u8();
			print!("{}", c as char);
		}
		println!("");

		println!("Testfile has pos {}", f.pos() );

		window.set_title("Fiiish! RS");
		let mut renderer = Renderer::new();
		renderer.setup( window, &mut self.system )?;

		renderer.register_effect( Effect::create( &mut self.system, EffectId::Default as u16  , "Default"  , "default_vs.glsl", "default_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::White as u16    , "White"    , "default_vs.glsl", "white_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::Textured as u16 , "Textured" , "textured_vs.glsl", "textured_fs.glsl" ) );

		renderer.register_texture( Texture::create( &mut self.system, "test_texture_1" ) );
		renderer.register_texture( Texture::create( &mut self.system, "test_texture_2" ) );
		renderer.register_texture( Texture::create( &mut self.system, "fish_swim0000" ) );

//		todo!("die");
		// setup sub parts
		self.game.setup( &mut self.system, &mut renderer );
		self.mixel.setup( &mut self.system, &mut renderer );

		self.renderer = Some( renderer );
		Ok(())
	}

	pub fn teardown( &mut self ) {
		// Note: teardown is currently not called
		// implement Drop if you really need cleanup, or just do it before returning true from is_done

		self.mixel.teardown();
		self.game.teardown();
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

		self.size = wuc.window_size;

		let scaling = 1024.0/self.size.y;
		self.scaling = 0.5*scaling;


		self.cursor_pos.x = self.scaling * wuc.window_size.x * ( 2.0*wuc.mouse_pos.x - 1.0 );
		self.cursor_pos.y = self.scaling * wuc.window_size.y * ( 2.0*wuc.mouse_pos.y - 1.0 );

		if wuc.was_mouse_button_pressed( 0 ) {
			println!("Left Mouse Button was pressed!");
			let cp = self.cursor_pos;

			self.click_positions.push( cp );
		}
		if wuc.mouse_buttons[ 1 ] {
//			println!("Middle Mouse Button is pressed! -> {}", self.click_positions.len());
			let cp = self.cursor_pos;

			for _ in 0..1 /*000*/ {
				self.click_positions.push( cp );
			}
		}

		if self.count % 180 == 0 {
			let fps = self.count as f64 / self.total_time;
			println!("fps: {}", fps);
		}

		if wuc.is_escaped_pressed { //|| wuc.is_space_pressed {
			self.is_done = true;
//			dbg!(&self);
		}
//		let next_frame_time = std::time::Instant::now() +
//        	std::time::Duration::from_nanos(4_000_000);	// use some time for update
//		std::thread::sleep( std::time::Duration::new(0, 4_000_000)); // 1_000_000_000 ns in 1s

		self.game.update( wuc );

		if wuc.was_key_pressed( 'm' as u8 ) {
			self.mixel_enabled = !self.mixel_enabled;
		}

		if self.mixel_enabled {
			self.mixel.update( wuc );
		}
	}

	pub fn render( &mut self ) {
//		println!("Render {}", &self.count );
//		std::thread::sleep( std::time::Duration::new(0, 5_000_000)); // 1_000_000_000 ns in 1s

		match &mut self.renderer {
			Some( renderer ) => {
				renderer.begin_frame();
				let color = Color::from_rgba( 0.5 + 0.5*( self.total_time * 0.5 ).sin() as f32, 0.5, 0.5, 1.0 );
				renderer.clear( &color );

				let scaling = self.scaling;
				let left = -self.size.x * scaling;
				let right = self.size.x * scaling;
				let top = self.size.y * scaling;
				let bottom = - self.size.y * scaling;
				let near = 1.0;
				let far = -1.0;

//				dbg!(&scaling);
//				dbg!(&top,&bottom);

				let mvp = Matrix44::ortho(
					left, right,
					bottom, top,
					near, far
				);

				renderer.set_mvp_matrix(
					&mvp
				);

//				renderer.use_effect( "Default" );
				renderer.use_effect( EffectId::Default as u16 );
				renderer.use_texture( "fish_swim0000" );

				// renderer.use_material( "rainbow" );
				for i in 0..100 {
					if i % 10 == 0 {
						renderer.use_effect( EffectId::Default as u16 );
					} else {
						renderer.use_effect( EffectId::Textured as u16 );
					}
					let s = 64.0;
					let fi = i as f32;
					let t = self.total_time as f32 + fi*1.01;
					let y = 0.2*t.sin() as f32;
					let x = 0.2*t.cos() as f32;
					let d = 1.0+(0.5+0.5*t.sin());
					let x = d*512.0*3.0 * x;
					let y = d*512.0*3.0 * y;

					let pos = Vector2::new( x, y );
					let size = Vector2::new( 2.0*s, 2.0*s );
					renderer.render_textured_quad_with_rotation( &pos, &size, -t*57.29577951289617186797 + 270.0 );
				}

				renderer.use_effect( EffectId::Default as u16 );
				for cp in &self.click_positions {
					renderer.render_quad( &cp, &Vector2::new( 64.0, 64.0 ) );
				}
				
				renderer.use_effect( EffectId::Textured as u16 );
				renderer.use_texture( "test_texture_1" );
				renderer.render_textured_quad( &Vector2::new( -256.0, 512.0 - 0.5*64.0 ), &Vector2::new( 64.0, 64.0 ) );
				renderer.use_texture( "test_texture_2" );
				renderer.render_textured_quad( &Vector2::new(  512.0 - 0.5*128.0, -512.0 + 0.5*128.0 ), &Vector2::new( 128.0, 128.0 ) );
//				renderer.use_texture( "test_texture_3" );

				renderer.use_effect( EffectId::Textured as u16 );
				renderer.use_texture( "fish_swim0000" );
				renderer.render_textured_quad( &self.cursor_pos, &Vector2::new( 128.0, 128.0 ) );

//				dbg!( &renderer );


				if self.mixel_enabled {
					self.mixel.render( renderer );
				}

				self.game.render( renderer );

				renderer.end_frame();
			},
			None => {},
		}
	}
}

