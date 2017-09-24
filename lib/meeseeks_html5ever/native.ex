defmodule MeeseeksHtml5ever.Native do
  @moduledoc false

  use Rustler, otp_app: :meeseeks_html5ever, crate: "meeseeks_html5ever_nif"

  defmodule NifNotLoadedError do
    @moduledoc false

    defexception message: "nif not loaded"
  end

  def parse_html(_binary), do: err()

  def parse_xml(_binary), do: err()

  defp err() do
    throw NifNotLoadedError
  end
end
