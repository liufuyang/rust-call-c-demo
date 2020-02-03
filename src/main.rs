extern "C" {
    fn doubler(x: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", doubler(1));
    }
}
