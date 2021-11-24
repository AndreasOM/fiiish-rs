
use toml_edit::{
	Document,
	Formatted,
	Item,
	Value,
	value,
};

use semver::{
	Version,
	Prerelease,
};

use anyhow::*;


pub struct Manifest {
	path: String,
	doc: Option< Document >,
}

impl Manifest {
	pub fn new( path: &str ) -> Self {
		Self {
			path: path.to_owned(),
			doc: None,
		}
	}

	pub fn load( &mut self ) -> anyhow::Result<()> {
		let toml = std::fs::read_to_string( &self.path )?;
		let mut doc = match toml.parse::<Document>(){
			Ok( doc ) => doc,
			Err( e ) => bail!( "Couldn't load manifest from >>{}<< {}", &self.path, &e ),
		};

		self.doc = Some( doc );
		Ok(())
	}

	pub fn save( &mut self ) -> anyhow::Result<()> {
		if let Some( doc ) = &self.doc {
			std::fs::write( &self.path, doc.to_string() ).unwrap();
		}

		Ok(())
	}

	fn get_formatted_version( &self ) -> anyhow::Result<( Formatted< String > )> {
		if let Some( doc ) = &self.doc {
			let fs = match &doc["package"]["version"] {
				Item::Value( Value::String( s ) ) => s, //.to_string(),
				_ => bail!("Unsupported version format!"),
			};
			Ok( fs.clone() )
		} else {
			bail!("No manifest loaded!");
		}
	}

	pub fn get_version( &self ) -> anyhow::Result<( Version )> {
		let fs = self.get_formatted_version()?;
//		dbg!(&fs);
		let v = fs.value();
		let mut version = Version::parse(&v).unwrap();
//			dbg!(&version);
		return Ok( version );
	}

	pub fn get_pretty_version( &self ) -> anyhow::Result<(String)> {
		let v = self.get_version()?;
		Ok( v.to_string() )
	}

	pub fn set_version( &mut self, version: &Version ) -> anyhow::Result<()>{
		let fs = self.get_formatted_version()?;

		// :TODO: talk to the edit_toml team what they were thinking
//		let mut nfs = fs.clone();
//		nfs.value = version.to_string();
//		dbg!(&nfs);
		if let Some( doc ) = &mut self.doc {
			let mut ni = value( version.to_string() );
			match &mut ni {
				Item::Value( Value::String( s ) ) => {
//					dbg!(&s);
					let mut d = s.decor_mut();
//					dbg!(&d);
					let od = fs.decor();
					match ( od.prefix(), od.suffix() ) {
						( Some( p ), Some( s ) ) => {
							d.set_prefix( p );
							d.set_suffix( s );
						},
						_ => {},
					}
				},
				_ => {},
			}
//			dbg!(&ni);
			doc[ "package" ][ "version" ] = ni;
		} else {
			bail!("No manifest loaded!");
		}
		Ok(())
	}

	pub fn bump_patch_version( &mut self ) -> anyhow::Result<()> {
		let old_version = self.get_version()?;
//		dbg!(&old_version);

		let mut new_version = old_version.clone();
		new_version.patch = old_version.patch + 1;
//		dbg!(&new_version);

		self.set_version( &new_version );
		Ok(())		
	}
	pub fn set_version_suffix(&mut self, suffix: &str ) -> anyhow::Result<()> {
		let old_version = self.get_version()?;
//		dbg!(&old_version);

		let mut new_version = old_version.clone();
		new_version.pre = Prerelease::new( suffix ).unwrap();
//		dbg!(&new_version);

		self.set_version( &new_version );
		Ok(())
	}
}
