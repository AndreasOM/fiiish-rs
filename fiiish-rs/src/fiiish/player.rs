use crate::system::{ System };
use oml_game::system::Serializer;
const SERIALIZED_VERSION: u16 = 0x0003;
const OLDEST_SERIALIZED_VERSION: u16 = 0x0001;

#[derive(Debug)]
pub struct Player {
	coins:			u32,
	last_distance:	u32,
	total_distance: u32,
	best_distance:	u32,
	play_count:		u32,
	is_music_enabled:	bool,
	is_sound_enabled:	bool,
	is_dirty:		bool,
}

impl Player {
	pub fn new() -> Self {
		Self {
			coins:			0,
			last_distance: 	0,
			total_distance: 0,
			best_distance:	0,
			play_count:		0,
			is_music_enabled: 	true,
			is_sound_enabled:	true,
			is_dirty:		false,
		}
	}

	pub fn is_dirty( &mut self ) -> bool {
		self.is_dirty
	}
	pub fn load( &mut self, system: &mut System ) -> bool {
		let filename = format!("{}{}", "default", ".fiiishsave" );
		let f = system.savegame_filesystem_mut().open( &filename );
		if !f.is_valid() {
			println!("Not loading player. File not found {}", &filename );
			return false;
		}
		println!("Loading player from {}", &filename );

		let mut s = Serializer::new( f );

		if !self.serialize( &mut s ) {
			return false;
		}

		self.is_dirty = false;
		true
	}

	pub fn save( &mut self, system: &mut System ) -> bool {
		println!("Saving player {:?}", self );
		let filename = format!("{}{}", "default", ".fiiishsave" );
		let f = system.savegame_filesystem_mut().create( &filename, true );
		dbg!(&f);
		if !f.is_valid() {
			println!("Not saving player. Couldn't create {}", &filename );
			return false;
		}
		println!("Saving player to {}", &filename );
		let mut s = Serializer::new( f );

		if !self.serialize( &mut s ) {
			return false;
		}

//		f.write_u8( 0xaa );
//		dbg!(&f);

		dbg!(&s);

		self.is_dirty = false;
		true
	}

	fn serialize( &mut self, s: &mut Serializer ) -> bool {
		const MAGIC: [u8;8] = [ 0x4f, 0x4d, 0x46, 0x49, 0x49, 0x49, 0x53, 0x48 ];
		let mut buff = MAGIC;
		for ( i, m ) in buff.iter_mut().enumerate() {
			s.serialize_u8( m );

			if *m != MAGIC[ i ] {
				println!( "Magic mismatch in savegame" );
				return false;
			}
		}

		let mut version = SERIALIZED_VERSION;
		s.serialize_u16( &mut version );
		if version != SERIALIZED_VERSION {
			if version < OLDEST_SERIALIZED_VERSION {
				println!( "Version mismatch in savegame" );
				return false;				
			}
			println!("Loading old version ({}) savegame, current is {}", version, SERIALIZED_VERSION );
//			panic!("");
		}


		s.serialize_u32( &mut self.coins );
		s.serialize_u32( &mut self.last_distance );	// :TODO: do we even need to store this?
		s.serialize_u32( &mut self.total_distance );
		s.serialize_u32( &mut self.best_distance );
//		if version >= SERIALIZED_VERSION {	// reader defaults to "0"
			s.serialize_u32( &mut self.play_count );
//		} else {
//			self.play_count = 0;
//		}

		if version >= 0x0003 {	// oldest version with music & sound settings
			s.serialize_bool( &mut self.is_music_enabled );
			s.serialize_bool( &mut self.is_sound_enabled );
		} 
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

	pub fn log_play( &mut self, _coins: u32, _distance: u32 ) {
		self.play_count += 1;
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

	pub fn set_music_enabled(&mut self, enabled: bool ) {
		self.is_dirty = true;
		self.is_music_enabled = enabled;
	}

	pub fn set_sound_enabled(&mut self, enabled: bool ) {
		self.is_dirty = true;
		self.is_sound_enabled = enabled;
	}

	pub fn music_enabled( &self ) -> bool {
		self.is_music_enabled
	}

	pub fn sound_enabled( &self ) -> bool {
		self.is_sound_enabled
	}
}
