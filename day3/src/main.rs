use std::fs;


fn part1() {
        println!("===== Part1 =====");
        let input = fs::read_to_string("input")
                            .expect("Should have been able to read the file");
        let mut score = 0;

        for line in input.lines() {
            let length = line.len();
            let (comp1, comp2) = line.split_at(line.len()/2);
            let array1:Vec<char> = comp1.chars().collect();

            for char in comp2.chars() {
                let item = char;
                if array1.contains(&item) {
                    let rank = item as u32;
                    if rank > 96 {
                        score += rank-96;
                    } else {
                        score += rank-64+26;
                    }
                    break;
                }
            }
        }
        println!("Final score is {}.", score);
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
