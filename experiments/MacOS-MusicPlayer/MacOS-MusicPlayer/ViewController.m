//
//  ViewController.m
//  MacOS-MusicPlayer
//
//  Created by anti on 11.11.21.
//

#import "ViewController.h"

#import <AVFAudio/AVFAudio.h>

@implementation ViewController

AVAudioPlayer* player = nil;

- (void)viewDidLoad {
	[super viewDidLoad];

	// Do any additional setup after loading the view.
	// find the file
	NSBundle *main = [NSBundle mainBundle];
	NSString *resourcePath = [main pathForResource:@"theme-00" ofType:@"mp3"];
	
//	NSString* resourcePath = [NSString stringWithUTF8String:"theme-00.mp3"];
	
	// load data into memory
	NSData* data = [NSData dataWithContentsOfFile:resourcePath];
	// play music from data
	NSError* error = [NSError alloc];
	/*AVAudioPlayer* */player = [[AVAudioPlayer alloc]initWithData:data error:&error];
	if( player == nil ){
		
	} else {
		if( ![player prepareToPlay] ){
			
		} else {
			[player setNumberOfLoops:-1];
			if( ![player play] ){
				
			} else {
				NSLog(@"Playing");
				if( [player isPlaying] ){
					NSLog(@"Really playing");
					[player setVolume:0.2 fadeDuration:10.0];
				}
			}
		}
	}
}


- (void)setRepresentedObject:(id)representedObject {
	[super setRepresentedObject:representedObject];

	// Update the view, if already loaded.
}


@end
