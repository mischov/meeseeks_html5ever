defmodule MeeseeksHtml5ever do
  @moduledoc """
  MeeseeksHtml5ever is intended for internal use by
  [Meeseeks](https://github.com/mischov/meeseeks), and parses HTML or XML into
  a `Meeseeks.Document`.

  By default this lib will try to use a precompiled NIF from the GitHub
  releases page. This way you don't need to have the Rust toolchain installed.
  In case no precompiled file is found and the Mix env is production then an
  error is raised.

  You can force the compilation to occur by setting the value of the
  `MEESEEKS_HTML5EVER_BUILD` environment variable to "true" or "1".
  Alternatively you can also set the application env `:build_from_source` to
  `true` in order to force the build:

  ```
  config :meeseeks_html5ever, MeeseeksHtml5ever, build_from_source: true
  ```
  """

  @doc """
  Parses an HTML string into a `Meeseeks.Document`.
  """
  def parse_html(html) do
    case MeeseeksHtml5ever.Native.parse_html(html) do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}

      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

  @doc """
  Parses an XML string into a `Meeseeks.Document`.
  """
  def parse_xml(xml) do
    case MeeseeksHtml5ever.Native.parse_xml(xml) do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}

      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end
end
