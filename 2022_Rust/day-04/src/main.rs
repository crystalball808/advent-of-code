use std::fs;

const CORRUPTED_FILE_MESSAGE: &str =
    "Corrupted file. Make sure you have it formatted the right way";

fn get_elf_range(elf: &str) -> (u32, u32) {
    let mut raw_scope = elf.split("-");

    let start: u32 = raw_scope
        .next()
        .expect(CORRUPTED_FILE_MESSAGE)
        .parse()
        .expect(CORRUPTED_FILE_MESSAGE);
    let end: u32 = raw_scope
        .next()
        .expect(CORRUPTED_FILE_MESSAGE)
        .parse()
        .expect(CORRUPTED_FILE_MESSAGE);

    (start, end)
}

fn first_part(input: &String) {
    let mut count: u32 = 0;
    for pair in input.split("\n") {
        let mut elves = pair.split(",");
        let first_elf = elves.next().expect(CORRUPTED_FILE_MESSAGE);
        let second_elf = elves.next().expect(CORRUPTED_FILE_MESSAGE);

        let first_elf_range = get_elf_range(first_elf);
        let second_elf_range = get_elf_range(second_elf);

        if second_elf_range.0 >= first_elf_range.0 && second_elf_range.1 <= first_elf_range.1
            || first_elf_range.0 >= second_elf_range.0 && first_elf_range.1 <= second_elf_range.1
        {
            count += 1;
        }
    }

   println!("First part result: {}", count);
}

fn second_part(input: String) {
    let mut count: u32 = 0;

    for pair in input.split("\n") {
        let mut elves = pair.split(",");
        let first_elf = elves.next().expect(CORRUPTED_FILE_MESSAGE);
        let second_elf = elves.next().expect(CORRUPTED_FILE_MESSAGE);

        let first_elf_range = get_elf_range(first_elf);
        let second_elf_range = get_elf_range(second_elf);

        if first_elf_range.0 <= second_elf_range.1 && second_elf_range.0 <= first_elf_range.1 {
            count += 1;
        }
    }

    println!("Second part result: {}", count);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    first_part(&input);
    second_part(input);
}
