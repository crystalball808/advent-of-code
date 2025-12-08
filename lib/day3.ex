defmodule Day3 do
  def part_one do
    {:ok, content} = File.read("input3")

    lines_with_index = content |> String.trim() |> String.split("\n") |> Enum.with_index()

    is_part = fn <<grapheme>> ->
      grapheme !== ?. and grapheme not in ?0..?9
    end

    part_coordinates = get_part_coordinates(lines_with_index, is_part)

    numbers_with_parts = get_numbers_with_parts(lines_with_index, part_coordinates)

    numbers_with_parts |> Enum.map(&String.to_integer/1) |> Enum.sum()
  end

  def get_numbers_with_parts(lines_with_index, part_coordinates, numbers_with_parts \\ [])
  def get_numbers_with_parts([], _part_coordinates, numbers_with_parts), do: numbers_with_parts

  def get_numbers_with_parts(
        [{line, index} | rest_lines],
        part_coordinates,
        numbers_with_parts
      ) do
    filter_condition = fn number_position ->
      has_part?(number_position, Map.get(part_coordinates, index - 1)) or
        has_part?(number_position, Map.get(part_coordinates, index)) or
        has_part?(number_position, Map.get(part_coordinates, index + 1))
    end

    numbers_with_parts_from_line =
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.filter(filter_condition)
      |> Enum.map(fn number_position ->
        String.slice(line, elem(number_position, 0), elem(number_position, 1))
      end)

    get_numbers_with_parts(
      rest_lines,
      part_coordinates,
      numbers_with_parts_from_line ++ numbers_with_parts
    )
  end

  defp has_part?({number_start, number_length}, index_list) do
    if index_list === nil do
      false
    else
      number_end = number_start + number_length - 1
      range_to_check = (number_start - 1)..(number_end + 1)

      Enum.any?(index_list, fn index -> index in range_to_check end)
    end
  end

  def get_part_coordinates(lines_with_index, condition, line_parts_map \\ %{})
  def get_part_coordinates([], _condition, line_parts_map), do: line_parts_map

  def get_part_coordinates([{line, index} | rest_lines], condition, line_parts_map)
      when is_function(condition, 1) do
    indexes_of_parts = line |> get_indexes_of_parts(condition)

    if length(indexes_of_parts) > 0 do
      new_lines_map = Map.put(line_parts_map, index, indexes_of_parts)

      get_part_coordinates(rest_lines, condition, new_lines_map)
    else
      get_part_coordinates(rest_lines, condition, line_parts_map)
    end
  end

  defp get_indexes_of_parts(line, condition) when is_function(condition, 1) do
    line
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.reduce([], fn {grapheme, index}, acc ->
      if condition.(grapheme) do
        [index | acc]
      else
        acc
      end
    end)
  end

  def part_two do
    {:ok, content} = File.read("input3")

    lines_with_index =
      content
      |> String.trim()
      |> String.split("\n")
      |> Enum.with_index()

    gear_coordinates =
      get_part_coordinates(lines_with_index, fn <<grapheme>> ->
        grapheme === ?*
      end)

    IO.inspect(gear_coordinates, label: "gear_coordinates")

    gear_ratios =
      gear_coordinates
      |> Map.to_list()
      |> get_ratios_of_gears(lines_with_index)

    gear_ratios |> Enum.sum()
  end

  def get_ratios_of_gears(gear_coordinates, lines_with_index)
      when is_list(gear_coordinates) and is_list(lines_with_index) do
    [{line_index, char_index} | other_lines_with_gears] = gear_coordinates
    IO.inspect(line_index, label: "line_index")
    IO.inspect(char_index, label: "char_index")

    IO.inspect(lines_with_index, label: "lines_with_index")

    prev_line = Enum.at(lines_with_index, line_index - 1)
    IO.inspect(prev_line, label: "prev_line")

    next_line = Enum.at(lines_with_index, line_index + 1)
    IO.inspect(next_line, label: "next_line")

    [1]
  end
end
