
use crate::system::System;

use objc::*;
use objc::runtime::*;

#[derive(Debug)]
pub struct SoundApple {
	player: Option< *mut Object >,
}

impl SoundApple {

	pub fn new() -> Self {
		Self {
			player: None,
		}
	}

	fn load_data( system: &mut System, filename: &str ) -> *mut Object {

		unsafe {
			let cls_nsdata = class!(NSData);

			let mut f = system.default_filesystem_mut().open( &filename );
			let data: *mut Object = if f.is_valid() {
				println!("Loading Data from {}.", &filename);
				let mut buf = Vec::new();
				while !f.eof() {
					let c = f.read_u8();
					buf.push( c );
				}
				let slice = buf.as_slice();

				msg_send![ cls_nsdata, dataWithBytes:slice.as_ptr() length:slice.len() ]
			} else {
				msg_send![ cls_nsdata, data ]
			};

			let data_len: u64 = msg_send![ data, length ];
			dbg!(&data_len);

			data
		}
	}

	fn load_from_data(&mut self, data: *const Object ) -> bool {

		unsafe {
			let cls_nserror = class!(NSError);
			let error: *mut Object = msg_send![ cls_nserror, alloc ];

			let cls_avaudioplayer = class!(AVAudioPlayer);
			let player: *mut Object = msg_send![ cls_avaudioplayer, alloc ];
			let player: *mut Object = msg_send![ player, initWithData: data error: &error ];

			let prep_result: bool = msg_send![ player, prepareToPlay ];
			if prep_result {
				self.player = Some( player );
//				let _: () = msg_send![ player, setNumberOfLoops: -1 ];
				true
			} else {
				self.player = None;
				false
			}
//			let _: () = msg_send![ player, setVolume: 0.2 fadeDuration: 10.0 ];
		}

	}

	pub fn load( &mut self, system: &mut System, name: &str, number: u16 ) -> bool {

		let extensions = [ ".caf", ".wav" ];

		let filename_maybe = extensions.iter().find_map(
			|e|{
				let filename = format!("{}{}", name, e);
				println!("Checking if {} exists:", filename );
				if system.default_filesystem_mut().exists( &filename ) {
					Some( filename.to_owned() )
				} else {
					None
				}
			}
		);

		if let Some( filename ) = filename_maybe {
			let data = SoundApple::load_data( system, &filename );
			if self.load_from_data( data ) {
				true
			} else {
				println!("Couldn't read Sound {} from {}!", &name, &filename);

				false
			}
		} else {
			println!("Sound {} not found", &name );
			false
		}
	}

	pub fn play( &mut self, name: &str ) {
		
		if let Some( player ) = self.player {
			unsafe {
				let _: () = msg_send![ player, play ];
			}
		}

	}

	pub fn pause( &mut self ) {
	}

	pub fn stop( &mut self ) {
	}

	pub fn update( &mut self, _time_step: f64 ) {

	}



}

