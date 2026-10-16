//! UI tests for the macros, using [`trybuild`].

#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/pass/*.rs");
}

/// Compiler diagnostics change between rust versions,
/// so only check them on recent stable.
///
/// Run with `TRYBUILD=overwrite` to update the expected output.
//
// NOTE: This must come before `#[test]`.
// Otherwise `#[test]` expands first and doesn't see the `ignore`,
// so the test still runs on older rust versions (seen on 1.64 and 1.71).
#[rustversion::attr(any(not(stable), before(1.99)), ignore = "diagnostics vary by compiler version")]
#[test]
fn ui_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/fail/*.rs");
}
