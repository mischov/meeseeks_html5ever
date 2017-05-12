defmodule MeeseeksHtml5everTest do
  use ExUnit.Case
  doctest MeeseeksHtml5ever

  test "parse div" do
    html = "<special:div>Hello, World!</div>"
    ret = {:ok,
           %{__struct__: :"Elixir.Meeseeks.Document",
             id_counter: 5,
             nodes: %{
               1 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [2, 3],
                      id: 1,
                      namespace: "",
                      parent: nil,
                      tag: "html"},
               2 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [],
                      id: 2,
                      namespace: "",
                      parent: 1,
                      tag: "head"},
               3 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [4],
                      id: 3,
                      namespace: "",
                      parent: 1,
                      tag: "body"},
               4 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [5],
                      id: 4,
                      namespace: "special",
                      parent: 3,
                      tag: "div"},
               5 => %{__struct__: :"Elixir.Meeseeks.Document.Text",
                      content: "Hello, World!",
                      id: 5,
                      parent: 4}},
             roots: [1]}}
    assert MeeseeksHtml5ever.parse(html) == ret
  end

  test "parse simple document" do
    html = "<html><head></head><body><div>Hello, World!</div></body></html>"
    ret = {:ok,
           %{__struct__: :"Elixir.Meeseeks.Document",
             id_counter: 5,
             nodes: %{
               1 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [2, 3],
                      id: 1,
                      namespace: "",
                      parent: nil,
                      tag: "html"},
               2 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [],
                      id: 2,
                      namespace: "",
                      parent: 1,
                      tag: "head"},
               3 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [4],
                      id: 3,
                      namespace: "",
                      parent: 1,
                      tag: "body"},
               4 => %{__struct__: :"Elixir.Meeseeks.Document.Element",
                      attributes: [],
                      children: [5],
                      id: 4,
                      namespace: "",
                      parent: 3,
                      tag: "div"},
               5 => %{__struct__: :"Elixir.Meeseeks.Document.Text",
                      content: "Hello, World!",
                      id: 5, parent: 4}},
             roots: [1]}}
    assert MeeseeksHtml5ever.parse(html) == ret
  end

  test "parse example" do
    html = File.read!("test/data/example.html")
    assert match?({:ok, _}, MeeseeksHtml5ever.parse(html))
  end

  test "parse unbalanced worst case" do
    html = String.duplicate("<div>", 100)
    assert match?({:ok, _}, MeeseeksHtml5ever.parse(html))
  end
end
