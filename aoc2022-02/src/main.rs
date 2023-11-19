use std::fs;

fn main() {
    let contents = fs::read_to_string("aoc2022-02/input.txt").expect("Expected file");

    let lines = contents.split("\n");
    let mut score = 0;
    for line in lines {
        let round: Vec<&str> = line.trim().split(" ").collect(); // I don't care about what I do

        if round[0] == "A" {
            match round[1] {
                "X" => score+=3+0,
                "Y" => score+=1+3,
                "Z" => score+=2+6,
                _ => score+=score
            }
        }
        if round[0] == "B" {
            match round[1] {
                "X" => score+=1+0,
                "Y" => score+=2+3,
                "Z" => score+=3+6,
                _ => score+=score
            }
        }
        if round[0] == "C" {
            match round[1] {
                "X" => score+=2+0,
                "Y" => score+=3+3,
                "Z" => score+=1+6,
                _ => score+=score
            }
        }
    }

    println!("Score {}",score);
}
