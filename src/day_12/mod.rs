use std::{fs, str::FromStr};

#[derive(Debug)]
struct Record {
    pattern: String,
    list: Vec<usize>,
}

impl FromStr for Record {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, rest) = s.split_once(" ").unwrap();
        let pattern = String::from(pattern);
        let list = rest
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>();
        Ok(Record { list, pattern })
    }
}

pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_12.txt").unwrap();
    let res = input
        .lines()
        .map(|rec_str| rec_str.parse::<Record>().unwrap())
        .fold(0, |acc, record| {
            let score = get_score(&record.pattern.chars().collect::<Vec<_>>(), &record.list);
            score + acc
        });
    println!("res = {res}");
}

fn get_score(pat: &[char], list: &[usize]) -> usize {
    if list.is_empty() {
        return (!pat.contains(&'#')) as usize;
    }

    if pat.len() < list.iter().sum::<usize>() + list.len() - 1 {
        return 0;
    }

    match pat[0] {
        '#' => get_hash_score(pat, list),
        '.' => get_score(&pat[1..], list),
        '?' => get_score(&pat[1..], list) + get_hash_score(pat, list),
        _ => panic!("founud invalid char input: {}", pat[0]),
    }
}

fn get_hash_score(pat: &[char], list: &[usize]) -> usize {
    if pat.len() < list[0] || pat[0..list[0]].contains(&'.') {
        return 0;
    }
    if pat.len() == list[0] {
        return if list.len() == 1 { 1 } else { 0 };
    }
    if pat[list[0]] == '#' {
        return 0;
    }

    get_score(&pat[list[0] + 1..], &list[1..])
}
