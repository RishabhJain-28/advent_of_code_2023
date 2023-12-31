use std::{collections::HashMap, fs, str::FromStr};

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
#[allow(dead_code)]
pub fn solve_1() {
    let mut dp = HashMap::new();

    let input = fs::read_to_string("inputs/day_12.txt").unwrap();
    let res = input
        .lines()
        .map(|rec_str| rec_str.parse::<Record>().unwrap())
        .fold(0, |acc, record| {
            let score = get_score(
                &record.pattern.chars().collect::<Vec<_>>(),
                &record.list,
                &mut dp,
            );
            score + acc
        });
    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_12.txt").unwrap();
    let mut dp = HashMap::new();
    let res = input
        .lines()
        .map(|rec_str| rec_str.parse::<Record>().unwrap())
        .fold(0, |acc, record| {
            let mut pat = Vec::new();
            let mut list = Vec::new();
            let pat_vec = record.pattern.chars().collect::<Vec<_>>();
            for _ in 0..4 {
                pat.extend(pat_vec.iter().chain([&'?']));
            }
            pat.extend(pat_vec.iter());

            for _ in 0..5 {
                list.extend(&record.list);
            }

            let score = get_score(&pat, &list, &mut dp);
            score + acc
        });
    println!("res = {res}");
}

fn get_score(
    pat: &[char],
    list: &[usize],
    dp: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if let Some(value) = dp.get(&(pat.to_vec(), list.to_vec())) {
        return *value;
    }

    if list.is_empty() {
        return (!pat.contains(&'#')) as usize;
    }

    if pat.len() < list.iter().sum::<usize>() + list.len() - 1 {
        return 0;
    }

    let score = match pat[0] {
        '#' => get_hash_score(pat, list, dp),
        '.' => get_score(&pat[1..], list, dp),
        '?' => get_score(&pat[1..], list, dp) + get_hash_score(pat, list, dp),
        _ => panic!("founud invalid char input: {}", pat[0]),
    };

    dp.insert((pat.to_vec(), list.to_vec()), score);

    score
}

fn get_hash_score(
    pat: &[char],
    list: &[usize],
    dp: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    if let Some(value) = dp.get(&(pat.to_vec(), list.to_vec())) {
        return *value;
    }

    if pat.len() < list[0] || pat[0..list[0]].contains(&'.') {
        return 0;
    }
    if pat.len() == list[0] {
        return if list.len() == 1 { 1 } else { 0 };
    }
    if pat[list[0]] == '#' {
        return 0;
    }

    get_score(&pat[list[0] + 1..], &list[1..], dp)
}
