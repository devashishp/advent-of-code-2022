use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    //  Part One
    let num_stacks = num_stacks(f);
    let number:u32 = num_stacks?;
    let mut stacks: Vec<Vec<char>> = vec![Vec::<char>::new(); number as usize];
    let stack_ref: &mut Vec<Vec<char>> = &mut stacks;

    let f = File::open("input.txt")?;
    let result = parse_one(f, stack_ref);
    if result? != 0 {
        println!("Error!!");
    }
    println!("Stacks Final:");
    for item in &stacks {
        println!("{:?}",item);
    }
    let top_crates = get_tops(&stacks);
    println!("Final String: {}",top_crates);
    //WSFTMRHPP
    // Part Two

    let mut stacks: Vec<Vec<char>> = vec![Vec::<char>::new(); number as usize];
    let stack_ref: &mut Vec<Vec<char>> = &mut stacks;

    let f = File::open("input.txt")?;
    let result = parse_two(f, stack_ref);
    if result? != 0 {
        println!("Error!!");
    }
    println!("Stacks Final:");
    for item in &stacks {
        println!("{:?}",item);
    }
    let top_crates = get_tops(&stacks);
    println!("Final String: {}",top_crates);
    //GSLCMFBRP


    Ok(())
}

fn get_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut result: String = "".to_owned();
    for item in stacks {
        let value: char = item[0];
        result.push_str(&value.to_string());
    }
    result
}

fn num_stacks <R: Read>(io: R) -> Result<u32, Error> {
    let mut reader = BufReader::new(io);
    let mut first_line: String = String::new();
    let _line = reader.read_line(& mut first_line);
    let characters = first_line.chars().collect::<Vec<char>>();
    let num_stacks = (characters.len() + 1) / 4;
    
    // println!("num : {}",num_stacks as u32);
    Ok(num_stacks as u32)
}

fn parse_one <R: Read>(io: R, stacks: & mut Vec<Vec<char>>) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    let mut parse: bool = true;
    for line in reader.lines() {
        if parse == true {
            let input = line?;
            let characters = input.chars().collect::<Vec<char>>();
            let mut index = 0;
            for character in characters {
                if character.is_digit(10) {
                    //done
                    parse = false;
                } else if character.is_alphabetic() {
                    stacks[(index/4) as usize].push(character);
                }
                index = index+1;
            }
        } else {
            let input = line?;
            if input.len() == 0 {
                println!("Stacks Initially:");
                for item in stacks.iter(){
                    println!("{:?}",item);

                }
                continue;
            }
            let parts: Vec<&str> = input.split(" ").collect();
            let num_moved = parts[1].parse::<u32>().unwrap();
            let stack_from = parts[3].parse::<u32>().unwrap();
            let stack_to = parts[5].parse::<u32>().unwrap();
            let mut i:u32 = 0;
            while i < num_moved {
                let value: char = stacks[(stack_from -1) as usize].remove(0);
                // println!("{:?}\n moving {value} from {stack_from} to {stack_to}",stacks);
                stacks[(stack_to-1) as usize].insert(0,value);
                i = i+1;
            }
            // break;
            // println!("Moved: {i}");
        }
    }
    Ok(0)
}

fn parse_two <R: Read>(io: R, stacks: & mut Vec<Vec<char>>) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    
    let mut parse: bool = true;
    for line in reader.lines() {
        if parse == true {
            let input = line?;
            let characters = input.chars().collect::<Vec<char>>();
            let mut index = 0;
            for character in characters {
                if character.is_digit(10) {
                    //done
                    parse = false;
                } else if character.is_alphabetic() {
                    stacks[(index/4) as usize].push(character);
                }
                index = index+1;
            }
        } else {
            let input = line?;
            if input.len() == 0 {
                println!("Stacks Initially:");
                for item in stacks.iter(){
                    println!("{:?}",item);
                }
                continue;
            }
            let parts: Vec<&str> = input.split(" ").collect();
            let num_moved = parts[1].parse::<u32>().unwrap();
            let stack_from = parts[3].parse::<u32>().unwrap();
            let stack_to = parts[5].parse::<u32>().unwrap();
            let mut i:u32 = 0;
            let mut temp_stack:Vec<char> = Vec::new();
            while i < num_moved {
                let value: char = stacks[(stack_from -1) as usize].remove(0);
                // println!("{:?}\n moving {value} from {stack_from} to {stack_to}",stacks);
                temp_stack.insert(0,value);
                i = i+1;
            }
            // println!("Temp Stack: {:?}",temp_stack);
            for item in temp_stack {
                // println!("{:?}\n moving {i} from {stack_from} to {stack_to}",stacks);
                stacks[(stack_to-1) as usize].insert(0,item);
            }
            // break;
            // println!("Moved: {i}");
        }
    }
    Ok(0)
}