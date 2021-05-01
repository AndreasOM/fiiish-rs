

#[derive(Debug)]
pub struct Player {
	coins: u32,
}

impl Player {
	pub fn new() -> Self {
		Self {
			coins: 1234,		// :HACK: until we save the player state between sessions
		}
	}

	pub fn coins( &self ) -> u32 {
		self.coins
	}

	pub fn give_coins( &mut self, count: u32 ) -> u32 {
		self.coins += count;
		self.coins
	}
}