//! Under `#[cfg(test)]`, `used_for_tests!` should still check its input,
//! even though it expands to nothing.
//@compile-flags: --test

useful::used_for_tests!(1 + 2); //~ ERROR: expected a path
