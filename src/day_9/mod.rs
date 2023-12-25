use std::fs;

pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_9.txt").unwrap();
    let mut histories: Vec<Vec<i32>> = vec![];
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
        'outer: loop {
            sum += diff.last().unwrap_or(&0);
            diff = get_diff_vector(&diff);

            for n in diff.iter() {
                if *n != 0 {
                    continue 'outer;
                }
            }
            break;
        }
        res += sum;
    }

    println!("res = {res}");
}

fn get_diff_vector(nums: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut iter = nums.iter();
    let mut first = iter.next().unwrap();
    for n in iter {
        res.push(n - first);
        first = n;
    }
    res
}
