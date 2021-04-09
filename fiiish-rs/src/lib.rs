#[macro_use]
extern crate derivative;


// App generic
pub mod math;
pub mod renderer;
pub mod system;
pub mod window;
pub mod window_update_context;

mod debug_renderer;
	pub use debug_renderer::DebugRenderer as DebugRenderer;
// AppFiiish specific
pub mod fiiish;
