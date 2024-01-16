use std::fs;

pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_15.txt").unwrap();
    let res = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|input_str| hash_algo(&input_str))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("res = {}", res);
}

fn hash_algo(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| (acc + (c as u32)) * 17 % 256)
}
