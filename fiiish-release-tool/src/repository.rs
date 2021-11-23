use git2::{
//	Repository,
	Signature,
	Status,
};

use anyhow::*;

use std::path::Path;


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
				check_s.insert(Status::INDEX_MODIFIED);
				check_s.insert(Status::WT_MODIFIED);

				let mut skip_s = Status::empty();
				skip_s.insert(Status::IGNORED);
				skip_s.insert(Status::WT_NEW);

				for se in repo.statuses( None ).unwrap().iter() {
					let s = se.status();
//					println!("Maybe dirty {:?} {}", s, se.path().unwrap_or( "" ) );
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

	pub fn commit( &mut self, files: &Vec<String>, message: &str ) -> anyhow::Result<()> {
		match &self.repo {
			Some( repo ) => {
				/*
				let name = "Somebody";
				let email = "some@body.org";
				let author = Signature::now( &name, &email )?;
				let commiter = Signature::now( &name, &email )?;
				*/

				let sig = repo.signature()?;

				let mut index = match repo.index() {
					Err(e) => bail!("No index for repository {}", &e),
					Ok(index) => index,
				};
				let cwd = match std::env::current_dir() {
					Ok( wd ) => wd,
					Err( e ) => bail!("No current working directory found"),
				};
//				dbg!(&repo.path());
//				dbg!(&repo.workdir());
				let rwd = match repo.workdir() {
					Some( wd ) => wd,
					None => bail!("No workdir for repository"),
				};
//				dbg!(&cwd);
//				dbg!(&rwd);				
				for f in files.iter() {
					let p = Path::new( &cwd ).join( &f );
//					dbg!(&p);
					// filenames are relative to working directory
					// make them relative to repository root
					let p = match p.strip_prefix( &rwd ) {
						Ok( p ) => p,
						Err( e ) => bail!("Error stripping {:?} from {:?}", &rwd, &p),
					};
//					dbg!(&p);
					index.add_path( &p )?;
				};

				index.write()?;

				let mut index = match repo.index() {
					Err(e) => bail!("No index for repository {}", &e),
					Ok(index) => index,
				};
/*
pub fn write_tree(&mut self) -> Result<Oid, Error>
*/
				let oid = match index.write_tree() {
					Ok( oid ) => oid,
					Err( e ) => bail!("Error writing tree for repository: {}", &e ),
				};
				index.write()?;

//				dbg!(&oid);

				let tree = match repo.find_tree(oid) {
					Ok( tree ) => tree,
					Err( e ) => bail!("Error findind tree for OID {}: {}", &oid, &e),
				};

//				dbg!(&tree);

				let parent = match repo.revparse_ext( "HEAD" ) {
					Ok( ( object, _ ) ) => object,
					Err( e ) => bail!( "Error finding HEAD {}", &e ),
				};

				let parent = match parent.as_commit() {
					Some( commit ) => commit,
					None => bail!( "Parent is not a commit" ),
				};

//				dbg!(&parent);
				/*
pub fn commit(
    &self,
    update_ref: Option<&str>,
    author: &Signature<'_>,
    committer: &Signature<'_>,
    message: &str,
    tree: &Tree<'_>,
    parents: &[&Commit<'_>]
) -> Result<Oid, Error>				
				*/
				/*
pub fn find_tree(&self, oid: Oid) -> Result<Tree<'_>, Error>				

pub fn revparse_ext(
    &self,
    spec: &str
) -> Result<(Object<'_>, Option<Reference<'_>>), Error>
				*/
				repo.commit(
					Some("HEAD"),
					&sig,
					&sig,
					message,
					&tree,
					&[ parent ],
				)?;
				Ok(())
			},
			None => bail!( "No repo open for commit" ),
		}
	}
}





