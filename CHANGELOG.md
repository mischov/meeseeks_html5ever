# Changelog

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
