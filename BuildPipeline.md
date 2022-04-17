# Build Pipeline



Assets:
- fiiish-content: The old content from original Fiiish, with some restrictions on licensing.
- dummy-data: New very simple content from of any licensing restrictions.


Data: (aka converted Assets ready to be packaged)
- dummy-data: Same as above. This _should_ be converted from dummy-content.
- fiiish-data: Converted from fiiish-content. Same licensing restrictions. :(

Runtime: (some might call it 'engine')
- fiiish-rs: The rust source code.


WE ARE HERE -> Magic Build process: Mangling all of the above into releasable packages.





## Ideal

If things are to hard/big to handle, slice them up.

- fiiish-content -> fiiish-data
- dummy-content -> dummy-data
- [build_runtime] fiiish-rs (source) -> runtime
- [create_packages] runtime + (optional) fiiish-data + dummy-data -> package (osx, windows, linux, etc)


### Structure
/temp/
	/parts/
	/package/

## TODO

- [ ] Create macos.app package
	- [ ] Build number
	- [ ] Info.plist
	- [ ] aarch64 binary
	- [ ] Add AppIcon compilation to build_binary action
	

