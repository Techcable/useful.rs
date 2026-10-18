#![deny(unused)]

use core::fmt::Display;
use std::collections::HashMap;
use std::collections::HashSet;

/// Like [`Foo::new`].
struct Foo;
impl Foo {
    fn new() -> Self {
        Foo
    }
}
useful::used_for_docs!(HashMap, use HashSet, let Foo::new, Display,);

/// This is almost as awesome as a [`HashMap`] or [`HashSet`], but doesn't implement [`Display`].
pub struct AwesomeStruct(pub u32, pub u32);

fn main() {}
