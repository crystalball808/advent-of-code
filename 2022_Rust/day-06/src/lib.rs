use std::{collections::HashMap, fs};

pub fn get_input() -> Result<Vec<char>, &'static str> {
    let input = fs::read_to_string("input.txt").expect("Failed to open file");
    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 4 {
        Err("At least 4 chars are required!")
    } else {
        Ok(chars)
    }
}

pub fn get_index_of_marker_end(chars: &Vec<char>, length_of_marker: usize) -> usize {
    let mut start = 0;

    let mut char_hash: HashMap<char, usize> = HashMap::new();
    loop {
        let end = start + length_of_marker - 1;
        let buffer_slice = chars.get(start..=end).unwrap();
        let mut all_unique = true;

        for (index, char) in buffer_slice.iter().enumerate() {
            match char_hash.insert(*char, index) {
                Some(index_of_present) => {
                    // shift to the next char after the duplicated one
                    start = start + index_of_present + 1;
                    all_unique = false;
                    break;
                }
                _ => {}
            }
        }

        if all_unique {
            return end
        } else {
            char_hash.clear();
        }
    }
}
