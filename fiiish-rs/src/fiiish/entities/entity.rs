

use crate::fiiish::EntityUpdateContext;
use crate::fiiish::entities::EntityConfiguration;
use crate::fiiish::entities::EntityType;
use crate::renderer::Renderer;

pub trait Entity {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

	fn setup( &mut self, _ec: &EntityConfiguration ){}
	fn teardown( &mut self );
	fn update( &mut self, euc: &mut EntityUpdateContext );
	fn render( &mut self, renderer: &mut Renderer );

	fn name( &self ) -> &str;
	fn entity_type( &self ) -> EntityType;
	fn is_alive( &self ) -> bool {
		true
	}
	fn kill( &mut self ) {}
}

impl std::fmt::Debug for dyn Entity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(
			f, "[Trait] Entity: {} [{:?}] {}",
			self.name(),
			self.entity_type(),
			if self.is_alive() {
				"[ALIVE]"
			} else {
				"[DEAD]"
			}
		)
	}
}
