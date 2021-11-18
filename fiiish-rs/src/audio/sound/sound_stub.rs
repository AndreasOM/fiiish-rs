
use crate::system::System;


#[derive(Debug)]
pub struct SoundStub {
}

impl SoundStub {

	pub fn new() -> Self {
		Self {
		}
	}

	pub fn load( &mut self, system: &mut System, _name: &str, _number: u16 ) -> bool {
		true
	}

	pub fn play( &mut self, _name: &str ) {
	}

	pub fn update( &mut self, _time_step: f64 ) {
	}
}

