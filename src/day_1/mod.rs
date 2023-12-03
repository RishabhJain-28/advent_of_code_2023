use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/day_1").unwrap();
    let mut res = 0;
    for line in input.lines() {
        let rest = line.trim_start_matches(char::is_alphabetic);
        let rest = rest.trim_end_matches(char::is_alphabetic);
        let a = rest.chars().nth(0).unwrap().to_digit(10).unwrap();
        let b = rest.chars().last().unwrap().to_digit(10).unwrap();
        res = res + a * 10 + b;
        println!("{res}")
    }
}
