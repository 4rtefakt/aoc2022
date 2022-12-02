use std::fs;

fn main() {
    let input = fs::read_to_string("input1")
                .expect("Should have been able to read the file");

    let mut current_elf = 0;
    let mut max_elf = 0;
    let mut max_elf2 = 0;
    let mut max_elf3 = 0;

    for line in input.lines() {

        // If line is empty, new elf and skip to next line
        // Update top 3 elves with current elf if needed
        if line.is_empty() {
            if current_elf > max_elf {
                max_elf3 = max_elf2;
                max_elf2 = max_elf;
                max_elf = current_elf;
            } else if current_elf > max_elf2 {
                max_elf3 = max_elf2;
                max_elf2 = current_elf;
            } else if current_elf > max_elf3 {
                max_elf3 = current_elf;
            }
            println!("New Elf ! Last had {}.", current_elf);
            current_elf = 0;
            continue;
        }

        // If line isn't empty, add it to current_elf and update max_elf
        let cur_line = line.parse::<i32>().unwrap();
        println!("Adding {} to current elf because of line {}.", cur_line, line);
        current_elf += cur_line;
    }

    // Adding up top 3 elves and printing result
    let total = max_elf + max_elf2 + max_elf3;

    println!("Top 3 elves are carrying {}, with top elf carrying {}", total, max_elf);
}
