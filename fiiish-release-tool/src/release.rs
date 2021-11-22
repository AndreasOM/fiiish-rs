use toml_edit::{
	Document,
//	Formatted,
	Item,
	Value,
	value,
};

use semver::{
	Prerelease,
	Version,
};

//use unescape::unescape;

use regex::Regex;

use anyhow::*;

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
		if true {
			repo.open()?;

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

		let toml = std::fs::read_to_string("Cargo.toml").unwrap();
		let mut doc = toml.parse::<Document>().expect("invalid doc");

		let old_version_value = &doc["package"]["version"];
		dbg!(&old_version_value);
		let vs = match &doc["package"]["version"] {
			Item::Value( Value::String( s ) ) => s.to_string(),
			_ => bail!("Unsupported version format!"),
		};
		dbg!(&vs);

		let re = Regex::new(r#"(\s*)"(.*)"(.*)"#).unwrap();

//		dbg!(&re);
		let caps = re.captures( &vs ).unwrap();
//		dbg!(&caps);
		if caps.len() < 3 {
			bail!("Invalid version format >>{}<<!", &vs);
		}
		let v = &caps[ 2 ];
		let mut version = Version::parse(&v).unwrap();
		dbg!(&version);
		let old_version = version.clone();
		version.pre = Prerelease::new( "alpha" ).unwrap();
		dbg!(&version.to_string());
//		dbg!(&doc);

//		let full_version = unescape( &format!("{}\"{}\"{}", &caps[1], &version, &caps[3]) ).unwrap();
//		let full_version = format!("{}\"{}\"{}", &caps[1], &version, &caps[3]);
		let full_version = version.to_string();
		dbg!(&full_version);
		println!("{}", &full_version);

//		let mut new_version_value = Formatted{
//			value: full_version,
//		};
//		let new_version_value = old_version_value.clone();
//		new_version_value.value = "TADA";
		//dbg!(&new_version_value);
//		let i = Item::Value( Value::String( Formatted::new( full_version ) ) );
		let i = value( full_version );
		dbg!(&i);
		doc["package"]["version"] = i;

		// dbg!(&doc);
		std::fs::write("Cargo.toml", doc.to_string() ).unwrap();

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
