// use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
// use std::io::ErrorKind;


fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    let vec = read_vec(f)?;
    
    // Part One
    let max_value = vec.iter().max();
    match max_value {
        Some(i) => println!( "Maximum value: {}", i ),
        None      => println!( "Vector is empty" ),
    }
    
    // Part Two
    let top3 = top_3(vec)?;
    let mut sum: u32 = 0;
    for number in top3 {
        sum = sum + number;
    }
    println!("The sum of top3 is: {}",sum);

    Ok(())
}

fn read_vec <R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let reader = BufReader::new(io);
    let mut v: Vec<u32> = Vec::new();
    let mut cumulative_sum: u32 = 0;

    for line in reader.lines() {
        let current_line: String = line?;
        if !current_line.is_empty() {
            cumulative_sum = cumulative_sum + current_line.trim().parse::<u32>().expect("Input not an Int");
        } else {
            v.push(cumulative_sum);
            cumulative_sum = 0;
        }
    }
    Ok(v)
}

fn top_3(vec: Vec<u32>) -> Result<Vec<u32>, Error>{
    let mut sorted_vec = vec;
    sorted_vec.sort();
    let len = sorted_vec.len();
    let top3 = sorted_vec[len-3..len].to_vec();
    Ok(top3)
}