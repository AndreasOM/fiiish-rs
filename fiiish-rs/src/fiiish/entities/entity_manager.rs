
use crate::fiiish::entities::Entity;

#[derive(Debug)]
pub struct EntityManager {
	entities: Vec< Box< dyn Entity > >,
}

impl EntityManager {
	pub fn new() -> Self {
		Self {
			entities: Vec::new(),
		}
	}

	pub fn setup( &self ) {

	}

	pub fn teardown( &self ) {

	}

	pub fn add( &mut self, entity: Box< Entity > ) {
		self.entities.push( entity );
	}

	pub fn iter_mut( &mut self ) -> std::slice::IterMut<'_, Box<(dyn Entity + 'static)>> {
		self.entities.iter_mut()
	}
}
