use std::fs;

use day_07::get_file_system;

fn main() {
    let log = fs::read_to_string("input.txt").expect("Failed to read the file");

    let file_system = get_file_system(log);
    if let Ok(folder) = file_system {
        println!("{}", folder);
    }
}
