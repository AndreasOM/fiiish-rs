
use crate::system::System;

use objc::*;
use objc::runtime::*;

#[derive(Debug)]
pub struct MusicApple {
	player: Option< *mut Object >,
}

impl MusicApple {

	pub fn new() -> Self {
		Self {
			player: None,
		}
	}


	fn load_from_slice( &mut self, slice: &[u8] ) -> bool {

		unsafe {
			let cls_nsdata = class!(NSData);

//			let data: *mut Object = msg_send![cls_nsdata, dataWithContentsOfFile: resourcePath];
			let data: *mut Object = msg_send![ cls_nsdata, dataWithBytes:slice.as_ptr() length:slice.len() ];
			let data_len: u64 = msg_send![ data, length ];
			dbg!(&data_len);

			let cls_nserror = class!(NSError);
			let error: *mut Object = msg_send![ cls_nserror, alloc ];

			let cls_avaudioplayer = class!(AVAudioPlayer);
			let player: *mut Object = msg_send![ cls_avaudioplayer, alloc ];
			let player: *mut Object = msg_send![ player, initWithData: data error: &error ];

			let prep_result: bool = msg_send![ player, prepareToPlay ];
			if prep_result {
				self.player = Some( player );
				let _: () = msg_send![ player, setNumberOfLoops: -1 ];
				true
			} else {
				self.player = None;
				false
			}
//			let _: () = msg_send![ player, setVolume: 0.2 fadeDuration: 10.0 ];
		}
	}
	pub fn load( &mut self, system: &mut System, filename: &str ) -> bool {

		let mut f = system.default_filesystem_mut().open( &filename );
		if f.is_valid() {
			println!("Loading Music from {} ({}).", &filename, &f.name());
			let mut buf = Vec::new();
			while !f.eof() {
				let c = f.read_u8();
				buf.push( c );
			}

//			let buf: &[u8] = buf.as_slice();

			self.load_from_slice( buf.as_slice() )
		} else {
			println!("Couldn't read Music from {}!", &filename);

			false
		}
	}

	pub fn play( &mut self ) {
		if let Some( player ) = self.player {
			unsafe {
				let _: () = msg_send![ player, play ];
			}
		}
	}

	pub fn pause( &mut self ) {
		if let Some( player ) = self.player {
			unsafe {
				let _: () = msg_send![ player, pause ];
			}
		}		
	}

	pub fn stop( &mut self ) {
		if let Some( player ) = self.player {
			unsafe {
				let _: () = msg_send![ player, stop ];
			}
		}		
	}

	pub fn update( &mut self, _time_step: f64 ) {

	}



}

