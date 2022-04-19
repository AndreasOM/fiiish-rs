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

## In Progress

## TODO

- [ ] Verify the tag matches the version
- [ ] Ensure build number has changed
- [ ] Create windows package
- [ ] Create linux package

- [ ] Create alpha, beta, and final releases
	

## DONE

- [x] Create macos.app package
	- [x] Build number
	- [x] Info.plist
	- [x] aarch64 binary
	- [x] Add AppIcon compilation to build_binary action
	- [x] Handle fat binary


## Howto

```
gh workflow run build_runtime.yaml -f ref=COMMIT_HASH
gh workflow run pack_fiiish_data.yaml -f ref=COMMIT_HASH
gh workflow run pack_dummy_data.yaml -f ref=COMMIT_HASH
```
Note: These can run in parallel

Once done
```
gh workflow run create_packages.yaml -f ref=COMMIT_HASH
```

Once done
```
gh workflow run push_to_itchio.yaml -f ref=COMMIT_HASH
```




