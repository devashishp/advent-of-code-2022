use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

// Making a BTree of the FS, size is 0 if directory, usize if file

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    const TOTAL_SPACE: usize = 70000000;
    const FREE_SPACE: usize = 30000000;

    let mut file_tree: BTreeMap<String, usize> = BTreeMap::new();

    let result = parse_tree(f, &mut file_tree);
    if result? != 0 {
        println!("Error!!");
    }
    // println!("Sizes Final:");
    // for item in &file_tree {
    //     println!("{:?}", item);
    // }
    let result = find_smaller(&mut file_tree, 100000);
    println!("Total size of small directories is {}", result);
    // 1491614

    // Part two:
    let delete = file_tree.clone();
    let filled = delete.get("/").unwrap();
    println!("Filled: {filled}");

    let to_free = FREE_SPACE - (TOTAL_SPACE - filled);
    println!(
        "Need to free up {} - {} = {}",
        FREE_SPACE,
        TOTAL_SPACE - filled,
        to_free
    );
    let result = find_smallest(delete, to_free);
    println!("Smallest Directory is {result}");
    //6400111

    Ok(())
}

fn find_smallest(file_tree: BTreeMap<String, usize>, to_free: usize) -> usize {
    let mut smallest: usize = usize::MAX;
    let mut to_return = 0;
    let directories: Vec<&usize> = file_tree.values().clone().collect();

    for item in directories {
        if item > &to_free {
            if smallest > (item - &to_free) {
                smallest = item - &to_free;
                to_return = *item;
            }
        }
    }
    return to_return;
}

fn parse_tree<R: Read>(io: R, file_tree: &mut BTreeMap<String, usize>) -> Result<u32, Error> {
    let mut directory_structure: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let mut count = 0;
    let mut total_size: usize = 0;

    let mut current_path = String::new();


    let mut subdirectories: Vec<String> = Vec::new();

    let reader = BufReader::new(io);

    for line in reader.lines() {
        let current_directory:String;
        count = count + 1;

        let input = line?.clone();
        if input.contains("$ cd") {
            let directory: Vec<&str> = input.split(" ").collect();

            if directory[2] == "/" {
                current_directory = directory[2].to_string();

                current_path.push_str(&current_directory);
                // println!("{current_directory}, {current_path}, {total_size}")
            } else if directory[2] != ".." {
                current_directory = directory[2].to_string();
                //   println!("Inserting {current_path} {total_size}");
                // }
                if !file_tree.contains_key(&current_path) {
                    file_tree.insert(current_path.clone(), total_size);
                } else {
                    // println!("Key exists {current_path}");
                }

                if !subdirectories.is_empty() {
                    directory_structure.insert(current_path.clone(), subdirectories.clone());
                }
                if current_path != "/".to_string() {
                    current_path.push('/');
                }

                current_path.push_str(&current_directory);
                total_size = 0;
                subdirectories = Vec::new();
            } else if directory[2] == ".." {
                if !file_tree.contains_key(&current_path) {
                    file_tree.insert(current_path.clone(), total_size);
                }

                let drain = current_path.rfind('/').unwrap_or(current_path.len());
                current_path = current_path.drain(..drain).collect();
                if current_path == "" {
                    current_path = "/".to_string();
                }
                // println!("Current after .. {}",current_path);
            }
        } else if input.contains("$ ls") {
            continue;
        } else if input.contains("dir ") {
            let directories: Vec<&str> = input.split(" ").collect();
            let mut clone = current_path.clone() + "/";
            if clone == "//" {
                clone = "/".to_string();
            }
            let full_dir_path = clone + &directories[1].to_string();
            subdirectories.push(full_dir_path.clone());
            // println!("Pushed {full_dir_path}");
        } else {
            let files: Vec<&str> = input.split(" ").collect();
            let current_size: usize = files[0].parse::<usize>().unwrap();
            total_size = total_size + current_size;
        }
    }
    // Last line
    file_tree.insert(current_path.clone(), total_size);
    // println!("{:?}",directory_structure);
    let _ = fix_size(file_tree, &directory_structure, "/".to_string());

    Ok(0)
}

fn find_smaller(file_tree: &mut BTreeMap<String, usize>, size: usize) -> usize {
    let mut sum: usize = 0;
    let directories: Vec<&usize> = file_tree.values().clone().collect();
    // println!("{:?}",directories);
    for item in directories {
        // println!("{}, {}",item,&size);
        if item <= &size {
            // println!("{item}");
            sum = sum + item;
        }
    }
    return sum;
}

fn fix_size(
    file_tree: &mut BTreeMap<String, usize>,
    directory_structure: &BTreeMap<String, Vec<String>>,
    directory: String,
) -> usize {
    let mut total_size: usize = *file_tree.get(&directory).unwrap();

    let values = directory_structure.get(&directory);
    match values {
        Some(values) => {
            if values.is_empty() {
                return total_size;
            }
            for item in values {
                // println!("Full {directory},{total_size}");
                total_size =
                    total_size + fix_size(file_tree, &directory_structure, item.to_string());
            }
        }
        None => {
            // println!("In none!");
            return total_size;
        }
    }
    file_tree.insert(directory, total_size);
    return total_size;
}
