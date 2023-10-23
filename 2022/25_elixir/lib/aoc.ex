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

  def from_snafu(snafu) do
    chars = snafu |> String.split("", trim: true) |> Enum.reverse()

    Stream.zip(Stream.iterate(0, &(&1 + 1)), chars)
    |> Enum.map(fn {k, v} -> {5 ** k, snafu_to_int(v)} end)
    |> Enum.map(fn {k, v} -> k * v end)
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
    stream =
      Stream.unfold(num, fn n ->
        if n > 0 do
          r = rem(n + 2, 5) - 2

          {
            int_to_snafu(r),
            Integer.floor_div(n - r, 5)
          }
        else
          nil
        end
      end)

    stream
    |> Enum.take(256)
  end
end

case File.read("./assets/input.txt") do
  {:ok, file} ->
    file
    |> String.split()
    |> Enum.map(&Aoc.from_snafu(&1))
    |> Enum.sum()
    |> Aoc.to_snafu()
    |> IO.puts()

  {:error, _} ->
    IO.puts("Error reading input file!")
end
