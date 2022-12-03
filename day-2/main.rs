use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

// ROUND ONE
// B = Paper = Y = 2
// A = Rock = X = 1
// C = Scissor = Z = 3
// 0 if you lost, 3 if tie, 6 if won

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
     // Part One
    let score = round_one(f);
    println!("My Strategy: {}",score?);

    let f = File::open("input.txt")?;
    // Part Two
    let score = round_two(f);
    println!("Elf's Strategy: {}",score?);

    Ok(())
}

fn round_one <R: Read>(io: R) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut score : u32 = 0;

    for line in reader.lines() {
        let vec = line?;
        match vec.as_str() {
            "A X" => score = score + 4,
            "A Y" => score = score + 8,
            "A Z" => score = score + 3,
            "B X" => score = score + 1,
            "B Y" => score = score + 5,
            "B Z" => score = score + 9,
            "C X" => score = score + 7,
            "C Y" => score = score + 2,
            "C Z" => score = score + 6,
            _ => println!("Error parsing"),
        }
    }
    Ok(score)
}

// ROUND TWO
// X = lose
// Y = draw
// Z = win
// B = Paper 2
// A = Rock 1
// C = Scissor 3
// 0 if you lost, 3 if tie, 6 if won

fn round_two <R: Read>(io: R) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut score : u32 = 0;

    for line in reader.lines() {
        let vec = line?;
        match vec.as_str() {
            "A X" => score = score + 3,
            "A Y" => score = score + 4,
            "A Z" => score = score + 8,
            "B X" => score = score + 1,
            "B Y" => score = score + 5,
            "B Z" => score = score + 9,
            "C X" => score = score + 2,
            "C Y" => score = score + 6,
            "C Z" => score = score + 7,
            _ => println!("Error parsing"),
        }
    }
    Ok(score)
}
