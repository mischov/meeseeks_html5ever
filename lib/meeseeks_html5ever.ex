defmodule MeeseeksHtml5ever do
  @moduledoc """
  MeeseeksHtml5ever is intended for internal use by
  [Meeseeks](https://github.com/mischov/meeseeks), and parses HTML or XML into
  a `Meeseeks.Document`.
  """

  @doc"""
  Parses an HTML string into a `Meseeks.Document`.

  If the string of HTML is larger than 500 bytes, parses asynchronously as
  to maintain the recommended maximum NIF runtime of 1ms.
  """

  def parse_html(html) when byte_size(html) > 500 do
    parse_html_async(html)
  end

  def parse_html(html) do
    parse_html_sync(html)
  end

  defp parse_html_async(html) do
    MeeseeksHtml5ever.Native.parse_html_async(html)
    receive do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

  defp parse_html_sync(html) do
    case MeeseeksHtml5ever.Native.parse_html_sync(html) do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

  @doc"""
  Parses an XML string into a `Meseeks.Document`.

  If the string of XML is larger than 500 bytes, parses asynchronously as
  to maintain the recommended maximum NIF runtime of 1ms.
  """

  def parse_xml(xml) when byte_size(xml) > 500 do
    parse_xml_async(xml)
  end

  def parse_xml(xml) do
    parse_xml_sync(xml)
  end

  defp parse_xml_async(xml) do
    MeeseeksHtml5ever.Native.parse_xml_async(xml)
    receive do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

  defp parse_xml_sync(xml) do
    case MeeseeksHtml5ever.Native.parse_xml_sync(xml) do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end
end
