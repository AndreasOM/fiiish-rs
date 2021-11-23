
use anyhow::*;

use crate::manifest::Manifest;
use crate::repository::Repository;

pub struct Release {

}

impl Release {
	pub fn new() -> Self {
		Self {

		}
	}

	pub fn run( &self ) -> anyhow::Result<()> {

		// check if git is clean
		let mut repo = Repository::new( "." );
		repo.open()?;
		if false { // skip dirty

			let dirty = repo.get_dirty();

			if dirty.len() > 0 {
				println!("Dirty files:");
				for d in dirty.iter() {
					println!("{}", d);
				}
				bail!("Repository is dirty");
			}
		}

		println!("Repositiory is clean (enough)");

		// load the Cargo.toml
		let mut manifest = Manifest::new( "Cargo.toml" );
		manifest.load()?;

		manifest.set_version_suffix("alpha");

		manifest.save()?;

		// dbg!(&doc);

		let dirty = repo.get_dirty();

		if dirty.len() > 0 {
			println!("Dirty files:");
			for d in dirty.iter() {
				println!("{}", d);
			}
		}

		// :TODO: update Cargo.lock

		let mut files = Vec::new();
		files.push( "Cargo.toml".to_owned() );
		repo.commit( &files, ": Bump version for alpha release" )?;

		// post release
		manifest.bump_patch_version()?;
		manifest.set_version_suffix("dev");

		manifest.save()?;


		// :TODO: update Cargo.lock

		repo.commit( &files, ": Bump version back to dev release, and bump patch level." )?;

		let dirty = repo.get_dirty();

		if dirty.len() > 0 {
			println!("Dirty files:");
			for d in dirty.iter() {
				println!("{}", d);
			}
		}

		Ok(())
	}
}
