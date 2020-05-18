fn main() {
    if cfg!(trybuild) {
        return;
    }

    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("xapian-bind.cc")
        .flag_if_supported("-std=c++14")
        .compile("xapian-rusty");

    println!("cargo:rustc-link-lib=xapianm");
}
