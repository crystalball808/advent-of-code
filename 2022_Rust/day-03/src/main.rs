use std::collections::HashMap;
use std::fs;

fn find_shared_item(first_compartment: &str, second_compartment: &str) -> char {
    let mut first_comp_map: HashMap<char, bool> = HashMap::new();
    for item in first_compartment.chars() {
        first_comp_map.insert(item, true);
    }
    for item in second_compartment.chars() {
        if first_comp_map.contains_key(&item) {
            return item;
        }
    }
    panic!("No shared items in these compartments: {first_compartment} {second_compartment}")
}

fn get_item_priority(item: char) -> u32 {
    let item_ascii_code = item as u32;
    if item.is_ascii_uppercase() {
        item_ascii_code - 38
    } else {
        item_ascii_code - 96
    }
}

fn first_part(input: &String) {
    let mut sum_of_priorities: u32 = 0;
    for rucksack in input.split("\n") {
        let length_of_compartment = rucksack.len() / 2;
        let first_compartment = &rucksack[0..length_of_compartment];
        let second_compartment = &rucksack[length_of_compartment..];
        let shared_item = find_shared_item(first_compartment, second_compartment);
        let shared_item_priority = get_item_priority(shared_item);
        sum_of_priorities += shared_item_priority;
    }
    println!("First part: Sum of priorities: {sum_of_priorities}");
}
fn second_part(input: &String) {
    let mut sum_of_priorities: u32 = 0;

    let mut first_rucksack_map: HashMap<char, bool> = HashMap::new();
    let mut second_rucksack_map: HashMap<char, bool> = HashMap::new();

    for rucksack in input.split("\n") {
        if first_rucksack_map.is_empty() {
            for item in rucksack.chars() {
                first_rucksack_map.insert(item, true);
            }
        } else if second_rucksack_map.is_empty() {
            for item in rucksack.chars() {
                second_rucksack_map.insert(item, true);
            }
        } else {
            for item in rucksack.chars() {
                if first_rucksack_map.contains_key(&item) && second_rucksack_map.contains_key(&item)
                {
                    sum_of_priorities += get_item_priority(item);
                    first_rucksack_map.clear();
                    second_rucksack_map.clear();
                    break;
                }
            }
        }
    }

    println!("Second part: Sum of priorities: {sum_of_priorities}")
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    first_part(&input);
    second_part(&input);
}
