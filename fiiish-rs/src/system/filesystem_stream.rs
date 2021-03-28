

pub trait FilesystemStream {
	fn size( &self, ) -> usize;
	fn pos( &self,  ) -> usize;
	fn set_pos( &mut self, pos: usize );
	fn read_u8( &mut self ) -> u8;
	fn is_valid( &self ) -> bool;
	fn eof( &self ) -> bool;
	fn name( &self ) -> &str;
	fn filesystem_stream_type( &self ) -> &str;
}


impl std::fmt::Debug for dyn FilesystemStream {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
		writeln!(
			f, "[Trait] FilesystemStream: {} [{}] {}",
			self.name(),
			self.filesystem_stream_type(),
			if self.is_valid() {
				"[VALID]"
			} else {
				"[INVALID]"
			}
		)
	}
}
