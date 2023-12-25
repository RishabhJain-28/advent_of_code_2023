use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_8.txt").unwrap();
    let mut lines = input.lines();
    let directions: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'R' { 1 } else { 0 })
        .collect();

    let mut map = HashMap::new();
    lines.next();

    for line in lines {
        let (source, mut rest) = line.split_once(" = ").unwrap();
        rest = rest.trim_start_matches('(');
        rest = rest.trim_end_matches(')');
        let (left, right) = rest.split_once(", ").unwrap();
        map.insert(source, (left, right));
    }
    let mut res = 0;
    let mut direction_index = 0;
    let mut source = "AAA";
    let dest = "ZZZ";

    loop {
        res += 1;
        let dir = directions[direction_index];
        let map_val = map.get(source).unwrap();
        source = if dir == 0 { map_val.0 } else { map_val.1 };
        if source == dest {
            break;
        }
        direction_index += 1;
        direction_index %= directions.len();
    }

    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_8.txt").unwrap();
    let mut lines = input.lines();
    let directions: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'R' { 1 } else { 0 })
        .collect();

    let mut map = HashMap::new();
    lines.next();
    let mut sources = vec![];
    for line in lines {
        let (source, mut rest) = line.split_once(" = ").unwrap();
        if source.chars().last().unwrap() == 'A' {
            sources.push(source);
        }
        rest = rest.trim_start_matches('(');
        rest = rest.trim_end_matches(')');
        let (left, right) = rest.split_once(", ").unwrap();
        map.insert(source, (left, right));
    }

    let mut path_counts = vec![];
    for s in sources {
        let mut source = s;
        let mut count = 0;
        let mut direction_index = 0;
        loop {
            count += 1;
            let dir = directions[direction_index];
            let map_val = map.get(source).unwrap();
            source = if dir == 0 { map_val.0 } else { map_val.1 };
            if source.chars().last().unwrap() == 'Z' {
                break;
            }
            direction_index += 1;
            direction_index %= directions.len();
        }
        path_counts.push(count);
    }
    let res = path_counts.iter().fold(1, |acc, x| lcm(acc, *x));
    println!("res = {res}");
}

fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
