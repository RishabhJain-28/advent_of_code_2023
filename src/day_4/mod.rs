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

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_4.txt").unwrap();
    let mut card_map = HashMap::<u32, u32>::new();
    let mut res = 0;
    for line in input.lines() {
        let (card_str, rest) = line.split_once(": ").unwrap();
        let card_num = card_str
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let (winning, all) = rest.split_once(" | ").unwrap();

        let win_set = parse_num_string_to_hash_set(winning);
        let my_set = parse_num_string_to_hash_set(all);

        let count = 1 + match card_map.get(&card_num) {
            Some(c) => *c,
            None => 0,
        };

        card_map.insert(card_num, count);

        let mut num_matches = 0;
        for num in my_set {
            if win_set.contains(&num) {
                num_matches += 1;
                let new_count = 1 * count
                    + match card_map.get(&(num_matches + card_num)) {
                        Some(c) => *c,
                        None => 0,
                    };
                card_map.insert(num_matches + card_num, new_count);
            }
        }

        res += count;
    }

    println!("res = {res}");
}

fn parse_num_string_to_hash_set(input: &str) -> HashSet<u32> {
    let nums = input
        .split_whitespace()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    return nums;
}
