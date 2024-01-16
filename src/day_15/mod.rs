use std::fs;

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_15.txt").unwrap();
    let res = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|input_str| hash_algo(&input_str) as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("res = {}", res);
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_15.txt").unwrap();

    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    for line in input.lines() {
        for instruction in line.split(",") {
            if let Some((label, _)) = instruction.split_once("-") {
                let hash = hash_algo(label);
                if let Some(pos) = boxes[hash].iter().position(|(lbl, _)| lbl == label) {
                    boxes[hash].remove(pos);
                }
            } else if let Some((label, lens)) = instruction.split_once("=") {
                let hash = hash_algo(label);
                let lens = lens.parse::<u32>().unwrap();
                if let Some(pos) = boxes[hash].iter().position(|(lbl, _)| lbl == label) {
                    boxes[hash][pos].1 = lens
                } else {
                    boxes[hash].push((String::from(label), lens));
                }
            } else {
                panic!("invalid instruction")
            }
        }
    }

    let res = boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(j, (_, lens))| {
                    let val = (j as u32 + 1) * (*lens) * (i as u32 + 1);
                    val
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("res = {}", res);
}

fn hash_algo(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
}
