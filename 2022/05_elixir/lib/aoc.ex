defmodule Aoc do
  def part1(lines) do

    lines
    |> Enum.map(fn line ->
      IO.inspect(line)
      l1 = String.slice(line, 1, 1)
      IO.inspect(l1)
    end)

    lines
  end
end

case File.read("./assets/input.txt") do
  {:ok, file} ->
    file
    |> String.split("\n")
    |> Aoc.part1()
    |> IO.inspect()

  {:error, _} ->
    IO.puts("Error reading input file!")
end
