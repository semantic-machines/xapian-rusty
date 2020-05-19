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

    let cargo_dir = cargo_dir().unwrap().to_str().unwrap().to_string();
    println!("@cargo_dirr={:?}", cargo_dir);

    let out_dir = out_dir().unwrap().to_str().unwrap().to_string();
    println!("@out dir={:?}", out_dir);

    let target_dir = include_dir().unwrap().to_str().unwrap().to_string();
    println!("@target dir={}", target_dir);
    fs::create_dir_all(target_dir.to_owned()).unwrap();
    fs::create_dir_all(target_dir.to_owned() + "/src").unwrap();

    println!("@copy bind header {:?}", fs::copy("xapian-bind.h", target_dir.to_owned() + "/xapian-bind.h"));

    fs::create_dir_all(target_dir.to_owned() + "/xapian").unwrap();
    println!("@copy headers {:?}", fs::copy("include/xapian.h", target_dir.to_owned() + "/xapian.h"));
    println!("@copy headers {:?}", copy_dir("include/xapian", target_dir.to_owned() + "/xapian"));

    if !cargo_dir.is_empty() {
        let c_from = target_dir.to_owned() + "/" + &cargo_dir + "/src/lib.rs.h";
        println!("@gen files {} {:?}", &c_from, std::os::unix::fs::symlink(c_from.to_owned(), target_dir.to_owned() + "/src/lib.rs.h"));
    }

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

fn cargo_dir() -> Result<PathBuf> {
    let dir = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).ok_or(Error::new(ErrorKind::Other, "fail read env var")).and_then(canonicalize)?;
    let mut new_dir = PathBuf::default();
    let mut is_found_cargo = false;
    for el in dir.iter() {
        if el == ".cargo" {
            is_found_cargo = true;
        }
        if is_found_cargo {
            new_dir.push(el);
        }
    }
    Ok(new_dir)
}

fn out_dir() -> Result<PathBuf> {
    env::var_os("OUT_DIR").map(PathBuf::from).ok_or(Error::new(ErrorKind::Other, "oh no!"))
}

fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    Ok(fs::canonicalize(path)?)
}

pub fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}
