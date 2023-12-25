use std::fs;

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_9.txt").unwrap();
    let mut histories: Vec<Vec<i64>> = vec![];
    for line in input.lines() {
        histories.push(
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }
    let mut res = 0;
    for history in histories {
        let mut sum = 0;
        let mut diff = history.clone();
        loop {
            sum += diff.last().unwrap_or(&0);
            diff = get_diff_vector(&diff);

            if diff.iter().all(|n| *n == 0) {
                break;
            }
        }
        res += sum;
    }

    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_9.txt").unwrap();
    let mut histories: Vec<Vec<i64>> = vec![];
    for line in input.lines() {
        histories.push(
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }
    // let mut res = 0;
    // for history in histories {
    //     let mut sum = *history.first().unwrap();
    //     let mut diff = history.into_iter().rev().collect::<Vec<_>>();
    //     loop {
    //         diff = get_diff_vector(&diff);
    //         sum = diff.first().unwrap_or(&0) - sum;

    //         if diff.iter().all(|n| *n == 0) {
    //             break;
    //         }
    //     }
    //     println!("{sum}");
    //     res += sum;
    // }
    let mut res = 0;
    for history in histories {
        let mut sum = 0;
        let mut diff = history.into_iter().rev().collect::<Vec<_>>();
        loop {
            sum += diff.last().unwrap_or(&0);
            diff = get_diff_vector(&diff);

            if diff.iter().all(|n| *n == 0) {
                break;
            }
        }
        res += sum;
    }
    println!("res = {res}");
}

fn get_diff_vector(nums: &[i64]) -> Vec<i64> {
    let mut res = Vec::new();
    let mut iter = nums.iter();
    let mut first = iter.next().unwrap();
    for n in iter {
        res.push(n - first);
        first = n;
    }
    res
}
