use std::fs;
#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_13.txt").unwrap();
    let mut res = 0;
    let mut pattern = Vec::new();
    let mut width: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            res += get_score(&pattern, width);
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
        res += get_score(&pattern, width);
    }

    println!("res = {res}");
}

fn get_score(pattern: &Vec<usize>, width: usize) -> usize {
    if let Some(score) = get_horizontal_score(&pattern) {
        return score * 100;
    } else if let Some(score) = get_vertical_score(&pattern, width) {
        return score;
    } else {
        panic!("Could not find a score");
    }
}

fn get_horizontal_score(pattern: &Vec<usize>) -> Option<usize> {
    let length = pattern.len();
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
        return Some(i);
    }

    None
}

fn get_vertical_score(pattern: &Vec<usize>, width: usize) -> Option<usize> {
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
        return Some(width - i - 1);
    }
    None
}
