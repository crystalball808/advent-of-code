use std::fs;
use unicode_segmentation::UnicodeSegmentation;

const MY_ROCK_SHAPE: &str = "X";
const MY_PAPER_SHAPE: &str = "Y";
const MY_SCISSORS_SHAPE: &str = "Z";

const ENEMY_ROCK_SHAPE: &str = "A";
const ENEMY_PAPER_SHAPE: &str = "B";
const ENEMY_SCISSORS_SHAPE: &str = "C";

fn first_part(input: &String) {
    let mut total_score: u32 = 0;

    for round in input.split("\n") {
        if round.len() == 0 {
            continue;
        }
        let graphemes = round.graphemes(true).collect::<Vec<&str>>();

        let enemy_shape = *graphemes.get(0).expect("Unable to read shapes");
        let my_shape = *graphemes.get(2).expect("Unable to read shapes");

        // Points based on shape
        if my_shape == MY_ROCK_SHAPE {
            total_score += 1
        }
        if my_shape == MY_PAPER_SHAPE {
            total_score += 2
        }
        if my_shape == MY_SCISSORS_SHAPE {
            total_score += 3
        }

        // Points based on round result

        // Draw
        if my_shape == MY_ROCK_SHAPE && enemy_shape == ENEMY_ROCK_SHAPE {
            total_score += 3
        }
        if my_shape == MY_PAPER_SHAPE && enemy_shape == ENEMY_PAPER_SHAPE {
            total_score += 3
        }
        if my_shape == MY_SCISSORS_SHAPE && enemy_shape == ENEMY_SCISSORS_SHAPE {
            total_score += 3
        }

        // Win
        if my_shape == MY_ROCK_SHAPE && enemy_shape == ENEMY_SCISSORS_SHAPE {
            total_score += 6
        }
        if my_shape == MY_PAPER_SHAPE && enemy_shape == ENEMY_ROCK_SHAPE {
            total_score += 6
        }
        if my_shape == MY_SCISSORS_SHAPE && enemy_shape == ENEMY_PAPER_SHAPE {
            total_score += 6
        }
    }
    println!("First task. Total score: {total_score}")
}

const NEED_TO_LOOSE: &str = "X";
const NEED_TO_DRAW: &str = "Y";
const NEED_TO_WIN: &str = "Z";

fn second_part(input: &String) {
    let mut total_score: u32 = 0;
    for round in input.split("\n") {
        if round.len() == 0 {
            continue;
        }
        let graphemes = round.graphemes(true).collect::<Vec<&str>>();

        let enemy_shape = *graphemes.get(0).expect("Unable to read shapes");
        let need_to = *graphemes.get(2).expect("Unable to read shapes");

        if need_to == NEED_TO_WIN {
            total_score += 6;
            if enemy_shape == ENEMY_ROCK_SHAPE {
                total_score += 2 // my move == paper
            }
            if enemy_shape == ENEMY_PAPER_SHAPE {
                total_score += 3 // my move == scissors
            }
            if enemy_shape == ENEMY_SCISSORS_SHAPE {
                total_score += 1 // my move == rock
            }
        }
        if need_to == NEED_TO_DRAW {
            total_score += 3;
            if enemy_shape == ENEMY_ROCK_SHAPE {
                total_score += 1
            }
            if enemy_shape == ENEMY_PAPER_SHAPE {
                total_score += 2
            }
            if enemy_shape == ENEMY_SCISSORS_SHAPE {
                total_score += 3
            }
        }
        if need_to == NEED_TO_LOOSE {
            if enemy_shape == ENEMY_ROCK_SHAPE {
                total_score += 3 // my move == scissors
            }
            if enemy_shape == ENEMY_PAPER_SHAPE {
                total_score += 1 // my move == rock
            }
            if enemy_shape == ENEMY_SCISSORS_SHAPE {
                total_score += 2 // my move == paper
            }
        }
    }

    println!("Second task. Total score: {total_score}")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    first_part(&input);
    second_part(&input);
}
