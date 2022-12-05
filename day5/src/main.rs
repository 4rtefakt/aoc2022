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
                    column.push(row.chars().nth(i).unwrap());
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
                
            for i in 1..amount {
                let value = crate_stacks[(start-1) as usize].pop().unwrap();
                crate_stacks[(end-1) as usize].push(value);
            }
        }
    }

    
    println!("trying to print crate stacks:\n{:?}",crate_stacks);
}

fn part2() {
        println!("===== Part2 =====");
        let input = fs::read_to_string("input")
                            .expect("Should have been able to read the file");

}

fn main () {
    part1();
    part2();
}
