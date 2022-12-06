use std::collections::VecDeque;
use std::fs;


fn resolve(input:&str, characters:usize) {
    let mut i = 0;
    let mut q: VecDeque<char> = VecDeque::new();

    for ch in input.chars() {
        q.push_back(ch);
        i += 1;
        if q.len() > characters {
            q.pop_front();
        }
        let mut valid = true;
        for item in &q {
            valid = q.len() == characters && valid && q.iter().filter(|&n| *n == *item).count() < 2;
        }
        if valid { 
            println!("Solution is {}.", i);
            break;
        }
    }
}


fn part1() {
    println!("===== Part1 =====");
    let input = fs::read_to_string("input")
                        .expect("Should have been able to read the file");
    resolve(&input, 4);
}

fn part2() {
    println!("===== Part2 =====");
    let input = fs::read_to_string("input")
                        .expect("Should have been able to read the file");
    resolve(&input, 14);        
}

fn main () {
    part1();
    part2();
}
