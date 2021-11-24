
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

		if false { // use repository in sub folder for testing
			let root = std::path::Path::new(&std::env::current_dir()?).join("./automatic-octo-guacamole/");
			std::env::set_current_dir(&root)?;
		}

		println!("Working in {:?}", std::env::current_dir()?);
		// check if git is clean
		let mut repo = Repository::new( "." );
//		let mut repo = Repository::new( "./automatic-octo-guacamole/" );

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


		if true { // skip for faster iteration
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


		} else {
			println!( "Skipping everything up to fetch/rebase/push!" );
		}

		let dirty = repo.get_dirty();

		if dirty.len() > 0 {
			println!("Dirty files before fetch:");
			for d in dirty.iter() {
				println!("{}", d);
			}
		}

		if repo.fetch()? > 0 {
			bail!("Fetch was not empty. Please resolve manually!")
		};
		repo.rebase()?;
		repo.push()?;

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
