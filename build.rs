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
    let mut builder = cxx_build::bridges(sources);
    builder
        .file("xapian-bind.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-cpp")
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-unknown-warning-option")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-maybe-uninitialized")
        .opt_level_str("2");
    
    if let Ok(cxx_var) = env::var("CXX") {
        builder.compiler(cxx_var);
    } else {
        for compiler in &["g++", "clang++", "c++"] {
            if std::process::Command::new(compiler)
                .arg("--version")
                .output()
                .is_ok()
            {
                builder.compiler(compiler);
                break;
            }
        }
    }
    
    builder.compile("xapian-rusty");

    println!("cargo:rustc-link-lib=xapianm");
    println!("cargo:rustc-link-lib=m");
    
    println!("cargo:rerun-if-changed=xapian-bind.cc");
    println!("cargo:rerun-if-changed=xapian-bind.h");
    println!("cargo:rerun-if-changed=include/");
    
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
}
