defmodule MeeseeksHtml5ever.Native do
  @moduledoc false

  use Rustler, otp_app: :meeseeks_html5ever, crate: "meeseeks_html5ever_nif"

  defmodule NifNotLoadedError do
    @moduledoc false

    defexception message: "nif not loaded"
  end

  # parse_html
  def parse_html_async(_binary), do: err()
  def parse_html_sync(_binary), do: err()

  # parse_xml
  def parse_xml_async(_binary), do: err()
  def parse_xml_sync(_binary), do: err()

  defp err() do
    throw NifNotLoadedError
  end
end
