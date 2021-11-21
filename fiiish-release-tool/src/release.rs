use git2::{
	Repository,
	Status,
};

use anyhow::*;

pub struct Release {

}

impl Release {
	pub fn new() -> Self {
		Self {

		}
	}

	pub fn run( &self ) -> anyhow::Result<()> {

		// check if git is clean
		let repo = match Repository::discover(".") {
		    Ok(repo) => repo,
		    Err(e) => bail!("failed to open: {}", e),
		};

		dbg!(&repo.state());
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

		if dirty.len() > 0 {
			for d in dirty.iter() {
				println!("{}", d);
			}
			bail!("Repository is dirty");
		}

		println!("Repositiory is clean (enough)");


		Ok(())
	}
}
