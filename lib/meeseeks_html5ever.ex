defmodule MeeseeksHtml5ever do
  @moduledoc """
  MeeseeksHtml5ever is intended for internal use by
  [Meeseeks](https://github.com/mischov/meeseeks), and parses HTML into a
  `Meeseeks.Document`.
  """

  @doc"""
  Parses an HTML string into a `Meseeks.Document`.

  If the string of HTML is larger than 500 bytes, parses asynchronously as
  to maintain the recommended maximum NIF runtime of 1ms.
  """

  def parse(html) when byte_size(html) > 500 do
    parse_async(html)
  end

  def parse(html) do
    parse_sync(html)
  end

  defp parse_async(html) do
    MeeseeksHtml5ever.Native.parse_async(html)
    receive do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

  defp parse_sync(html) do
    case MeeseeksHtml5ever.Native.parse_sync(html) do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

end
