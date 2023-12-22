use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_4.txt").unwrap();
    let mut res = 0;
    for line in input.lines() {
        let (_, rest) = line.split_once(": ").unwrap();

        let (winning, all) = rest.split_once(" | ").unwrap();

        let win_set = parse_num_string_to_hash_set(winning);
        let my_set = parse_num_string_to_hash_set(all);
        let mut count = 0;

        for num in my_set {
            if win_set.contains(&num) {
                count += 1;
            }
        }
        // println!("{count} {}", 2.pow (count - 1));
        if count > 0 {
            res += u32::pow(2, count - 1);
        }
    }
    println!("sum = {res}");
}

fn parse_num_string_to_hash_set(input: &str) -> HashSet<u32> {
    let nums = input
        .split_whitespace()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    return nums;
}
