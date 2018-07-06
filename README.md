# Meeseeks_Html5ever

[![Build Status](https://travis-ci.org/mischov/meeseeks_html5ever.svg?branch=master)](https://travis-ci.org/mischov/meeseeks_html5ever)
[![MeeseeksHtml5ever version](https://img.shields.io/hexpm/v/meeseeks_html5ever.svg)](https://hex.pm/packages/meeseeks_html5ever)

A fork of Hansihe's [html5ever_elixir](https://github.com/hansihe/html5ever_elixir) that is more tightly coupled with [Meeseeks](https://github.com/mischov/meeseeks).

## Dependencies

Meeseeks_Html5ever depends on the Rust library [html5ever](https://github.com/servo/html5ever), and you will need to have the Rust compiler [installed](https://www.rust-lang.org/en-US/install.html).

## Installation

Ensure Rust is installed, then add Meeseeks_Html5ever to your `mix.exs`:

```elixir
def deps do
  [
    {:meeseeks_html5ever, "~> 0.10.0"}
  ]
end
```

Finally, run `mix deps.get`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
