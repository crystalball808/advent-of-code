defmodule Day1 do
  def foo do
    File.stream!("input1", :line)
    |> Stream.map(&String.trim/1)
    |> Stream.filter(&(byte_size(&1) > 0))
  end

  def part_one do
    {:ok, content} = File.read("input1")

    content
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&calibrate_value/1)
    |> Enum.sum()
  end

  def part_two do
    {:ok, content} = File.read("input1")

    content
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&calibrate_value_p2/1)
    |> Enum.sum()
    |> IO.puts()
  end

  @spec calibrate_value_p2(String.t()) :: integer()
  def calibrate_value_p2(line) do
    line
    |> words_to_digits()
    |> keep_first_and_last()
    |> String.to_integer()
  end

  @spec words_to_digits(String.t()) :: String.t()
  def words_to_digits(line) do
    # Find all occurrences of numbers either in digit or word representation
    Regex.scan(~r/(?=(\d|one|two|three|four|five|six|seven|eight|nine))/, line,
      capture: :all_but_first
    )
    |> List.flatten()
    |> Enum.map_join("", &number_to_digit/1)
  end

  @spec number_to_digit(String.t()) :: String.t()
  def number_to_digit(number) do
    case number do
      "one" -> "1"
      "two" -> "2"
      "three" -> "3"
      "four" -> "4"
      "five" -> "5"
      "six" -> "6"
      "seven" -> "7"
      "eight" -> "8"
      "nine" -> "9"
      other -> other
    end
  end

  @spec calibrate_value(String.t()) :: integer()
  def calibrate_value(line) do
    line
    |> String.replace(~r/\D/, "", global: true)
    |> keep_first_and_last()
    |> String.to_integer()
  end

  @spec keep_first_and_last(String.t()) :: String.t()
  def keep_first_and_last(str) do
    "#{String.first(str)}#{String.last(str)}"
  end
end
