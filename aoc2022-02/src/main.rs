use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("aoc2022-02/input.txt").expect("Expected file");
    let _hand_score = HashMap::from([("X",1),("Y",2),("Z",3)]);

    let lines = contents.split("\n");
    let mut score = 0;
    for line in lines {
        let round: Vec<&str> = line.trim().split(" ").collect(); // I don't care about what I do

        if round[0] == "A" {
            match round[1] {
                "X" => score+=3,
                "Y" => score+=6,
                "Z" => score+=0,
                _ => score+=score
            }
        }
        if round[0] == "B" {
            match round[1] {
                "X" => score+=0,
                "Y" => score+=3,
                "Z" => score+=6,
                _ => score+=score
            }
        }
        if round[0] == "C" {
            match round[1] {
                "X" => score+=6,
                "Y" => score+=0,
                "Z" => score+=3,
                _ => score+=score
            }
        }
        match round[1] {
            "X" => score+=1,
            "Y" => score+=2,
            "Z" => score+=3,
            _ => score+=score
        }
    }

    println!("Score {}",score);
}
