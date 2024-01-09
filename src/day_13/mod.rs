use std::fs;

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_13.txt").unwrap();
    let mut res = 0;
    let mut pattern = Vec::new();
    let mut width: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            res += get_scores(&pattern, width).first().unwrap_or(&0);

            pattern = Vec::new();
        } else {
            width = line.len();
            let num = line
                .chars()
                .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 });
            pattern.push(num);
        }
    }
    if !pattern.is_empty() {
        res += get_scores(&pattern, width).first().unwrap_or(&0);
    }

    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_13.txt").unwrap();
    let mut res = 0;
    let mut pattern = Vec::new();
    let mut width: usize = 0;
    let mut patterns = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            res += find_smudge_and_get_score(&pattern, width);
            patterns.push(pattern);
            pattern = Vec::new();
        } else {
            width = line.len();
            let num = line
                .chars()
                .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 });
            pattern.push(num);
        }
    }
    if !pattern.is_empty() {
        res += find_smudge_and_get_score(&pattern, width);
    }

    println!("res = {res}");
}
fn find_smudge_and_get_score(pattern: &Vec<usize>, width: usize) -> usize {
    let score = *get_scores(&pattern, width).first().unwrap_or(&0);
    let mut pat = pattern.clone();
    for i in 0..pattern.len() {
        for j in 0..width {
            let mask = 1 << j;
            pat[i] ^= mask;

            let scores = get_scores(&pat, width);
            for new_score in scores {
                if new_score != 0 && new_score != score {
                    return new_score;
                }
            }

            pat[i] ^= mask;
        }
    }
    println!("Cant find a smudje");
    0
}
fn get_scores(pattern: &Vec<usize>, width: usize) -> Vec<usize> {
    get_horizontal_score(&pattern)
        .iter()
        .copied()
        .chain(get_vertical_score(&pattern, width).iter().copied())
        .collect::<Vec<usize>>()
}

fn get_horizontal_score(pattern: &Vec<usize>) -> Vec<usize> {
    let length = pattern.len();
    let mut scores = vec![];
    'outer: for i in 1..length {
        let mut j = i - 1;
        let mut k = i;
        loop {
            if pattern[j] != pattern[k] {
                continue 'outer;
            }
            k += 1;
            if j == 0 || k >= length {
                break;
            }

            j -= 1;
        }
        scores.push(i * 100)
    }
    scores
}

fn get_vertical_score(pattern: &Vec<usize>, width: usize) -> Vec<usize> {
    let mut scores = vec![];
    let mut pat = pattern.clone();
    let mut reflection = vec![0usize; pattern.len()];
    'outer: for i in 0..width - 1 {
        for (p, r) in pat.iter_mut().zip(reflection.iter_mut()) {
            *r = *r << 1 | (*p & 1);
            *p >>= 1;
        }
        let reflected = (1 << (width - i - 1)) - 1;
        let mask = (1 << (i + 1)) - 1;
        for j in 0..pattern.len() {
            if pat[j] & mask != reflection[j] & reflected {
                continue 'outer;
            }
        }
        scores.push(width - i - 1);
    }
    scores
}
