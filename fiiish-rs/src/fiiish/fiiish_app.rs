
use std::rc::Rc;
use std::cell::RefCell;

use super::effect_ids::EffectId;
use crate::fiiish::layer_ids::LayerId;
use crate::fiiish::font_ids::FontId;
use crate::math::{ Matrix44, Vector2 };
use crate::renderer::{
	Color,
	Effect,
	Renderer,
	Texture,
	TextureAtlas,
};
use crate::system::System;
//use crate::system::filesystem::Filesystem;
use crate::system::filesystem_archive::FilesystemArchive;
use crate::system::filesystem_disk::FilesystemDisk;
use crate::system::filesystem_layered::FilesystemLayered;
use crate::system::filesystem_memory::FilesystemMemory;

use crate::fiiish::app_update_context::AppUpdateContext;

use crate::window::Window;
use crate::window_update_context::WindowUpdateContext;

use crate::fiiish::game::Game;
use crate::fiiish::GameUi;

use crate::DebugRenderer;

use crate::debug_renderer;

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
	viewport_size: Vector2,
	scaling: f32,

	game: Rc< RefCell< Game > >,
	game_ui: GameUi,

	demo: Demo,
	demo_enabled: bool,

	mixel: Mixel,
	mixel_enabled: bool,

	debug_renderer: Rc< Option< RefCell< DebugRenderer >  > >,
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
			viewport_size: Vector2::zero(),
			scaling: 1.0,

			game: Rc::new( RefCell::new( Game::new() ) ),
			game_ui: GameUi::new( ),

			demo: Demo::new(),
			demo_enabled: false,
			mixel: Mixel::new(),
			mixel_enabled: false,

			debug_renderer: Rc::new( None ),
		}
	}

	fn add_filesystem_disk( &mut self, lfs: &mut FilesystemLayered, path: &str ) {
		let cwd = std::env::current_dir().unwrap();
		let cwd = cwd.to_string_lossy();

		let datadir = format!("{}/{}", &cwd, &path);
		let dfs = FilesystemDisk::new( &datadir );
		lfs.add_filesystem( Box::new( dfs ) );
	}
	fn add_pakfile_from_data( &mut self, lfs: &mut FilesystemLayered, name: &str, data: &Vec< u8 > ) {
		if data.len() > 0 {
			let mut mfs = FilesystemMemory::new();
			let mut omar_file = mfs.open_from_data( name, &data.to_vec() );

			let afs = FilesystemArchive::new_from_file( name, &mut omar_file );
			lfs.add_filesystem( Box::new( afs ) );
		}
	}

	// without archive
	#[cfg(not(fiiish_with_fiiish_omar))]
	fn get_fiiish_data_omar_data( ) -> &'static [u8] {
		println!("Loading fiiish-data from disk.");
		&[0;0]
	}
	// with archive
	#[cfg(fiiish_with_fiiish_omar)]
	fn get_fiiish_data_omar_data( ) -> &'static [u8] {
		println!("Loading fiiish-data from memory.");
		include_bytes!("../../fiiish-data.omar")
	}

	#[cfg(not(fiiish_with_dummy_omar))]
	fn get_dummy_data_omar_data( ) -> &'static [u8] {
		println!("Loading dummy-data from disk.");
		&[0;0]
	}

	#[cfg(fiiish_with_dummy_omar)]
	fn get_dummy_data_omar_data( ) -> &'static [u8] {
		println!("Loading dummy-data from memory.");
		include_bytes!("../../dummy-data.omar")
	}

	pub fn setup( &mut self, window: &mut Window ) -> anyhow::Result<()> {
		// :TODO: make configurable via command line, or environment

		// new filesytem based on linked in data

		let mut lfs = FilesystemLayered::new();
		// Note: Filesystems will be searched last to first (lifo)

		self.add_pakfile_from_data( &mut lfs, "dummy-data.omar", &FiiishApp::get_dummy_data_omar_data().to_vec() );
		self.add_pakfile_from_data( &mut lfs, "fiiish-data.omar", &FiiishApp::get_fiiish_data_omar_data().to_vec() );

		// check local files first for faster development (and easier modding)
		self.add_filesystem_disk( &mut lfs, "../dummy-data" );
		self.add_filesystem_disk( &mut lfs, "../fiiish-data" );

		self.system.set_default_filesystem( Box::new( lfs ) );

		// :TODO: load confiiguration

		let fs = self.system.default_filesystem_mut();
		dbg!(&fs);


		window.set_title("Fiiish! RS");
		let mut renderer = Renderer::new();
		renderer.setup( window, &mut self.system )?;

		renderer.register_effect( Effect::create( &mut self.system, EffectId::Default as u16  , "Default"  , "default_vs.glsl", "default_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::White as u16    , "White"    , "default_vs.glsl", "white_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::Colored as u16    , "Colored"    , "colored_vs.glsl", "colored_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::Textured as u16 , "Textured" , "textured_vs.glsl", "textured_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::ColoredTextured as u16 , "ColoredTextured" , "coloredtextured_vs.glsl", "coloredtextured_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::Background as u16 , "Background" , "background_vs.glsl", "background_fs.glsl" ) );
		renderer.register_effect( Effect::create( &mut self.system, EffectId::FontColored as u16 , "FontColored" , "fontcolored_vs.glsl", "fontcolored_fs.glsl" ) );

		TextureAtlas::load_all( &mut self.system, &mut renderer, "game-atlas-%d" );
		TextureAtlas::load_all( &mut self.system, &mut renderer, "gui-atlas-%d" );

		renderer.register_texture( Texture::create( &mut self.system, "test_texture_1" ) );
		renderer.register_texture( Texture::create( &mut self.system, "test_texture_2" ) );
		renderer.register_texture( Texture::create( &mut self.system, "cursor" ) );

		renderer.load_font( &mut self.system, FontId::Default as u8, "pink" );
		renderer.load_font( &mut self.system, FontId::Huge as u8, "pink_huge" );

//		todo!("die");
		// setup sub parts
		let window_size = Vector2::new( 1024.0, 1024.0 ); // :TODO: get from window
		self.game.borrow_mut().setup( &mut self.system, &mut renderer );
//		self.game.set_size( &window_size );

		self.game_ui.setup( &mut self.system, &mut renderer, &mut self.game );
		self.game_ui.set_size( &window_size );

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

		self.game_ui.teardown();
		self.game.borrow_mut().teardown();
		self.renderer = None;
	}

	pub fn is_done( &self ) -> bool {
//		println!("is_done {} <= 0", &self.count );
		self.is_done
	}

	pub fn update( &mut self, wuc: &mut WindowUpdateContext ) {
//		println!("Update {}", &wuc.time_step );
		if wuc.was_key_pressed( 'i' as u8 ) {
			if self.debug_renderer.is_none() {
				self.debug_renderer = Rc::new( Some( RefCell::new(
											DebugRenderer::new(
												LayerId::DebugRenderer as u8,
												EffectId::Colored as u16
											)
										) ) );
//				let game = self.game.borrow_mut();
				self.game.borrow_mut().enable_debug_renderer( &self.debug_renderer );
				self.game_ui.enable_debug_renderer( &self.debug_renderer );
			} else {
				self.debug_renderer = Rc::new( None );
				self.game.borrow_mut().disable_debug_renderer();
				self.game_ui.enable_debug_renderer( &self.debug_renderer );
			}
		}

		if wuc.was_key_pressed( 'j' as u8 ) {
			debug_renderer::debug_renderer_toggle(
				LayerId::DebugRenderer as u8,
				EffectId::Colored as u16
			);			
		}

		if let Some( debug_renderer ) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.begin_frame();
		}

		debug_renderer::debug_renderer_begin_frame();


		self.count += 1;
		self.total_time += wuc.time_step;

		self.viewport_size = wuc.window_size;

		let scaling = 1024.0/self.viewport_size.y;
		self.scaling = 1.0 * scaling;	// !!! Do not tweak here

		self.size.x = ( self.scaling ) * self.viewport_size.x;
		self.size.y = ( self.scaling ) * self.viewport_size.y;

		// :TODO: only call on change
		self.game_ui.set_size( &self.size );

//		dbg!(&self.viewport_size);
//		dbg!(&self.size);

		self.cursor_pos.x = 0.5 * self.scaling * wuc.window_size.x * ( 2.0*wuc.mouse_pos.x - 1.0 );
		self.cursor_pos.y = 0.5 * self.scaling * wuc.window_size.y * ( 2.0*wuc.mouse_pos.y - 1.0 );

		if let Some( debug_renderer ) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.add_line( &self.cursor_pos, &Vector2::zero(), 3.0, &Color::white() );
		}


//		dbg!( &wuc.mouse_pos );
//		dbg!( &self.cursor_pos );

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

		self.game.borrow_mut().update( wuc, &mut auc );
		self.game_ui.update( wuc, &mut auc );


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

		if let Some( debug_renderer ) = &*self.debug_renderer {
			let mut debug_renderer = debug_renderer.borrow_mut();
			debug_renderer.end_frame();
		}

	}

	pub fn render( &mut self ) {
//		println!("Render {}", &self.count );
//		std::thread::sleep( std::time::Duration::new(0, 5_000_000)); // 1_000_000_000 ns in 1s

		match &mut self.renderer {
			Some( renderer ) => {
//				dbg!(&self.size);
				renderer.set_size( &self.size );
				renderer.set_viewport( &Vector2::zero(), &self.viewport_size );
				renderer.begin_frame();
				let color = Color::from_rgba( 0.5 + 0.5*( self.total_time * 0.5 ).sin() as f32, 0.5, 0.5, 1.0 );
				renderer.clear( &color );

//				let scaling = self.scaling * 0.5;
				let scaling = 0.5;
//				dbg!(&scaling);
				let left = -self.size.x * scaling;
				let right = self.size.x * scaling;
				let top = self.size.y * scaling;
				let bottom = - self.size.y * scaling;
				let near = 1.0;
				let far = -1.0;

//				dbg!(&top,&bottom);

				let mvp = Matrix44::ortho(
					left, right,
					bottom, top,
					near, far
				);

//				dbg!(&mvp);

				renderer.set_mvp_matrix(
					&mvp
				);

				if self.demo_enabled {
					self.demo.render( renderer );
				}
				if self.mixel_enabled {
					self.mixel.render( renderer );
				}

				self.game.borrow_mut().render( renderer );
				self.game_ui.render( renderer );

				// :DEBUG: render atlas
				/*
				renderer.use_effect( EffectId::Textured as u16 );
				renderer.use_texture( "game-atlas-0" );
				renderer.render_textured_quad( &Vector2::zero(), &Vector2::new( 1024.0, 1024.0 ) );
				*/
				renderer.use_effect( EffectId::Textured as u16 );
				renderer.use_texture( "cursor" );
				renderer.render_textured_quad( &self.cursor_pos, &Vector2::new( 128.0, 128.0 ) );

				if let Some( debug_renderer ) = &*self.debug_renderer {
					let debug_renderer = debug_renderer.borrow();
					debug_renderer.render( renderer );
				}

//				renderer.use_texture( "pink" );
//				renderer.use_texture( "cursor" );
//				renderer.render_textured_quad( &Vector2::new( -64.0, 0.0 ), &Vector2::new( 128.0, 128.0 ) );

				renderer.use_effect( EffectId::FontColored as u16 );
//				renderer.use_font( FontId::Default as u8 );
				renderer.use_font( FontId::Huge as u8 );
				renderer.print( &Vector2::new( -800.0*1.0, 0.0 ), "abcdefghijklmnopqrstuvwxyz Test Text -=_" );
//				renderer.print( &Vector2::new( -800.0*0.0, 0.0 ), "deqr_h-=^\"Bhijklmn" );
//				renderer.print( &Vector2::new( -800.0*0.0, 0.0 ), "jk" );
				renderer.use_font( FontId::Default as u8 );

//				renderer.render_textured_quad( &Vector2::new( 64.0, 0.0 ), &Vector2::new( 128.0, 128.0 ) );

//				debug_renderer::debug_renderer_add_line( &Vector2::zero(), &Vector2::new( 512.0, 512.0 ), 10.0, &Color::white() );
				debug_renderer::debug_renderer_render( renderer );
				renderer.end_frame();
			},
			None => {},
		}
	}

}

