
use std::collections::HashMap;
use crate::fiiish::Shape;
use crate::fiiish::entities::EntityId;
use crate::system::System;

#[derive(Debug)]
pub struct ShapeCache {
	shapes: HashMap< EntityId, Shape >,
}

impl ShapeCache {
	pub fn new() -> Self {
		Self {
			shapes: HashMap::new(),
		}
	}


	pub fn load_shape( &mut self, system: &mut System, name: &str, entity_id: EntityId ) {
		let mut shape =Shape::new();
		shape.load( system, name );
		self.shapes.insert( entity_id, shape );
	}

	pub fn find( &self, entity_id: EntityId ) -> Option< &Shape > {
		self.shapes.get( &entity_id )
	}
}
