
use crate::system::System;

use std::collections::VecDeque;

use objc::*;
use objc::runtime::*;

#[derive(Debug)]
pub struct SoundApple {
	players: 	 	 	VecDeque< *mut Object >,
	playing_players: 	VecDeque< *mut Object >,
}

impl SoundApple {

	pub fn new() -> Self {
		Self {
			players: 	 		VecDeque::new(),
			playing_players: 	VecDeque::new(),
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
				self.players.push_back( player );
//				let _: () = msg_send![ player, setNumberOfLoops: -1 ];
				true
			} else {
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


			for n in 0..number {
				if !self.load_from_data( data ) {
					println!("Couldn't read Sound {} from {}!", &name, &filename);
					return false
				}
			}
			dbg!( self.players.len() );
			true
		} else {
			println!("Sound {} not found", &name );
			false
		}
	}

	pub fn play( &mut self, name: &str ) {
		
		if let Some( player ) = match self.players.pop_front() {
			Some( player ) => Some( player ),
			None => {
				if let Some( &player ) = self.playing_players.front() {
					unsafe {
						let playing: bool = msg_send![ player, isPlaying ];
						if !playing {
							self.playing_players.pop_front()
						} else {
							None
						}
					}
				} else {
					None
				}
			},
		} {
			unsafe {
				let _: () = msg_send![ player, play ];
			}
			self.playing_players.push_back( player );
		};


/*

		if let Some( player ) = self.players.pop_front() {
			unsafe {
				let _: () = msg_send![ player, play ];
			}
			self.playing_players.push_back( player );
		} else {
			if let Some( &player ) = self.playing_players.front() {
				unsafe {
					let playing: bool = msg_send![ player, isPlaying ];
					if !playing {
						if let Some( player ) = self.playing_players.pop_front() {
							unsafe {
								let _: () = msg_send![ player, play ];
							}
							self.playing_players.push_back( player );							
						}
					}
				}
			}
		}
*/
	}

	pub fn update( &mut self, _time_step: f64 ) {

	}



}

