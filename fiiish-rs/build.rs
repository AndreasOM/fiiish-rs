use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

	println!("cargo:rerun-if-changed=fiiish-data.omar");
    if std::path::Path::new( "fiiish-data.omar" ).exists() {
    	println!("warning=Found fiiish-data.omar.");
    	println!("cargo:rustc-cfg=fiiish_with_fiiish_omar");
    } else {
    	println!("warning=Didn't find fiiish-data.omar!");
    }
	println!("cargo:rerun-if-changed=dummy-data.omar");
    if std::path::Path::new( "dummy-data.omar" ).exists() {
    	println!("warning=Found dummy-data.omar.");
    	println!("cargo:rustc-cfg=fiiish_with_dummy_omar");
    } else {
    	println!("warning=Didn't find dummy-data.omar!");
    }
}
