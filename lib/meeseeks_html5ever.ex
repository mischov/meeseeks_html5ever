defmodule MeeseeksHtml5ever do
  @moduledoc """
  MeeseeksHtml5ever is intended for internal use by
  [Meeseeks](https://github.com/mischov/meeseeks), and parses HTML or XML into
  a `Meeseeks.Document`.
  """

  @doc"""
  Parses an HTML string into a `Meseeks.Document`.
  """
  def parse_html(html) do
    MeeseeksHtml5ever.Native.parse_html(html)
    receive do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end

  @doc"""
  Parses an XML string into a `Meseeks.Document`.
  """
  def parse_xml(xml) do
    MeeseeksHtml5ever.Native.parse_xml(xml)
    receive do
      {:html5ever_nif_result, :ok, result} ->
        {:ok, result}
      {:html5ever_nif_result, :error, err} ->
        {:error, err}
    end
  end
end
