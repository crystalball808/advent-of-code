use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    let mut summed_calories: Vec<u32> = Vec::new();

    for block in input.split("\n\n") {
        let mut calories_amount: u32 = 0;
        for calory in block.split("\n") {
            let calory: u32 = calory.parse().unwrap();
            calories_amount += calory;
        }
        summed_calories.push(calories_amount);
    }

    summed_calories.sort_by(|a, b| b.cmp(a));
    println!("Top three: {:?}", &summed_calories[0..3]);
    println!("Total: {}", summed_calories[0..3].iter().sum::<u32>());
}
