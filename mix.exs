defmodule MeeseeksHtml5ever.Mixfile do
  use Mix.Project

  @version "0.13.1"

  def project do
    [
      app: :meeseeks_html5ever,
      name: "MeeseeksHtml5ever",
      version: @version,
      description: description(),
      elixir: "~> 1.7",
      deps: deps(),
      package: package(),
      source_url: "https://github.com/mischov/meeseeks_html5ever",
      docs: docs(),
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      compilers: Mix.compilers()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:rustler, "~> 0.22.0"},

      # docs
      {:ex_doc, "~> 0.21.0", only: :docs, runtime: false}
    ]
  end

  defp description do
    """
    Meeseeks-specific NIF binding of html5ever using Rustler.
    """
  end

  defp package do
    [
      maintainers: ["Mischov"],
      licenses: ["MIT", "Apache-2.0"],
      files: [
        "lib",
        "native",
        "priv/.gitkeep",
        "mix.exs",
        "README.md",
        "LICENSE-MIT",
        "LICENSE-APACHE"
      ],
      links: %{"GitHub" => "https://github.com/mischov/meeseeks_html5ever"}
    ]
  end

  defp docs do
    [main: "MeeseeksHtml5ever"]
  end
end
