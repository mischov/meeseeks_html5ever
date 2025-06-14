# Changelog

## Unreleased

## v0.15.0 (2025-06-01)

### Compatibility

  * No longer support Elixir versions under 1.16 ir Elixir/OTP versions under 26
  * Support Elixir 1.16, 1.17 and 1.18 as well as Erlang/OTP 26 and 27

### Fixes

  * Upgrade old, invalid actions in workflows
  * Update compilation targets in release workflow
  * Fix rustler precompilation issue relating to NIF versions

## v0.14.3 (2023-03-27)

### Fixes

  * Add `RUSTLER_NIF_VERSION` to release workflow

## v0.14.2 (2023-03-26)

### Fixes

  * Undo adding NIF version 2.14 and make other minor adjustments to the `rustler_precompiled` settings

## v0.14.1 (2023-03-26)

### Fixes

  * Add missing NIF target 2.14

## v0.14.0 (2023-03-26)

### Compatibility

  * No longer support Elixir versions under 1.12 or Erlang/OTP versions under 23.0
  * Support Elixir 1.13 and 1.14 and Erlang/OTP 25.0

### Enhancements

  * Use Rust 2018 edition
  * Update to Rustler `v0.27`
  * Update to latest versions of Html5ever and Xml5ever
  * Use `rustler_precompiled` to precompile NIFs

### Fixes

  * Fix Rust formatting and clippy issues

## v0.13.1 (2021-10-20)

### Compatibility

  * Support compilation on Apple M1 devices

## v0.13.0 (2021-06-24)

### Compatibility

  * No longer support Elixir 1.6 or Erlang/OTP20
  * Support Elixir 1.12 and Erlang/OTP 24
  * Suppress warnings on x86_64-apple-darwin
  * Use Rustler v0.22

## v0.12.1 (2019-09-09)

### Enhancements

  * Use dirty scheduler for NIF instead of working asynchronously

## v0.12.0 (2019-09-08)

### Compatibility

  * No longer support Elixir 1.4, Elixir 1.5, or Erlang/OTP 19 (minumum tested compatiblity is now Elixir 1.6 and Erlang/OTP 20)
  * Support Elixir 1.9 and Erlang/OTP 22

### Fixes

  * Update to Rustler `v0.21`, which supports Erlang/OTP 22 (but requires a minumum Elixir version of 1.6)

## v0.11.1 (2019-06-28)

### Fixes

  * Improve error returned when provided with non-UTF-8 input

## v0.11.0 (2019-02-27)

### Compatibility

  * No longer support Elixir 1.3 (minimum tested compatibility is now Elixir 1.4 and Erlang/OTP 19.3)
  * Support Elixir 1.8

### Enhancements

  * Faster, more memory efficient encoding on Erlang/OTP 21
  * Update to latest versions of `html5ever`, `xml5ever`, and `rustler`

###

## v0.10.1 (2018-09-27)

### Enhancements

  * Test more Elixir+OTP combinations with Travis CI

## v0.10.0 (2018-07-06)

### Fixes

  * Update to Rustler `v0.18`, which supports OTP 21
  * Update types to work with Rustler `v0.17`

## v0.9.0 (2018-06-15)

### Enhancements

  * Add .formatters.exs and mix format project

### Fixes

  * Fix Dialyzer-related nif stub error

## v0.8.1 (2018-02-07)

### Fixes

  * Update to `rustler v0.16/0.16`, which works with OTP 20.2

## v0.8.0 (2017-09-24)

### Enhancements

  * Update to latest `html5ever` and `xml5ever` versions
  * Parse template elements (using template element as document fragment) instead of panicking

### Fixes

  * Remove synchronous parsing (it did not correctly handle panics, and broke the <1ms contract on first call)
  * Remove the panic on `mark_script_already_started`

## v0.7.0 (2017-09-23)

### Fixes

  * Modify `get_parent_and_index` so that an absent parent doesn't make it panic

## v0.6.1 (2017-06-29)

### Fixes

  * Update to `rustler v0.10.1/0.15.1`, which works with OTP 20

## v0.6.0 (2017-06-05)

### Breaking

  * Rename `MeeseeksHtml5ever.parse/1` to `MeeseeksHtml5ever.parse_html/1`

### Enhancements

  * Add `MeeseeksHtml5ever.parse_xml/1`
  * Add `xml5ever` as a dependency
  * Parse CDATA comments as `Meeseeks.Document.Data` nodes
