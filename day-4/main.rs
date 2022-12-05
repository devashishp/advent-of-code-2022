use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;


fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
     // Part One
    let pairs = round_one(f);
    println!("The contains are: {}",pairs?);
    //573

    // Part Two
    let f = File::open("input.txt")?;
    let overlaps = round_two(f);
    println!("The overlaps are: {}",overlaps?);
    // 867
    Ok(())
}

fn round_one <R: Read>(io: R) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut pairs:u32 = 0;
    for line in reader.lines() {
        let input = line?;
        let numbers = process_line(input);
        pairs = pairs + get_pairs(numbers[0],numbers[1],numbers[2],numbers[3]);
    }
    Ok(pairs)
}

fn round_two <R: Read>(io: R) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut overlaps:u32 = 0;
    for line in reader.lines() {
        let input = line?;
        let numbers = process_line(input);
        overlaps = overlaps + get_overlaps(numbers[0],numbers[1],numbers[2],numbers[3]);
    }
    Ok(overlaps)
}

fn process_line (input: String) -> Vec<u32> {
    let mut numbers:Vec<u32> = Vec::new();
    let word: Vec<&str> = input.split(",").collect();
    let first: Vec<&str> = word[0].split("-").collect();
    let second: Vec<&str> = word[1].split("-").collect();
    numbers.push(first[0].parse().unwrap());
    numbers.push(first[1].parse().unwrap());
    numbers.push(second[0].parse().unwrap());
    numbers.push(second[1].parse().unwrap());
    numbers
}

fn get_pairs (lower1: u32,upper1: u32,lower2: u32,upper2:u32) -> u32 {
    let mut contains:u32 = 0;
    if lower1 >= lower2 && upper1 <= upper2 {
        contains = 1;
    } else if lower2 >= lower1 && upper2 <= upper1 {
        contains = 1;
    }
    // println!("Contains: {}",contains);
    contains
}

fn get_overlaps (lower1: u32,upper1: u32,lower2: u32,upper2:u32) -> u32 {
    let mut overlaps:u32 = 0;
    if upper1 >= lower2 && upper1 <= upper2 {
        overlaps = 1;
    } else if upper2 >= lower1 && upper2 <= upper1 {
        overlaps = 1;
    }
    // println!("Overlaps: {}",overlaps);
    overlaps
}