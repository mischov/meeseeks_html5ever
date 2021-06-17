# Meeseeks_Html5ever

[![MeeseeksHtml5ever version](https://img.shields.io/hexpm/v/meeseeks_html5ever.svg)](https://hex.pm/packages/meeseeks_html5ever)
![tests](https://github.com/mischov/meeseeks_html5ever/workflows/tests/badge.svg)

Originally a fork of Hansihe's [html5ever_elixir](https://github.com/hansihe/html5ever_elixir) that is more tightly coupled with [Meeseeks](https://github.com/mischov/meeseeks), Meeseeks_Html5ever has grown to include additional functionality, such as use of xml5ever for XML parsing.

## Compatibility

Meeseeks_Html5ever requires a minimum combination of Elixir 1.7.0 and Erlang/OTP 21.0, and is tested with a maximum combination of Elixir 1.12.0 and Erlang/OTP 24.0.

## Dependencies

Meeseeks_Html5ever depends on the Rust library [html5ever](https://github.com/servo/html5ever), and you will need to have the Rust compiler [installed](https://www.rust-lang.org/en-US/install.html).

## Installation

Ensure Rust is installed, then add Meeseeks_Html5ever to your `mix.exs`:

```elixir
def deps do
  [
    {:meeseeks_html5ever, "~> 0.12.1"}
  ]
end
```

Finally, run `mix deps.get`.

## Contribute

Contributions are very welcome, especially bug reports.

If submitting a bug report, please search open and closed issues first.

To make a pull request, fork the project, create a topic branch off of `master`, push your topic branch to your fork, and open a pull request.

If you're submitting a bug fix, please include a test or tests that would have caught the problem.

If you're submitting new features, please test and document as appropriate.

Before submitting a PR, please run `mix format`.

By submitting a patch, you agree to license your work under the license of this project.

### Running Tests

```
$ git clone https://github.com/mischov/meeseeks.git
$ cd meeseeks_html5ever
$ mix deps.get
$ mix test
```

### Building Docs

```
$ MIX_ENV=docs mix docs
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
