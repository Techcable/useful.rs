# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

Most changes include the relevant [jj](https://jj-vcs.dev) change ids in parens. An example of a change id is wuoxvnsw.

## Unreleased

## v0.1.1 - 2026-10-16
Add `used_for_tests!` macro.

### Added
- Provide `used_for_tests!` macro, which marks items as used only on `#[cfg(not(test))]`. (qxuoyvst)
- More extensive testing (mainly UI tests) (mmvwwvsn, zwvulzqr)

## 0.1.0 - 2026-09-10
Initial release.

Provides only the `used!` and `used_for_docs!` macros.
