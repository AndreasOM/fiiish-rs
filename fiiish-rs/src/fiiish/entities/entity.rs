

use crate::fiiish::EntityUpdateContext;
use crate::renderer::Renderer;

pub trait Entity {
	fn setup( &mut self, name: &str );
	fn teardown( &mut self );
	fn update( &mut self, euc: &mut EntityUpdateContext );
	fn render( &mut self, renderer: &mut Renderer );

	fn name( &self ) -> &str;
}

impl std::fmt::Debug for Entity {
	fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		todo!()
	}
}
