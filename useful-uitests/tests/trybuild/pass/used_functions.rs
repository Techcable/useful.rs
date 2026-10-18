#![deny(unused)]

fn foo() {}
fn bar() {}
useful::used!(use foo, use bar,);

fn main() {}
