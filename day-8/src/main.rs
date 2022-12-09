use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    let mut tree_map: Vec<Vec<u32>> = Vec::new();
    let _ = parse_tree(f, &mut tree_map);
    // println!("{:?}",tree_map);
    let result = get_visible_trees(&tree_map)?;
    println!("The number of visible trees are: {result}");

    let result = get_best_tree(&tree_map)?;
    println!("The best tree has a score of: {result}");
    Ok(())
}

fn parse_tree<R: Read>(io: R, tree_map: &mut Vec<Vec<u32>>) -> Result<u32, Error> {
    let reader = BufReader::new(io);
    // let mut index: usize = 0;
    for line in reader.lines() {
        let input1 = line?;
        let input = input1.chars().flat_map(|c| c.to_digit(10)).collect();
        tree_map.push(input);
        // index = index + 1;
        // println!("{:?}",input);
    }
    return Ok(0);
}

fn get_visible_trees(tree_map: &Vec<Vec<u32>>) -> Result<u32, Error> {
    let mut result: u32 = 0;
    let mut visible: Vec<Vec<u8>> = vec![vec![0;99];99];

    for i in 0..tree_map.len() {
        let mut prev_max:i32 = -1;
        for j in 0..tree_map[i].len(){
            if (tree_map[i][j] as i32) > prev_max as i32 {
                prev_max = tree_map[i][j] as i32;
                visible[i][j] = 1;
            }
        }
    }
    for i in 0..tree_map.len() {
        let mut prev_max:i32 = -1;
        for j in (0..tree_map[i].len()).rev(){
            if (tree_map[i][j] as i32) > prev_max as i32 {
                prev_max = tree_map[i][j] as i32;
                visible[i][j] = 1;
            }
        }
    }
    for j in 0..tree_map[0].len() {
        let mut prev_max:i32 = -1;
        for i in 0..tree_map.len(){
            if (tree_map[i][j] as i32) > prev_max as i32 {
                prev_max = tree_map[i][j] as i32;
                visible[i][j] = 1;
            }
        }
    }
    for j in 0..tree_map[0].len() {
        let mut prev_max:i32 = -1;
        for i in (0..tree_map.len()).rev(){
            if (tree_map[i][j] as i32) > prev_max as i32 {
                prev_max = tree_map[i][j] as i32;
                visible[i][j] = 1;
            }
        }
    }

    for item in visible {
        result = result + item.iter().filter(|x| *x == &1).count() as u32;
    }

    Ok(result as u32)
}

fn get_best_tree(tree_map: &Vec<Vec<u32>>) -> Result<usize, Error> {
    let mut best_score:usize = 0;
    let mut tree_scores: Vec<Vec<usize>> = vec![vec![0;99];99];

    for i in 0..tree_map.len() {

        for j in 0..tree_map[0].len() {
            let ldistance = get_ldistance(&tree_map, i, i, j);
            let rdistance = get_rdistance(&tree_map, i, i, j);
            let udistance = get_udistance(&tree_map, i, j, j);
            let ddistance = get_ddistance(&tree_map, i, j, j);
            tree_scores[i][j] = ldistance*rdistance*ddistance*udistance;
            // println!("i:{},j:{},ldistance:{},rdistance{},udistance{},ddistance{}",i,j,ldistance,rdistance,udistance,ddistance);          
        }
    }
    // println!("{:?}",tree_scores);
    for element in tree_scores {
       if best_score < *element.iter().max().unwrap(){
            best_score = *element.iter().max().unwrap();
       } 
    }

    Ok(best_score)
}

fn get_ldistance(tree_map: &Vec<Vec<u32>>, row:usize, current_row:usize, column:usize) -> usize {
    let mut ldistance = 1;
    if current_row == 0 || row == 0 {
        return 0;
    } else if tree_map[current_row-1][column] < tree_map[row][column] 
    {
        ldistance = 1 + get_ldistance(tree_map,row,current_row-1,column);
    }
    return ldistance;
}

fn get_rdistance(tree_map: &Vec<Vec<u32>>, row:usize, current_row:usize, column:usize) -> usize {
    let mut rdistance = 1;
    if current_row == tree_map.len() -1 || row == tree_map.len() -1 {
        return 0;
    } else if tree_map[current_row+1][column] < tree_map[row][column] 
    {
        rdistance = 1 + get_rdistance(tree_map,row,current_row+1,column);
    }
    return rdistance;
}


fn get_ddistance(tree_map: &Vec<Vec<u32>>, row:usize, current_col:usize, column:usize) -> usize {
    let mut ddistance = 1;
    if current_col == tree_map[0].len() -1 || column == tree_map.len() -1 {
        return 0;
    } else if tree_map[row][current_col+1] < tree_map[row][column] 
    {
        ddistance = 1 + get_ddistance(tree_map,row,current_col+1,column);
    }
    return ddistance;
}

fn get_udistance(tree_map: &Vec<Vec<u32>>, row:usize, current_col:usize, column:usize) -> usize {
    let mut udistance = 1;
    if current_col == 0 || column == 0 {
        return 0;
    } else if tree_map[row][current_col-1] < tree_map[row][column] 
    {
        udistance = 1 + get_udistance(tree_map,row,current_col-1,column);
    }
    return udistance;
}