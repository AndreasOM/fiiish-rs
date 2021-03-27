
use glutin::event::{ElementState, Event, KeyboardInput, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{
	WindowBuilder,
};
use glutin::ContextBuilder;
use glutin::{
	ContextWrapper,
	PossiblyCurrent,
};
use glutin::event::VirtualKeyCode;

use crate::window_update_context::WindowUpdateContext;

pub struct Window {
	el: Option< EventLoop<()> >,
	windowed_context: Option< ContextWrapper<PossiblyCurrent, glutin::window::Window> >,
}

impl Window {
	pub fn new() -> Self {
		Self {
			el: None,
			windowed_context: None,
		}
	}

	// some form of configuration

	pub fn setup(&mut self) -> anyhow::Result<()> {

	    let el = EventLoop::new();
	    let wb = WindowBuilder::new().with_title("A fantastic window!");

	    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

	    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

	    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

	    self.el = Some( el );
	    self.windowed_context = Some( windowed_context );

		Ok(())
	}

	pub fn teardown(&mut self) {

	}

	pub fn run<F: 'static>( &mut self, mut f: F )
		where F: FnMut( &mut WindowUpdateContext ) -> bool
	{
		let el = self.el.take().unwrap();
		let windowed_context = self.windowed_context.take().unwrap();
	    let mut is_done = false;
	    let mut window_update_context = WindowUpdateContext::new();

	    el.run(move |event, _, control_flow| {
//	        println!("{:?}", event);
	        *control_flow = ControlFlow::Poll;

	        match event {
	            Event::LoopDestroyed => return,
	            Event::WindowEvent { event, .. } => match event {
	                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
	                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
					WindowEvent::KeyboardInput {
                    	input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                    	..
                	} => match (virtual_code, state) {
                		( VirtualKeyCode::Escape, state ) => {
                			window_update_context.is_escaped_pressed = state == ElementState::Pressed;
//                			println!("Escape {:?}", &state );
                		},
                		( VirtualKeyCode::Space, state ) => {
                			window_update_context.is_space_pressed = state == ElementState::Pressed;
//                			println!("Space {:?}", &state );
                		},
                		_ => {},
                	},
					_ => (),
	            },
	            Event::RedrawRequested(_) => {
//	                gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
	                windowed_context.swap_buffers().unwrap();
	            },
	            Event::MainEventsCleared | Event::RedrawEventsCleared => {},
	            Event::NewEvents( _ ) => {},
	            Event::DeviceEvent{ .. } => {

	            },
	            e => {
	            	println!("Unhandled event: {:?}", e);
	            },
	        }

	        if !is_done && f( &mut window_update_context ) {
	        	println!("f returned false");
	        	*control_flow = ControlFlow::Exit;
	        	is_done = true;
	        }
    	});
	}

}
