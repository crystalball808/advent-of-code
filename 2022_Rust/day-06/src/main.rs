use day_06::{get_index_of_marker_end, get_input};

fn main() {
    let chars = match get_input() {
        Ok(chars) => chars,
        Err(message) => panic!("Error while trying to get input: {message}"),
    };

    let first_part_result = get_index_of_marker_end(&chars, 4);
    println!("answer to the first part: {}", first_part_result + 1);

    let second_part_result = get_index_of_marker_end(&chars, 14);
    println!("answer to the second part: {}", second_part_result + 1);
}
