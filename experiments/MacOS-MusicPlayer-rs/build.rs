fn main() {
    if std::env::var("TARGET").unwrap().contains("-apple") {
//        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AVFAudio");
    }
}
