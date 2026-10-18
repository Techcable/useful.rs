//! Marking a function as used must not suppress lints inside its body.
#![deny(unused)]

fn foo() {
    let x = 3;
}
useful::used!(use foo);

fn main() {}
