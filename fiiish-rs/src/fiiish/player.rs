use crate::system::System;

#[derive(Debug)]
pub struct Player {
	coins:			u32,
	last_distance:	u32,
	total_distance: u32,
	best_distance:	u32,
	is_dirty:		bool,
}

impl Player {
	pub fn new() -> Self {
		Self {
			coins:			0,
			last_distance: 	0,
			total_distance: 0,
			best_distance:	0,
			is_dirty:		false,
		}
	}

	pub fn is_dirty( &mut self ) -> bool {
		self.is_dirty
	}
	pub fn load( &mut self, system: &mut System ) -> bool {
		self.is_dirty = false;
		true
	}

	pub fn save( &mut self, system: &mut System ) -> bool {
		println!("Saving player {:?}", self );
		self.is_dirty = false;
		true
	}

	pub fn coins( &self ) -> u32 {
		self.coins
	}

	pub fn last_distance( &self ) -> u32 {
		self.last_distance
	}

	pub fn total_distance( &self ) -> u32 {
		self.total_distance
	}

	pub fn best_distance( &self ) -> u32 {
		self.best_distance
	}

	pub fn give_coins( &mut self, count: u32 ) -> u32 {
		self.is_dirty = true;
		self.coins += count;
		self.coins
	}

	pub fn reset_last_distance( &mut self ) {
		self.is_dirty = true;
		self.last_distance = 0;
	}

	pub fn add_to_last_distance( &mut self, amount: u32 ) -> u32 {
		self.is_dirty = true;
		self.last_distance += amount;
		if self.best_distance < self.last_distance {
			self.best_distance = self.last_distance
		}
		self.total_distance += amount;
		self.last_distance
	}
}
