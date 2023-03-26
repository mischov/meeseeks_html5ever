defmodule MeeseeksHtml5ever.Mixfile do
  use Mix.Project

  @description "Meeseeks-specific NIF binding of html5ever using Rustler"
  @source_url "https://github.com/mischov/meeseeks_html5ever"
  @version "0.13.1"

  def project do
    [
      app: :meeseeks_html5ever,
      version: @version,
      elixir: "~> 1.12",
      deps: deps(),

      # Hex
      description: @description,
      package: package(),

      # HexDocs
      name: "MeeseeksHtml5ever",
      docs: docs()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:rustler_precompiled, "~> 0.6.1"},

      # Optional
      {:rustler, ">= 0.0.0", optional: true},

      # Docs
      {:ex_doc, "~> 0.24.0", only: :docs, runtime: false}
    ]
  end

  defp docs do
    [
      main: "MeeseeksHtml5ever",
      source_url: @source_url,
      source_ref: "v#{@version}",
      extras: ["CHANGELOG.md"]
    ]
  end

  defp package do
    [
      maintainers: ["Mischov"],
      licenses: ["MIT", "Apache-2.0"],
      files: [
        "lib",
        "native",
        "checksum-*.exs",
        "priv/.gitkeep",
        "mix.exs",
        "README.md",
        "CHANGELOG.md",
        "LICENSE-MIT",
        "LICENSE-APACHE"
      ],
      links: %{"GitHub" => @source_url}
    ]
  end
end
