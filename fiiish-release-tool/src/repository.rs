use git2::{
//	Repository,
	Status,
};

use anyhow::*;



pub struct Repository {
	path: String,
	repo: Option< git2::Repository >,
}

impl Repository {
	pub fn new( path: &str ) -> Self {
		Self {
			path: path.to_owned(),
			repo: None,
		}
	}

	pub fn open( &mut self ) -> anyhow::Result<()> {
		let repo = match git2::Repository::discover( &self.path ) {
		    Ok(repo) => repo,
		    Err(e) => bail!("failed to open: {}", e),
		};

		dbg!(&repo.state());

		self.repo = Some( repo );

		Ok(())
	}

	pub fn get_dirty( &mut self ) -> Vec<String> {
		match &self.repo {
			Some( repo ) => {
				let mut dirty = Vec::new();
				let mut check_s = Status::empty();
				check_s.insert(Status::INDEX_NEW);
				check_s.insert(Status::WT_MODIFIED);

				let mut skip_s = Status::empty();
				skip_s.insert(Status::IGNORED);
				skip_s.insert(Status::WT_NEW);

				for se in repo.statuses( None ).unwrap().iter() {
					let s = se.status();
					if s.intersects( check_s ) {
						dirty.push( se.path().unwrap_or( "" ).to_owned());
					} else {
						if !s.intersects( skip_s ) {
							println!("Not dirty {:?} {}", s, se.path().unwrap_or( "" ) );
						}
					}
				}
				dirty
			},
			None => {
				Vec::new()
			},
		}
	}
}