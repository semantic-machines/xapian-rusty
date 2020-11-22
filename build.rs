use cxx_build::CFG;
use std::env;
use std::path::Path;

fn main() {
    if cfg!(trybuild) {
        return;
    }

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let xapian_include_dir = Path::new(&manifest_dir).join("include");
    CFG.exported_header_dirs.push(&xapian_include_dir);

    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("xapian-bind.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-declarations")
        .compile("xapian-rusty");

    println!("cargo:rustc-link-lib=xapianm");
    println!("cargo:rustc-link-lib=m");
}
