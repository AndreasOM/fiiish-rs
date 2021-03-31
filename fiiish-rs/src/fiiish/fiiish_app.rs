use super::effect_ids::EffectId;

use crate::math::{ Matrix44, Vector2 };
use crate::renderer::{
	Color,
	Effect,
	Renderer,
	Texture,
	TextureAtlas,
};
use crate::system::System;
use crate::system::filesystem_disk::FilesystemDisk;
use crate::system::filesystem_layered::FilesystemLayered;

use crate::fiiish::app_update_context::AppUpdateContext;

use crate::window::Window;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::game::Game;

use super::demo::Demo;
use super::mixel::Mixel;

#[derive(Debug)]
pub struct FiiishApp {
	count: isize,
	total_time: f64,
	is_done: bool,
	renderer: Option< Renderer >,
	cursor_pos: Vector2,

	system: System,

	size: Vector2,
	scaling: f32,

	game: Game,

	demo: Demo,
	demo_enabled: bool,

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
			system: System::new(),

			size: Vector2::zero(),
			scaling: 1.0,

			game: Game::new(),

			demo: Demo::new(),
			demo_enabled: false,
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


		TextureAtlas::load_all( &mut self.system, &mut renderer, "game-atlas-%d" );
		renderer.register_texture( Texture::create( &mut self.system, "test_texture_1" ) );
		renderer.register_texture( Texture::create( &mut self.system, "test_texture_2" ) );
		renderer.register_texture( Texture::create( &mut self.system, "cursor" ) );

//		todo!("die");
		// setup sub parts
		self.game.setup( &mut self.system, &mut renderer );


		self.demo.setup( &mut self.system, &mut renderer );
		self.mixel.setup( &mut self.system, &mut renderer );

		self.renderer = Some( renderer );
		Ok(())
	}

	pub fn teardown( &mut self ) {
		// Note: teardown is currently not called
		// implement Drop if you really need cleanup, or just do it before returning true from is_done

		self.mixel.teardown();
		self.demo.teardown();

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

		let mut auc = AppUpdateContext::new()
					.set_time_step( wuc.time_step )
					.set_cursor_pos( &self.cursor_pos )
					.set_wuc( &wuc );

		if self.count % 180 == 0 {
			let fps = self.count as f64 / self.total_time;
			println!("fps: {}", fps);
		}

		if wuc.is_escaped_pressed { //|| wuc.is_space_pressed {
			self.is_done = true;
		}

		self.game.update( wuc );


		if wuc.was_key_pressed( 't' as u8 ) {
			self.demo_enabled = !self.demo_enabled;
		}
		if self.demo_enabled {
			self.demo.update( &mut auc );
		}

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

				let mvp = Matrix44::ortho(
					left, right,
					bottom, top,
					near, far
				);

				renderer.set_mvp_matrix(
					&mvp
				);

				if self.demo_enabled {
					self.demo.render( renderer );
				}
				if self.mixel_enabled {
					self.mixel.render( renderer );
				}

				self.game.render( renderer );

				// :DEBUG: render atlas
				renderer.use_effect( EffectId::Textured as u16 );
				renderer.use_texture( "game-atlas-0" );
				renderer.render_textured_quad( &Vector2::zero(), &Vector2::new( 1024.0, 1024.0 ) );

				renderer.use_effect( EffectId::Textured as u16 );
				renderer.use_texture( "cursor" );
				renderer.render_textured_quad( &self.cursor_pos, &Vector2::new( 128.0, 128.0 ) );

				renderer.end_frame();
			},
			None => {},
		}
	}

}

