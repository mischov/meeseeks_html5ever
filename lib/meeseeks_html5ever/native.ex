defmodule MeeseeksHtml5ever.Native do
  @moduledoc false

  use Rustler, otp_app: :meeseeks_html5ever, crate: :meeseeks_html5ever_nif, mode: :release

  def parse_html(_binary), do: err()
  def parse_xml(_binary), do: err()

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
end
