
use objc::*;
use objc::runtime::*;


fn print_nsstring( nsstring: *const Object ) {

	unsafe {
		let len: usize = msg_send![ nsstring, length ];
		let utf8string: *const u8 = msg_send![ nsstring, UTF8String ];
		let slice = std::ptr::slice_from_raw_parts( utf8string, len );
		let s = std::str::from_utf8( &*slice ).unwrap();

		println!("{}", s);
	}
}

fn play_music( filename: &str ) {

	let l = filename.len();
//	dbg!(l);
	unsafe {
		let cls_nsstring = class!(NSString);

		let resourcePath: *mut Object = msg_send![cls_nsstring, stringWithUTF8String: filename];

		print_nsstring( resourcePath );

		// NSData* data = [NSData dataWithContentsOfFile:resourcePath];
		let cls_nsdata = class!(NSData);

		let data: *mut Object = msg_send![cls_nsdata, dataWithContentsOfFile: resourcePath];

		let data_len: u64 = msg_send![ data, length ];
		dbg!(&data_len);

		let cls_nserror = class!(NSError);

//		NSError* error = [NSError alloc];
		let error: *mut Object = msg_send![ cls_nserror, alloc ];

		let cls_avaudioplayer = class!(AVAudioPlayer);
//		/*AVAudioPlayer* */player = [[AVAudioPlayer alloc]initWithData:data error:&error];
		let player: *mut Object = msg_send![ cls_avaudioplayer, alloc ];
		let player: *mut Object = msg_send![ player, initWithData: data error: &error ];

		let prep_result: bool = msg_send![ player, prepareToPlay ];
		let _: () = msg_send![ player, setNumberOfLoops: -1 ];
		let _: () = msg_send![ player, play ];
		let _: () = msg_send![ player, setVolume: 0.2 fadeDuration: 10.0 ];
	}
}

fn main() {
//	#[link(name = "Foundation", kind = "framework")]

    println!("Hello, world!");
    play_music("../../fiiish-content/music/theme-00.mp3");
    loop {

    };
}
