//! The macro is documented as not working with `#[forbid(unused)]`.
//!
//! Currently this is a future-compat warning (rust-lang/rust#81670),
//! so deny that lint to make it an error.
#![deny(forbidden_lint_groups)]
#![forbid(unused)]

fn foo() {}
useful::used!(foo);

fn main() {}
