# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

Most changes include the relevant [jj](https://jj-vcs.dev) change ids in parens. An example of a change id is wuoxvnsw.

## Unreleased

### Added
- Accept arbitrary expressions after `let`, such as `let foo("")` to assist inference for functions with `impl Trait` arguments (zvosmzmq)
  - Fixes [issue #3]

[issue #3]: https://github.com/Techcable/useful.rs/issues/3

## v0.2.0 - 2026-10-17
Remove support for unprefixed paths.

### Removed
- *BREAKING*: Remove support for unprefixed paths, which were deprecated in v0.1.2 (oomoqlml)

### Fixed
- `used_for_tests!` now checks its input when `cfg(test)` is set, instead of silently accepting invalid syntax. (qknypvko)

## v0.1.2 - 2026-10-17
Support more types of items by using a `let` prefix.

Old behavior available with the `use` prefix.
Paths without prefixes are deprecated.

### Added
- Support associated functions, methods, and constants in `used!` with a `let` prefix, as in `used!(let Foo::new)`. (ymxposuv)
  - These cannot be imported with `use`, so they previously failed with "`Foo` is a struct, not a module".
- Allow an explicit `use` prefix for paths that are imported, as in `used!(use Debug, let Foo::new)`. (ymxposuv)
  - For compatibility, paths without a prefix are still imported with `use` (but this is deprecated, see below).

### Deprecated
- Deprecated paths without a `use` or `let` prefix in `used!`. (ymxposuv)
  - For compatibility, they still behave as if prefixed with `use`.
  - Support for this will be removed in version v0.2.0
  - Cannot be marked `#[deprecated(...)]` due to macro limitations.

### Fixed
- Fixed doctests failing on old rust versions by adding `#[allow(unused_extern_crates)]` (qrronrlo)

## v0.1.1 - 2026-10-16
Add `used_for_tests!` macro.

### Added
- Provide `used_for_tests!` macro, which marks items as used only on `#[cfg(not(test))]`. (qxuoyvst)
- More extensive testing (mainly UI tests) (mmvwwvsn, zwvulzqr)

## 0.1.0 - 2026-09-10
Initial release.

Provides only the `used!` and `used_for_docs!` macros.
