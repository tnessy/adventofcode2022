use std::cmp::Reverse;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Expected file");

    let lines = contents.split("\n");
    let mut elves = Vec::new();

    let mut curr_elf= 0;
    for line in lines {
        if line.trim().len() > 0 {
            let value: i32 = line.trim().parse().unwrap();
            curr_elf += value;
        } else {
            elves.push(curr_elf);
            curr_elf = 0;
        }
    }
    elves.sort_by_key(|w| Reverse(*w));
    println!("Largest calories: {}",elves[0]);
    println!("Top 3 combined: {}", elves[0..=2].iter().sum::<i32>())
}
