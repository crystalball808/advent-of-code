import file_streams/file_stream
import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string

const input_path = "src/day_1/input.txt"

pub fn main() {
  let assert Ok(stream) = file_stream.open_read(input_path)

  let lists = collect_location_ids(stream, #([], []))

  let assert Ok(_) = first_part(lists)
  let assert Ok(_) = second_part(lists)
}

type Lists =
  #(List(Int), List(Int))

fn second_part(lists: Lists) {
  let #(first_list, second_list) = lists

  let appearences =
    second_list
    |> list.fold(dict.new(), fn(acc, n) {
      case dict.get(acc, n) {
        Ok(appearence) -> {
          acc
          |> dict.insert(n, appearence + 1)
        }
        Error(_) -> {
          acc
          |> dict.insert(n, 1)
        }
      }
    })

  let res =
    first_list
    |> list.fold(0, fn(acc, n) {
      let appearence =
        dict.get(appearences, n)
        |> result.unwrap(0)

      acc + n * appearence
    })
    |> int.to_string

  io.println("Day 2, first part: " <> res)

  Ok(Nil)
}

fn first_part(lists: Lists) {
  let #(first_list, second_list) = lists

  let first_sorted = list.sort(first_list, by: int.compare)
  let second_sorted = list.sort(second_list, by: int.compare)

  use zipped <- result.try(list.strict_zip(first_sorted, second_sorted))

  let res =
    list.fold(zipped, 0, fn(acc, pair) {
      acc + int.absolute_value(pair.0 - pair.1)
    })
    |> int.to_string

  io.println("Day 1, first part: " <> res)

  Ok(Nil)
}

fn collect_location_ids(stream: file_stream.FileStream, acc: Lists) -> Lists {
  case file_stream.read_line(stream) {
    Ok(line) -> {
      let assert Ok(#(first_number, second_number)) = parse_line(line)

      let new_acc = #([first_number, ..acc.0], [second_number, ..acc.1])
      collect_location_ids(stream, new_acc)
    }
    Error(_) -> {
      acc
    }
  }
}

/// Parse a line that should resemble a pattern "X   Y"
/// 
/// ## Examples
/// ```gleam
/// let assert Ok(#(36, 17)) = parse_line("36   17")
/// ```
fn parse_line(line: String) -> Result(#(Int, Int), Nil) {
  let assert [first_number, second_number] =
    line
    |> string.trim_end
    |> string.split(on: "   ")

  use first_number_parsed <- result.try(int.parse(first_number))
  use second_number_parsed <- result.try(int.parse(second_number))

  Ok(#(first_number_parsed, second_number_parsed))
}
