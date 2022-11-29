extern crate rusty_core;

fn main() {
    if let Err(e) = rusty_core::start() {
        print!("{:?}", e);
    }
}
