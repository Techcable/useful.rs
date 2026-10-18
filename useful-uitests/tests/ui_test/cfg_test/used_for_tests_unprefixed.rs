//! Under `#[cfg(test)]`, `used_for_tests!` should still check its input,
//! even though it expands to nothing.
//@compile-flags: --test

fn only_used_in_tests() {}
useful::used_for_tests!(only_used_in_tests); //~ ERROR: must be prefixed
