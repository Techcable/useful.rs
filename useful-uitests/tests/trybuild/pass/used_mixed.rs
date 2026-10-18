#![deny(unused)]
use core::fmt::Debug;
use core::hash::Hash;

fn foo() {}
struct Bar;
impl Bar {
    fn new() -> Self {
        Bar
    }
}
useful::used!(use Debug, let Bar::new, use Hash, use foo,);

fn main() {}
