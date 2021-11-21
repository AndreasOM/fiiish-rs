use git2::{
	Repository,
	Status,
};

use toml_edit::{
	Document,
	Formatted,
	Item,
	Value,
	value,
};

use semver::{BuildMetadata, Prerelease, Version, VersionReq};

use unescape::unescape;

use regex::Regex;

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
		if true {
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


		Ok(())
	}
}
