use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::{Path, PathBuf};

fn main() {
    if cfg!(trybuild) {
        return;
    }

    let target = include_dir().unwrap().to_str().unwrap().to_string();
    println!("copy headers {:?}", fs::copy("xapian-bind.h", target + "/xapian-bind.h"));

    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources).file("xapian-bind.cc").flag_if_supported("-std=c++14").compile("xapian-rusty");

    println!("cargo:rustc-link-lib=xapianm");
}

fn include_dir() -> Result<PathBuf> {
    let target_dir = target_dir()?;
    Ok(target_dir.join("cxxbridge"))
}

fn target_dir() -> Result<PathBuf> {
    let mut dir = out_dir().and_then(canonicalize)?;
    loop {
        if dir.ends_with("target") {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        }
    }
}

fn out_dir() -> Result<PathBuf> {
    env::var_os("OUT_DIR").map(PathBuf::from).ok_or(Error::new(ErrorKind::Other, "oh no!"))
}

fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    Ok(fs::canonicalize(path)?)
}
