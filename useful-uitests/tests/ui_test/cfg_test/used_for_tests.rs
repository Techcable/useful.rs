//! Under `#[cfg(test)]`, `used_for_tests!` should expand to nothing,
//! so a function which isn't actually used by any tests is dead code.
//@compile-flags: --test
#![deny(unused)]

fn only_used_in_tests() {} //~ ERROR: never used
useful::used_for_tests!(only_used_in_tests);
