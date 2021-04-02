
use chrono::prelude::*;

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
	title: String,
}

impl Window {
	pub fn new() -> Self {
		Self {
			el: None,
			windowed_context: None,
			title: String::new(),
		}
	}

	// some form of configuration
	pub fn set_title( &mut self, title: &str ) {
		self.title = title.to_string();
		// if the window is already open fix the title
		if let Some( ctx ) = &mut self.windowed_context {
			ctx.window().set_title( &self.title );
		}
	}

	pub fn setup(&mut self) -> anyhow::Result<()> {

	    let el = EventLoop::new();
	    let wb = WindowBuilder::new()
//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 1920/2, height: 1080/2 } )
//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 1920/2, height: 512 } )
	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 1920/2, height: 700 } )
//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 512, height: 512 } )
	    			.with_title(&self.title);

	    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

	    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

	    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

	    self.el = Some( el );
	    self.windowed_context = Some( windowed_context );

		Ok(())
	}

	pub fn teardown(&mut self) {

	}

	pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
		match &self.windowed_context {
			Some( windowed_context ) => {
		        windowed_context.get_proc_address(addr)
			}
			None => {
				std::ptr::null()
			},
		}
    }	

	pub fn run<F: 'static>( &mut self, mut f: F )
		where F: FnMut( &mut WindowUpdateContext ) -> bool
	{
		let el = self.el.take().unwrap();
		let windowed_context = self.windowed_context.take().unwrap();
	    let mut is_done = false;
	    let mut window_update_context = WindowUpdateContext::new();

	    let mut previous_now: DateTime<Utc> = Utc::now();

	    el.run(move |event, _, control_flow| {
		    window_update_context.window_size.x = windowed_context.window().inner_size().width as f32;
		    window_update_context.window_size.y = windowed_context.window().inner_size().height as f32;

//	        println!("{:?}", event);
//	        *control_flow = ControlFlow::Poll;
			let next_frame_time = std::time::Instant::now() +
            	std::time::Duration::from_nanos(16_666_667);
        	*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

	        match event {
	            Event::LoopDestroyed => return,
	            Event::WindowEvent { event, .. } => match event {
	                WindowEvent::Resized(physical_size) => {
	                	dbg!(&physical_size);
	                	windowed_context.resize(physical_size)
	                },
	                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
	                WindowEvent::CursorMoved { position, .. } => {
	                	let inner_size = windowed_context.window().inner_size();

	                	let w = inner_size.width as f64;
	                	let h = inner_size.height as f64;
	                	let mouse_x = position.x/w;
	                	let mouse_y = ( h - position.y )/h;
						window_update_context.mouse_pos.x = mouse_x as f32;
						window_update_context.mouse_pos.y = mouse_y as f32;
//	                	dbg!(&position, &inner_size, &mouse_x, &mouse_y);
	                },
	                WindowEvent::MouseInput { state, button, .. } => {
	                	let button_index = match button {
	                		glutin::event::MouseButton::Left => 0,
	                		glutin::event::MouseButton::Middle => 1,
	                		glutin::event::MouseButton::Right => 2,
	                		_ => 0,
	                	};
	                	window_update_context.mouse_buttons[ button_index ] = state == glutin::event::ElementState::Pressed;

//	                	dbg!(&state, &button, &window_update_context.mouse_buttons);
	                },
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
                		( vkc, state ) if vkc >= VirtualKeyCode::A && vkc <= VirtualKeyCode::Z => {
                			let o = ( ( vkc as u16 ) - ( VirtualKeyCode::A as u16 ) ) as u8;
                			let o = ( o + 'a' as u8 ) as usize;
                			//println!("KeyboardInput A-Z {:?} -> {}", &vkc, &o);
                			window_update_context.is_key_pressed[ o ] = state == ElementState::Pressed;
                		},
                		_ => {
                			println!("KeyboardInput {:?}", &virtual_code);
                		},
                	},
					_ => (),
	            },
	            Event::RedrawRequested(_) => {
//	                gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
	                windowed_context.swap_buffers().unwrap();
	            },
	            Event::MainEventsCleared => {
	            	// all evens handled, lets render
			        let now: DateTime<Utc> = Utc::now();
			        let frame_duration = now.signed_duration_since( previous_now );
			        let time_step = frame_duration.num_milliseconds() as f64 / 1000.0;
			        previous_now = now;
			        window_update_context.time_step = time_step;

			        if !is_done && f( &mut window_update_context ) {
			        	println!("f returned false");
			        	*control_flow = ControlFlow::Exit;
			        	is_done = true;
			        }
			        
			        window_update_context.update();
	                windowed_context.swap_buffers().unwrap();	            	
	            },
	            Event::RedrawEventsCleared => {},
	            Event::NewEvents( _ ) => {},
	            Event::DeviceEvent{ .. } => { // :TODO: handle Button

	            },
	            e => {
	            	println!("Unhandled event: {:?}", e);
	            },
	        }



    	});
	}

}
