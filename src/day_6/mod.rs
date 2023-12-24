use std::{fs, iter::zip};

pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_6.txt").unwrap();
    let mut lines = input.lines();

    let time: Vec<f64> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let distance: Vec<f64> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut res = 1.0;
    for (t, d) in zip(time, distance) {
        let min_val = f64::floor((t - f64::sqrt(t * t - 4.0 * d)) / 2.0 + 1.0);
        let max_val = f64::ceil((t + f64::sqrt(t * t - 4.0 * d)) / 2.0 - 1.0);

        res *= max_val - min_val + 1.0;
    }
    println!("res = {}", res);
}
