use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {

    if cfg!(trybuild) {
        return;
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::create_dir_all(dst.join("include")).unwrap();
    fs::copy("xapian-bind.h", dst.join("include/xapian-bind.h")).unwrap();

//    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}/include", dst.to_str().unwrap());

    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("xapian-bind.cc")
        .flag_if_supported("-std=c++14")
        .compile("xapian-rusty");

    println!("cargo:rustc-link-lib=xapianm");
}
