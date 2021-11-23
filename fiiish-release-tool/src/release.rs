
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
			println!("Checking if repository is clean...");

			let dirty = repo.get_dirty();

			if dirty.len() > 0 {
				println!("Dirty files:");
				for d in dirty.iter() {
					println!("{}", d);
				}
				bail!("Repository is dirty");
			}
			println!("Repositiory is clean (enough)");
		} else {
			println!("Skipping check if repository is clean!");
		}


		// load the Cargo.toml
		let mut manifest = Manifest::new( "Cargo.toml" );
		manifest.load()?;

		let old_version = manifest.get_pretty_version()?;
		println!("Current version: {}", &old_version);
		manifest.set_version_suffix("alpha");

		manifest.save()?;

		let release_version = manifest.get_pretty_version()?;
		println!("Release version: {}", &release_version);

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
		let msg = format!( ": Bump version for alpha release - {}", &release_version );
		repo.commit( &files, &msg )?;

		// post release
		manifest.bump_patch_version()?;
		manifest.set_version_suffix("dev");

		manifest.save()?;

		let new_version = manifest.get_pretty_version()?;
		println!("New development version: {}", &new_version);


		// :TODO: update Cargo.lock
		let msg = format!( ": Bump version back to dev release, and bump patch level - {}", &new_version );
		repo.commit( &files, &msg )?;

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
