use chrono::prelude::*;

use fiiish_rs::fiiish::fiiish_app::FiiishApp;
use fiiish_rs::window::Window;
//use fiiish_rs::window_update_context::WindowUpdateContext;

fn main() -> anyhow::Result<()>{
	println!("Fiiish!");

	// handle command line arguments

	// create App
	let mut app = FiiishApp::new();

	// create window for App
	let mut window = Window::new();

	window.setup()?;
	// window and context are active

	let start_time: DateTime<Utc> = Utc::now();
	app.setup( &mut window )?;
	let end_time: DateTime<Utc> = Utc::now();
	let load_duration = end_time.signed_duration_since( start_time );
	let load_time = load_duration.num_milliseconds() as f64 / 1000.0;
	println!("App setup took {} seconds", load_time);

//	todo!("die");

	window.run(move |wuc|{
//		dbg!(&wuc);
		app.update( wuc );
		app.render();
		if app.is_done() {
			println!("App is done, tearing down");
			app.teardown();
			true
		} else {
			false
		}
	});

//	window.run( ... )
	/*
	while !app.is_done() {
		app.update();
		app.render();
	}
	*/

//	app.teardown();

	window.teardown();

	Ok(())
}

