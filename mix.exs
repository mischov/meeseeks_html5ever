defmodule MeeseeksHtml5ever.Mixfile do
  use Mix.Project

  @version "0.13.2"
  @repo_url "https://github.com/un3481/meeseeks_html5ever"

  def project do
    [
      app: :meeseeks_html5ever,
      name: "MeeseeksHtml5ever",
      version: @version,
      elixir: "~> 1.11",
      description: "Meeseeks-specific NIF binding of html5ever using Rustler.",
      deps: deps(),
      docs: docs(),
      package: package(),
      source_url: @repo_url
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:rustler_precompiled, "~> 0.4"},
      {:rustler, ">= 0.0.0", optional: true},
      {:ex_doc, "~> 0.21.0", only: :docs, runtime: false}
    ]
  end

  defp docs do
    [
      main: "MeeseeksHtml5ever",
      extras: ["CHANGELOG.md"],
      skip_undefined_reference_warnings_on: ["CHANGELOG.md"],
      source_ref: "v#{@version}",
      source_url: @repo_url
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
        "LICENSE-APACHE",
        "LICENSE-MIT"
      ],
      links: %{"GitHub" => @repo_url}
    ]
  end
end
