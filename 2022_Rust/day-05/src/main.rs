use std::{collections::VecDeque, fs, ops::RangeInclusive};

const CORRUPTED_FILE_MESSAGE: &str =
    "Corrupted file. Make sure you have it formatted the right way";

const STEP_BETWEEN_PACKAGE_NAMES: usize = 4;
const PACKAGE_NAME_OFFSET: usize = 1;

const ASCII_NUMBER_CODES: RangeInclusive<u8> = 48..=57;
fn is_not_ascii_number(ch: char) -> bool {
    let ascii_code = ch as u8;

    !ASCII_NUMBER_CODES.contains(&ascii_code)
}

fn parse_stacks(stacks: &str) -> Vec<VecDeque<char>> {
    let mut result: Vec<VecDeque<char>> = vec![];

    for level in stacks.split("\n") {
        for (idx, package_name) in level.chars().enumerate() {
            if idx % STEP_BETWEEN_PACKAGE_NAMES != PACKAGE_NAME_OFFSET {
                continue;
            }

            let stack_idx = idx / STEP_BETWEEN_PACKAGE_NAMES;

            let stack = result.get_mut(stack_idx);
            match stack {
                Some(st) => {
                    if package_name != ' ' && is_not_ascii_number(package_name) {
                        st.push_front(package_name);
                    }
                }
                None => {
                    let mut new_stack = VecDeque::new();
                    if package_name != ' ' && is_not_ascii_number(package_name) {
                        new_stack.push_front(package_name);
                    }
                    result.push(new_stack);
                }
            }
        }
    }

    return result;
}

const NUMBER_TO_INDEX: usize = 1;
fn operate_procedure_first_part(
    procedure: &str,
    mut stacks: Vec<VecDeque<char>>,
) -> Vec<VecDeque<char>> {
    for command in procedure.split("\n") {
        let mut command_parts = command.split(" ");
        command_parts.next();
        let package_count: u32 = command_parts
            .next()
            .expect(CORRUPTED_FILE_MESSAGE)
            .parse()
            .expect(CORRUPTED_FILE_MESSAGE);
        command_parts.next();
        let from_stack_index: usize = command_parts
            .next()
            .expect(CORRUPTED_FILE_MESSAGE)
            .parse()
            .expect(CORRUPTED_FILE_MESSAGE);
        let from_stack_index = from_stack_index - NUMBER_TO_INDEX;
        command_parts.next();
        let to_stack_index: usize = command_parts
            .next()
            .expect(CORRUPTED_FILE_MESSAGE)
            .parse()
            .expect(CORRUPTED_FILE_MESSAGE);
        let to_stack_index = to_stack_index - NUMBER_TO_INDEX;

        for _ in 0..package_count {
            let from_stack = stacks
                .get_mut(from_stack_index)
                .expect(CORRUPTED_FILE_MESSAGE);

            let package = match from_stack.pop_back() {
                Some(package) => package,
                None => {
                    panic!(
                        "No package from given stack: {:?}. Command: {}",
                        from_stack, command
                    )
                }
            };

            let to_stack = stacks
                .get_mut(to_stack_index)
                .expect(CORRUPTED_FILE_MESSAGE);

            to_stack.push_back(package);
        }
    }

    return stacks;
}
fn first_part(parsed_stacks: Vec<VecDeque<char>>, procedure: &str) -> String {
    let stacks = operate_procedure_first_part(procedure, parsed_stacks);

    let mut answer = String::new();

    for mut stack in stacks {
        let package = stack.pop_back().expect(CORRUPTED_FILE_MESSAGE);
        answer.push(package)
    }

    return answer;
}

fn operate_procedure_second_part(
    procedure: &str,
    mut stacks: Vec<VecDeque<char>>,
) -> Vec<VecDeque<char>> {
    for command in procedure.split("\n") {
        let mut command_parts = command.split(" ");
        command_parts.next();
        let package_count: usize = command_parts
            .next()
            .expect(CORRUPTED_FILE_MESSAGE)
            .parse()
            .expect(CORRUPTED_FILE_MESSAGE);
        command_parts.next();
        let from_stack_index: usize = command_parts
            .next()
            .expect(CORRUPTED_FILE_MESSAGE)
            .parse()
            .expect(CORRUPTED_FILE_MESSAGE);
        let from_stack_index = from_stack_index - NUMBER_TO_INDEX;
        command_parts.next();
        let to_stack_index: usize = command_parts
            .next()
            .expect(CORRUPTED_FILE_MESSAGE)
            .parse()
            .expect(CORRUPTED_FILE_MESSAGE);
        let to_stack_index = to_stack_index - NUMBER_TO_INDEX;

        let from_stack = stacks
            .get_mut(from_stack_index)
            .expect(CORRUPTED_FILE_MESSAGE);
        let mut packages = from_stack.split_off(from_stack.len() - package_count);
        let to_stack = stacks
            .get_mut(to_stack_index)
            .expect(CORRUPTED_FILE_MESSAGE);
        to_stack.append(&mut packages);
    }

    return stacks;
}
fn second_part(parsed_stacks: Vec<VecDeque<char>>, procedure: &str) -> String {
    let stacks = operate_procedure_second_part(procedure, parsed_stacks);

    let mut answer = String::new();

    for mut stack in stacks {
        let package = stack.pop_back().expect(CORRUPTED_FILE_MESSAGE);
        answer.push(package)
    }

    return answer;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    let mut stacks_and_procedure = input.split("\n\n");

    let stacks = stacks_and_procedure.next().expect(CORRUPTED_FILE_MESSAGE);
    let procedure = stacks_and_procedure.next().expect(CORRUPTED_FILE_MESSAGE);

    let parsed_stacks = parse_stacks(stacks);

    let second_answer = second_part(parsed_stacks, procedure);

    println!("second answer: {second_answer}")
}
