use std::fs;

#[derive(Debug)]
struct Range {
    source_start: u64,
    dest_start: u64,
    range: u64,
}

pub fn solve_1() {
    let mut maps: Vec<Vec<Range>> = Vec::new();
    let input = fs::read_to_string("inputs/day_5.txt").expect("Couldn't read day_5.txt");
    let mut lines = input.lines();

    let (_, seeds_str) = lines.next().unwrap().split_once(": ").unwrap();
    let mut seeds = seeds_str
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        if line.contains(":") {
            maps.push(Vec::new());
            continue;
        }
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        maps.last_mut().unwrap().push(Range {
            dest_start: nums[0],
            source_start: nums[1],
            range: nums[2],
        });
    }

    for map in maps {
        let mut new_vals = Vec::from(seeds.clone());
        for (index, seed) in (seeds).iter().enumerate() {
            for range in &map {
                if *seed >= range.source_start && *seed < range.source_start + range.range {
                    new_vals[index] = range.dest_start + *seed - range.source_start;
                }
            }
        }
        seeds = new_vals;
    }

    println!("{}", seeds.iter().min().unwrap());
}
