# fiiiish-release-tool

## About

Does all the necessary preparation to have github build a release.

- Prepare release
	- Checks workspace/git is clean
	- Removes -dev from version, and replaces by alpha/beta/[none]
	- Update Cargo.lock `cargo update --workspace --dry-run --verbose`
	- Commmits Cargo.toml (and other files as needed)
	- Pushes to git
	- Tags the release
	- Pushes the tag
- Prepare to continue with development
	- Bumps the local version patch/minor/major
	- Commits Cargo.toml
	- Pushes to git
