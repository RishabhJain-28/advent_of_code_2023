use std::fs;

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("./inputs/day_1.txt").unwrap();
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

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("./inputs/day_1.txt").unwrap();
    let mut res = 0;
    let patterns = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    for line in input.lines() {
        let a = get_matched(line, &patterns).unwrap();
        let b = get_matched_last(line, &patterns).unwrap();
        res += a * 10 + b;
        println!("{res}")
    }
}

fn get_matched<'a>(line: &str, patterns: &[&'a str]) -> Option<i32> {
    for i in 0..line.len() {
        for (index, p) in patterns.iter().enumerate() {
            if p.len() + i > line.len() {
                continue;
            }

            if &line[i..(i + p.len())] == *p {
                return Some((index as i32) / 2 + 1);
            }
        }
    }
    None
}

fn get_matched_last<'a>(line: &str, patterns: &[&'a str]) -> Option<i32> {
    for i in (0..line.len()).rev() {
        for (index, p) in patterns.iter().enumerate() {
            if p.len() + i > line.len() {
                continue;
            }

            if &line[i..(i + p.len())] == *p {
                return Some((index as i32) / 2 + 1);
            }
        }
    }
    None
}
