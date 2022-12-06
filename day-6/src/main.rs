use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::collections::VecDeque;

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    //  Part One
    let message_len = parse_one(f,4)?;
    println!("Length: {message_len}");

    let f = File::open("input.txt")?;
    //  Part One
    let message_len = parse_one(f,14)?;
    println!("Length: {message_len}");


    Ok(())
}

fn parse_one <R: Read>(io: R, length:usize) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut text:Vec<char> = Vec::<char>::new();
    let mut deque:VecDeque<char> = VecDeque::with_capacity(length as usize);
    // println!("{:?}",deque);
    let mut count:u32 = 0;
    for line in reader.lines() {
        text = line?.chars().collect();
    }
    // println!("{:?}",text);
    for item in text {
        // let mut flag = false;
        if count < length as u32 {
            let _ = &deque.push_back(item);
            count = count +1;
            continue;
        } 
        let _ = &deque.pop_front();
        let _ = &deque.push_back(item);
        count = count + 1;
        let flag = check_unique(&deque,length)?;
        if flag == true {
            break;
        }

    }
    println!("First successful set: {:?}",deque);
    Ok(count)
}

fn check_unique (current:&VecDeque<char>, length:usize) -> Result<bool, Error> {
    let mut test = current.clone();
    let mut index = 0;
    let mut unique = true;
    while index < length {
        // println!("{:?}",test);
        let value:char = test.pop_front().unwrap();
        if test.contains(&value){
            // println!("Value: {}, Test: {:?}",value,test);
            unique = false;
            break;
        }
        index = index + 1;
    }
    // println!("Value: {}, Test: {:?}",unique,test);
    Ok(unique)
}