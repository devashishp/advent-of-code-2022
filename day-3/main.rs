use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;


fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
     // Part One
    let score = round_one(f);
    println!("The score is {}",score?);
    //8515

    // Part Two
    let f = File::open("input.txt")?;
    let score = round_two(f);
    println!("The score is {}",score?);
    // 2434
    Ok(())
}

fn round_one <R: Read>(io: R) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut score : u32 = 0;
    let mut line_no: u32 = 0;
    let mut last_line: u32 = 0;
    

    for line in reader.lines() {
        let input = line?;
        line_no = line_no + 1;
        let char_vec : Vec<char> = input.chars().collect();
        let mut first: Vec<char> = Vec::new();
        let mut second: Vec<char> = Vec::new();
        let mut i = 0;
        while i < char_vec.len()/2 {
            first.push(char_vec[i]);
            i = i + 1;
        }
        while i < char_vec.len() {
            second.push(char_vec[i]);
            i = i + 1;
        }
        
        for character in &first {
            if last_line == line_no {
                // Has been detected before
                continue;
            }
            // Silly Math to convert letter to numbers
            let digit = silly_math(*character);
            if second.contains(character) {
                score = score + digit;
                last_line = line_no;
                // println!("Line, character, digit, score: {},{},{},{}",line_no,character,digit,score);
            }
        }
        }
    Ok(score)
}

fn round_two <R: Read>(io: R) -> Result<u32, Error> {
    let mut reader = BufReader::new(io);
    let mut len = 1;
    let mut score = 0;
    loop {
        
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();

        // Read 3 lines as strings at a time, break if EOF
        len = reader.read_line(&mut line1)?;
        if len == 0{
            break;
        }
        len = reader.read_line(&mut line2)?;
        len = reader.read_line(&mut line3)?;

        // 3 Bitmaps to tell which characters are used
        let map1 = return_bitmap(line1);
        let map2 = return_bitmap(line2);
        let map3 = return_bitmap(line3);
        let mut result_bitmap: Vec<bool> = vec![false; 52];
        let mut i:u8 =0;
        // Bitwise and to get the intersection of the 3 bitmaps
        while i < 52 {
            result_bitmap[i as usize] = map1[i as usize] & map2[i as usize] & map3[i as usize];
            i = i + 1;
        }
        // println!("{:?}",result_bitmap);
        score = score + get_score(result_bitmap);
        
    }

    Ok(score)
}

// Function to get the problem-correct character code from ascii
fn silly_math (character: char) -> u32 {
    let mut digit = character as u32 - 38;
    if digit > 52 {
        digit = digit - 58;
    }
    digit
}


// Populates and returns a bitmap of unique characters
fn return_bitmap (line: String) -> Vec<bool> {
    let mut bitmap: Vec<bool> = vec![false; 52];
    let char_vec : Vec<char> = line.chars().collect();
    for character in &char_vec{
        if *character != '\n' {
            let index = silly_math(*character);
            let size_index = index as usize;
            bitmap[size_index -1] = true;
        }
    }
    bitmap
}

// Returns index of the intersection
fn get_score (result: Vec<bool>) -> u32 {
    let mut index: u32 = 0;
    for element in result{
        index = index + 1;
        if element == true {
            break;
        }
    }
    // println!("{}",index);
    index
}