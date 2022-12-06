use std::fs;

fn part1() {
    println!("===== Part1 =====");
    let input = fs::read_to_string("input")
                        .expect("Should have been able to read the file");
    let mut i = 1;
    let mut a:char = '0';
    let mut b:char = '0';
    let mut c:char = '0';
    for ch in input.chars() {
        if a != ch && b != ch && c != ch && i > 3 && a != '0' && a != b && a != c && b != c {
            println!("Need to process {} characters.", i);
            break;
        }
        a = b;
        b = c;
        c = ch;
        i += 1;
    }

}

fn part2() {
        println!("===== Part2 =====");
        let _input = fs::read_to_string("input")
                            .expect("Should have been able to read the file");

}

fn main () {
    part1();
    part2();
}
