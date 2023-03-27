# Meeseeks_Html5ever

[![MeeseeksHtml5ever version](https://img.shields.io/hexpm/v/meeseeks_html5ever.svg)](https://hex.pm/packages/meeseeks_html5ever)
![tests](https://github.com/mischov/meeseeks_html5ever/workflows/tests/badge.svg)

Originally a fork of Hansihe's [html5ever_elixir](https://github.com/hansihe/html5ever_elixir) that is more tightly coupled with [Meeseeks](https://github.com/mischov/meeseeks), Meeseeks_Html5ever has grown to include additional functionality, such as use of xml5ever for XML parsing.

## Compatibility

Meeseeks_Html5ever requires a minimum combination of Elixir 1.12.0 and Erlang/OTP 23.0, and is tested with a maximum combination of Elixir 1.14.0 and Erlang/OTP 25.0.

## Installation

Ensure Rust is installed, then add Meeseeks_Html5ever to your `mix.exs`:

```elixir
def deps do
  [
    {:meeseeks_html5ever, "~> 0.14.3"}
  ]
end
```

Finally, run `mix deps.get`.

## Dependencies

Meeseeks_Html5ever depends on the Rust library [html5ever](https://github.com/servo/html5ever), providing a Rustler-based NIF to interface with it.

You do not need to have Rust installed because the library will attempt to download a precompiled NIF file.

To force compilation you can either set the `MEESEEKS_HTML5EVER_BUILD` environment variable to `true` or `1`, or add the following application configuration

```elixir
config :meeseeks_html5ever, MeeseeksHtml5ever, build_from_source: true
```

If you want to force compilation you will need to have the Rust compiler [installed](https://www.rust-lang.org/en-US/install.html), and will need to add Rustler to your dependencies.

```elixir
def deps do
  [
    {:meeseeks_html5ever, "~> 0.14.3"},
    {:rustler, ">= 0.0.0", optional: true}
  ]
end
```

## Contributing

If you are interested in contributing please read the [contribution guidelines](CONTRIBUTING.md).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
