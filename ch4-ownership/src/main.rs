use crate::only_once::only_once;

mod copy;
mod fn_and_ownership;
mod borrow;
mod only_once;
mod dangling;

fn main() {
    // At the end of the function, as s leaves it's scope
    // Rust will do a "drop" to s, which means this var is killed
    let s = String::from("hello");

    only_once()
}
