defmodule MeeseeksHtml5ever.Native do
  @moduledoc false

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]
  env_config = Application.compile_env(:meeseeks_html5ever, MeeseeksHtml5ever, [])

  use RustlerPrecompiled,
    otp_app: :meeseeks_html5ever,
    crate: "meeseeks_html5ever_nif",
    mode: :release,
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build:
      System.get_env("MEESEEKS_HTML5EVER_BUILD") in ["1", "true"] or
        env_config[:build_from_source],
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version,
    nif_versions: ["2.15", "2.16"]

  def parse_html(_binary), do: err()
  def parse_xml(_binary), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
end
