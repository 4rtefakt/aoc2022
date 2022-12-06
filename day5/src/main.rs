use std::fs;
use regex::Regex;

fn part1() {
    println!("===== Part1 =====");
    let input = fs::read_to_string("input")
                        .expect("Should have been able to read the file");


    let re = Regex::new(r"move ([0-9]*) from ([0-9]) to ([0-9])").unwrap();
    let mut crate_stacks:Vec<Vec<char>> = Vec::new();
    let mut part1_strings:Vec<String> = Vec::new();

    for line in input.lines() {
        if line.is_empty() { 
            part1_strings.reverse();
            for i in 0..9 {
                let mut column:Vec<char> = Vec::new();
                for row in &part1_strings {
                    if row.chars().nth(i).unwrap() != ' ' {
                        column.push(row.chars().nth(i).unwrap());
                    }
                }
                crate_stacks.push(column);
            }
            continue; 
        } // It's a useless line
        else if line.starts_with("[") {     // It's a line from part1
            let mut crates:String = String::new();
            for i in 0..9 {
                crates.push(line.chars().nth(1+i*4).unwrap());
            }
            part1_strings.push(crates);
        }
        else if line.starts_with("move") { //It's a line from part2

            let caps = re.captures(line).unwrap();
            let amount:i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let start:i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let end:i32 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            
            for _i in 0..amount {
                let value = crate_stacks[(start-1) as usize].pop().unwrap();
                crate_stacks[(end-1) as usize].push(value);
            }
        }
    }

    let mut output:String = String::new();
    for mut stack in crate_stacks {
        output.push(stack.pop().unwrap());
    }

    
    println!("The top of each stack at the very end is : {}.", output);
}

fn part2() {
        println!("===== Part2 =====");
        let input = fs::read_to_string("input")
                            .expect("Should have been able to read the file");

    let re = Regex::new(r"move ([0-9]*) from ([0-9]) to ([0-9])").unwrap();
    let mut crate_stacks:Vec<Vec<char>> = Vec::new();
    let mut part1_strings:Vec<String> = Vec::new();

    for line in input.lines() {
        if line.is_empty() { 
            part1_strings.reverse();
            for i in 0..9 {
                let mut column:Vec<char> = Vec::new();
                for row in &part1_strings {
                    if row.chars().nth(i).unwrap() != ' ' {
                        column.push(row.chars().nth(i).unwrap());
                    }
                }
                crate_stacks.push(column);
            }
            continue; 
        } // It's a useless line
        else if line.starts_with("[") {     // It's a line from part1
            let mut crates:String = String::new();
            for i in 0..9 {
                crates.push(line.chars().nth(1+i*4).unwrap());
            }
            part1_strings.push(crates);
        }
        else if line.starts_with("move") { //It's a line from part2

            let caps = re.captures(line).unwrap();
            let amount:i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let start:i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let end:i32 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            
            let mut tmp_stack:Vec<char> = Vec::new();

            for _i in 0..amount {
                let value = crate_stacks[(start-1) as usize].pop().unwrap();
                tmp_stack.push(value);
            }
            tmp_stack.reverse();
            for item in tmp_stack {
                crate_stacks[(end-1) as usize].push(item);
            }
        }
    }

    let mut output:String = String::new();
    for mut stack in crate_stacks {
        output.push(stack.pop().unwrap());
    }

    
    println!("The top of each stack at the very end is : {}.", output);
}

fn main () {
    part1();
    part2();
}
