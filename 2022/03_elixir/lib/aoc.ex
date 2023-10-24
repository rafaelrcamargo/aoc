defmodule Aoc do
  def convert_to_priorities(dec) do
    if dec >= 97, do: dec - 96, else: dec - 38
  end

  def to_priorities(chars) do
    chars
    |> Enum.dedup()
    |> List.to_string()
    |> :binary.first()
    |> convert_to_priorities
  end

  def find_common(file) do
    file
    |> Enum.map(fn line -> String.split_at(line, Integer.floor_div(String.length(line), 2)) end)
    |> Enum.map(fn {f, s} -> {String.to_charlist(f), String.to_charlist(s)} end)
    |> Enum.map(fn {f, s} ->
      Enum.filter(f, fn e -> Enum.member?(s, e) end)
      |> to_priorities()
    end)
  end

  def find_badge(file) do
    file
    |> Enum.chunk_every(3)
    |> Enum.map(fn c -> Enum.map(c, &String.to_charlist/1) end)
    |> Enum.map(fn [a, b, c] ->
      Enum.filter(a, fn e -> Enum.member?(b, e) end)
      |> Enum.filter(fn e -> Enum.member?(c, e) end)
      |> to_priorities()
    end)
  end
end

file =
  case File.read("./assets/input.txt") do
    {:ok, file} ->
      file
      |> String.split()

    {:error, _} ->
      IO.puts("Error reading input file!")
  end

IO.write("Commons: ")

file
|> Aoc.find_common()
|> Enum.sum()
|> IO.inspect()

IO.write("Badges: ")

file
|> Aoc.find_badge()
|> Enum.sum()
|> IO.inspect()
