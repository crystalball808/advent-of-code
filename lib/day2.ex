defmodule Day2 do
  @max_red 12
  @max_green 13
  @max_blue 14
  @number_color_regex ~r/\d+\s(green|red|blue)/

  def part_one do
    {:ok, content} = File.read("input2")

    content
    |> String.trim()
    |> String.split("\n")
    |> Enum.reduce([], &keep_possible_game_ids/2)
    |> Enum.sum()
  end

  @spec keep_possible_game_ids(String.t(), [integer()]) :: [integer()]
  defp keep_possible_game_ids(line, acc) do
    # Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    [game_name, game] = String.split(line, ":")

    is_possible =
      game
      |> String.split(";")
      |> are_all_rounds_valid()

    if is_possible do
      id =
        game_name
        |> String.replace(~r/\D/, "")
        |> String.to_integer()

      [id | acc]
    else
      acc
    end
  end

  # round looks similar to: 4 red, 1 green, 2 blue
  @spec are_all_rounds_valid([String.t()]) :: boolean()
  defp are_all_rounds_valid([]), do: true

  defp are_all_rounds_valid([round | rest]) do
    number_color_pairs =
      Regex.scan(@number_color_regex, round, capture: :first)
      |> List.flatten()

    if Enum.any?(number_color_pairs, &is_pair_invalid/1) do
      false
    else
      are_all_rounds_valid(rest)
    end
  end

  @spec is_pair_invalid(String.t()) :: boolean()
  defp is_pair_invalid(pair) do
    [number, color] = String.split(pair, " ")

    case color do
      "red" -> String.to_integer(number) > @max_red
      "green" -> String.to_integer(number) > @max_green
      "blue" -> String.to_integer(number) > @max_blue
      _ -> false
    end
  end
end
