defmodule Aoc do
  def int_or_zero(char) do
    case Integer.parse(char) do
      {i, _} -> i
      :error -> 0
    end
  end

  def last_or_head(head, tail) do
    case Enum.at(Enum.take(tail, -1), 0) do
      nil -> head
      n -> n
    end
  end

  def last_or(list, anchor) do
    case list do
      [] -> anchor
      [head | tail] -> last_or_head(head, tail)
    end
  end

  def contain(file) do
    file
    |> Enum.map(fn line -> line |> String.split(",") end)
    |> Enum.map(fn x -> Enum.map(x, fn y -> String.split(y, "-") end) end)
    |> Enum.map(fn x -> Enum.map(x, fn y -> Enum.map(y, fn z -> int_or_zero(z) end) end) end)
    |> Enum.map(fn [[a | b], [c | d]] ->
      [b, d] = [Enum.at(b, 0), Enum.at(d, 0)]
      if (a <= c and b >= d) or (a >= c and b <= d), do: 1, else: 0
    end)
  end

  def overlap(file) do
    file
    |> Enum.map(fn line -> line |> String.split(",") end)
    |> Enum.map(fn x -> Enum.map(x, fn y -> String.split(y, "-") end) end)
    |> Enum.map(fn x -> Enum.map(x, fn y -> Enum.map(y, fn z -> int_or_zero(z) end) end) end)
    |> Enum.map(fn [[a | b], [c | d]] ->
      [b, d] = [Enum.at(b, 0), Enum.at(d, 0)]
      if not (b < c or a > d), do: 1, else: 0
    end)
  end
end

case File.read("./assets/input.txt") do
  {:ok, file} ->
    file
    |> String.split()
    |> Aoc.contain()
    |> Enum.sum()
    |> IO.inspect()

  {:error, _} ->
    IO.puts("Error reading input file!")
end

case File.read("./assets/input.txt") do
  {:ok, file} ->
    file
    |> String.split()
    |> Aoc.overlap()
    |> Enum.sum()
    |> IO.inspect()

  {:error, _} ->
    IO.puts("Error reading input file!")
end
