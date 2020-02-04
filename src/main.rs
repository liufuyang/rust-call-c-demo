extern "C" {
    pub fn doubler(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

fn main() {
    unsafe {
        println!("{}", doubler(1));
    }
}
