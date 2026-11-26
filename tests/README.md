# Integration tests

This folder contains integration tests for various parts of the crates.
All of the integration tests here need access to all `instrumentrs` related crates.

## Derive macro tests

The following folder constain tests for the derive macros.

- `derive-macro-tests-string-delimited`: Tests for the `Parameter` derive macro
  that tests structs and enums for `String`/`&str` instruments and
  where we have well-known delimiters between the strings.
- `derive-macro-tests-string-char-length`: Tests for the `Parameter` derive macro
  that tests structs and enums for `String`/`&str` instruments and
  where we have character length and position defined parameters.
