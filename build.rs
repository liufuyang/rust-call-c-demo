extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/doubler.h")
        .file("src/doubler.cpp")
        // .cpp_link_stdlib("c++")
        // .cpp_set_stdlib("c++")
        .shared_flag(true)
        .compile("libdoubler.a");
}