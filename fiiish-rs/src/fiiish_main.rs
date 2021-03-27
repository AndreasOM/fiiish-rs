
use fiiish_rs::fiiish::fiiish_app::FiiishApp;
use fiiish_rs::window::Window;
use fiiish_rs::window_update_context::WindowUpdateContext;

fn main() -> anyhow::Result<()>{
	println!("Fiiish!");

	// handle command line arguments

	// create App
	let mut app = FiiishApp::new();

	// create window for App
	let mut window = Window::new();

	window.setup()?;
	// window and context are active

	app.setup();

	window.run(move |wuc|{
//		dbg!(&wuc);
		app.update( wuc );
		app.render();
		app.is_done()
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

