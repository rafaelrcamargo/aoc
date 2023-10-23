defmodule Aoc do
  def int_or_zero(char) do
    case Integer.parse(char) do
      {i, _} -> i
      :error -> 0
    end
  end

  def snafu_to_int(char) do
    case char do
      "-" -> -1
      "=" -> -2
      char -> int_or_zero(char)
    end
  end

  def convert(snafu) do
    chars = snafu |> String.split("", trim: true) |> Enum.reverse()

    Stream.zip(Stream.iterate(0, &(&1 + 1)), chars)
    |> Enum.map(fn {k, v} -> {5 ** k, snafu_to_int(v)} end)
    # |> IO.inspect()
    |> Enum.map(fn {k, v} -> k * v end)
    # |> IO.inspect()
    |> Enum.sum()
  end

  def int_to_snafu(num) do
    case num do
      -1 -> "-"
      -2 -> "="
      num -> Integer.to_string(num)
    end
  end

  def to_snafu(num) do
    Stream.unfold({num, []}, fn {n, acc} ->
      if n > 0 do
        d = rem(n + 2, 5) - 2
        {[int_to_snafu(d) | acc], {Integer.floor_div(n - d, 5), [int_to_snafu(d) | acc]}}
      else
        nil
      end
    end)
    |> Enum.take(-1)
    |> Enum.at(0)
    |> Enum.join()
  end
end

case File.read("./assets/input.txt") do
  {:ok, file} ->
    file
    |> String.split()
    # |> IO.inspect()
    |> Enum.map(fn line -> Aoc.convert(line) end)
    # |> IO.inspect()
    |> Enum.sum()
    |> IO.inspect()
    |> Aoc.to_snafu()
    |> IO.inspect()

  {:error, _} ->
    IO.puts("Error reading input file!")
end
