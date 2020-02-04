extern crate cc;

fn main() {
    // println!("cargo:rustc-link-search=all=src");      // works like "rustc -L src ..."
    // println!("cargo:rustc-link-lib=dylib=doubler.o"); // works like "rustc -l doubler.o"

    cc::Build::new()
        .cpp(true)
        .file("src/doubler.cpp")
        // .cpp_link_stdlib("c++")
        // .cpp_set_stdlib("c++")
        .compile("libdoubler.a");
}