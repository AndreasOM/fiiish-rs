

use crate::renderer::{
	Renderer,
	Texture,
};
use crate::system::System;

#[derive(Debug)]
pub struct AnimatedTexture {
	prefix: String,
	number_of_digits: u8,
	number_of_frames: u16,
	fps: f32,
	current_frame: u16,
	time_per_frame: f32,
	time_in_current_frame: f32,
}

impl AnimatedTexture {

	pub fn new() -> Self {
		Self {
			prefix: String::new(),
			number_of_digits: 0,
			number_of_frames: 0,
			fps: 0.0,
			current_frame: 0,
			time_per_frame: f32::MAX,
			time_in_current_frame: 0.0,
		}
	}

	pub fn setup( &mut self, prefix: &str, number_of_digits: u8, number_of_frames: u16, fps: f32 ) {
		self.prefix = prefix.to_owned();
		self.number_of_digits = number_of_digits;
		self.number_of_frames = number_of_frames;
		self.fps = fps;
		self.time_per_frame = 1.0/fps;
	}

	pub fn update( &mut self, time_step: f64 ) {
		self.time_in_current_frame += time_step as f32;
		while self.time_in_current_frame > self.time_per_frame {
			self.current_frame = ( self.current_frame+1 ) % self.number_of_frames;
			self.time_in_current_frame -= self.time_per_frame;
		}
	}

	pub fn r#use( &self, renderer: &mut Renderer ) {
		let name = AnimatedTexture::append_number_with_digits( &self.prefix, self.current_frame, self.number_of_digits );
		renderer.use_texture( &name )
	}

	fn append_number_with_digits( prefix: &str, number: u16, number_of_digits: u8 ) -> String {
		match number_of_digits {
			2 => {
				format!("{}{:02}", &prefix, number)
			},
			4 => {
				format!("{}{:04}", &prefix, number)
			},
			_ => todo!("Add support for {} digits", number_of_digits),
		}
	}

	// :HACK: Scanning the filesystem is a bad idea, the info should come from the config
	pub fn register_all( system: &mut System, renderer: &mut Renderer, prefix: &str, number_of_digits: u8 ) -> usize {

		let mut fs = system.default_filesystem_mut();

		let mut to_load = Vec::new();
		let mut i = 0;

		loop {
//			let name = format!( &template, i );	// :(
			let name = AnimatedTexture::append_number_with_digits( prefix, i, number_of_digits );

			// :HACK: to workaround missing "exists with .*"
			let name_ext = format!("{}.png", &name);

			if fs.exists( &name_ext ) {
				to_load.push( name.to_owned() );
			} else {
				println!("{} does not exist", &name_ext);
				break;
			}
			i += 1;
		};

		dbg!(&to_load);

		for name in to_load.iter() {
			renderer.register_texture( Texture::create( system, &name ) );
		}

//		todo!("die");
		to_load.len()
	}	
}