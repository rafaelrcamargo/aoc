defmodule Aoc do
  alias List.Chars

  def convert_to_priorities(dec) do
    if dec >= 97, do: dec - 96, else: dec - 38
  end

  def read(file) do
    file
    |> Enum.map(fn line ->
      String.split_at(line, Integer.floor_div(String.length(line), 2))
    end)
    |> Enum.map(fn {first, second} ->
      first = String.to_charlist(first)
      second = String.to_charlist(second)

      Enum.filter(first, fn e ->
        Enum.member?(second, e)
      end)
      |> Enum.dedup()
      |> List.to_string()
      |> :binary.first()
      |> convert_to_priorities
    end)
  end
end

case File.read("./assets/input.txt") do
  {:ok, file} ->
    file
    |> String.split()
    |> Aoc.read()
    |> Enum.sum()
    |> IO.inspect()

  {:error, _} ->
    IO.puts("Error reading input file!")
end
