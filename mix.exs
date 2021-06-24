defmodule MeeseeksHtml5ever.Mixfile do
  use Mix.Project

  @version "0.13.0"

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
      {:ex_doc, ex_doc_version(), only: :docs, runtime: false}
    ]
  end

  defp ex_doc_version do
    if System.version() >= "1.7", do: "~> 0.19.0", else: "~> 0.18.0"
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
