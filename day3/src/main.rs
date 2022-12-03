use std::fs;


fn part1() {
        println!("===== Part1 =====");
        let input = fs::read_to_string("input")
                            .expect("Should have been able to read the file");
        let mut score = 0;

        for line in input.lines() {
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

        let mut score = 0;
        let mut line1 = "";
        let mut line2 = "";
        let mut line3;

        for line in input.lines() {

            if line1.is_empty() {
                line1 = line;
            } else if line2.is_empty() {
                line2 = line;
            } else {
                line3 = line;

                let array1:Vec<char> = line1.chars().collect();
                let array2:Vec<char> = line2.chars().collect();

                for char in line3.chars() {
                    let item = char;
                    if array1.contains(&item) && array2.contains(&item) {
                        let rank = item as u32;
                        if rank > 96 {
                            score += rank-96;
                        } else {
                            score += rank-64+26;
                        }
                        line1 = "";
                        line2 = "";
                        break;
                    }
                }
            }
        }
        println!("Final score is {}.", score);
}

fn main () {
    part1();
    part2();
}
