
use oml_game::Game;
use fiiish_rs::fiiish::fiiish_app::FiiishApp;

use tracing::*;
use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()>{
	println!("Fiiish!");
	let use_ansi = atty::is(atty::Stream::Stdout);

	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::TRACE)
		.with_ansi(use_ansi) // sublime console doesn't like it :(
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

	let app = FiiishApp::new();

	match Game::run(app) {
		Ok(_) => {},
		Err(e) => {
			error!("Game returned {}", &e)
		},
	}

	Ok(())
/*
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
*/
}

