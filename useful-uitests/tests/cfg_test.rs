//! Check that `used_for_tests!` doesn't suppress anything under `#[cfg(test)]`.
//!
//! This uses [`ui_test`] instead of `trybuild`,
//! because `trybuild` never compiles with `--cfg test`.

use std::path::Path;

use ui_test::dependencies::DependencyBuilder;
use ui_test::{Config, ignore_output_conflict, run_tests};

fn main() -> ui_test::color_eyre::Result<()> {
    let mut config = Config::rustc("tests/ui_test/cfg_test");
    // default is relative to the current directory, which is the package (not workspace) root
    config.out_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("ui_test");
    // Expected errors are checked by `//~` annotations,
    // so avoid `.stderr` files that vary by compiler version.
    config.output_conflict_handling = ignore_output_conflict;
    config
        .comment_defaults
        .base()
        .set_custom("dependencies", DependencyBuilder::default());
    run_tests(config)
}
