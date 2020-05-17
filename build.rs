fn main() {
    if cfg!(trybuild) {
        return;
    }

    let sources = vec!["lib.rs"];
    cxx_build::bridges(sources)
        .file("xapian-bind.cc")
        .compile("cxx-test-suite");

    println!("cargo:rustc-link-lib=xapianm");
}
