defmodule MeeseeksHtml5ever.Mixfile do
  use Mix.Project

  @version "0.4.4"

  def project do
    [app: :meeseeks_html5ever,
     name: "MeeseeksHtml5ever",
     version: @version,
     description: description(),
     elixir: "~> 1.3",
     deps: deps(),
     package: package(),
     source_url: "https://github.com/mischov/meeseeks_html5ever",
     docs: [main: "MeeseeksHtml5ever"],
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     compilers: [:rustler] ++ Mix.compilers(),
     rustler_crates: rustler_crates()]
  end

  def rustler_crates do
    [meeseeks_html5ever_nif: [
        path: "native/meeseeks_html5ever_nif",
        cargo: :system,
        default_features: false,
        features: [],
        mode: :release,
        # mode: (if Mix.env == :prod, do: :release, else: :debug),
      ]
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [{:rustler, "~> 0.9"},

     # docs
     {:ex_doc, "~> 0.14", only: :docs},
     {:markdown, github: "devinus/markdown", only: :docs}]
  end

  defp description do
    """
    Meeseeks-specific NIF binding of html5ever using Rustler.
    """
  end

  defp package do
    [maintainers: ["Mischov"],
     licenses: ["MIT", "Apache-2.0"],
     files: ["lib", "native", "mix.exs", "README.md", "LICENSE-MIT", "LICENSE-APACHE"],
     links: %{"GitHub" => "https://github.com/mischov/meeseeks_html5ever",
              "Docs" => "https://hexdocs.pm/meeseeks_html5ever"}]
  end

end
