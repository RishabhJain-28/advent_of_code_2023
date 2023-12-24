use std::fs;

#[derive(Debug)]
struct Range {
    source_start: u64,
    dest_start: u64,
    range: u64,
}
#[derive(Debug, Clone)]
struct SeedRange {
    source_start: u64,
    range: u64,
}

#[allow(dead_code)]
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

pub fn solve_2() {
    let mut maps: Vec<Vec<Range>> = Vec::new();
    let input = fs::read_to_string("inputs/day_5.txt").expect("Couldn't read day_5.txt");
    let mut lines = input.lines();

    let (_, seeds_str) = lines.next().unwrap().split_once(": ").unwrap();
    let mut seeds_input = seeds_str
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
    let mut res = u64::MAX;

    for seed_range in seeds_input.chunks(2) {
        println!("{:?} {}", seed_range, seeds_input.len() / 2);
        for s in seed_range[0]..(seed_range[0] + seed_range[1]) {
            let mut seed = s;
            for map in maps.iter() {
                let mut new_val = seed;
                for range in map {
                    if seed >= range.source_start && seed < range.source_start + range.range {
                        new_val = range.dest_start + seed - range.source_start;
                        break;
                    }
                }
                seed = new_val;
            }
            res = res.min(seed);
        }
    }

    println!("{}", res);
}

#[allow(dead_code)]
pub fn _solve_2() {
    let mut maps: Vec<Vec<Range>> = Vec::new();
    let input = fs::read_to_string("inputs/day_5.txt").expect("Couldn't read day_5.txt");
    let mut lines = input.lines();

    let (_, seeds_str) = lines.next().unwrap().split_once(": ").unwrap();
    let seeds_vals = seeds_str
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut seed_ranges = Vec::new();
    let mut source_start = 0;
    let mut range = 0;

    for (i, seed) in seeds_vals.into_iter().enumerate() {
        if i % 2 == 0 {
            source_start = seed;
        } else {
            range = seed;
            seed_ranges.push(SeedRange {
                source_start,
                range,
            });
        }
    }

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
        let mut new_vals = Vec::from(seed_ranges.clone());
        for (index, seed) in (seed_ranges).iter_mut().enumerate() {
            for range in &map {
                println!("range {:?}", range);
                println!("seed range {:?}", seed);

                if seed.source_start + seed.range < range.source_start
                    || range.source_start + range.range < seed.source_start
                {
                    println!("no overlap");
                    continue;
                }

                let overlap_start = seed.source_start.max(range.source_start);
                let overlap_end =
                    (seed.source_start + seed.range).min(range.source_start + range.range);

                if overlap_start >= range.source_start
                    && overlap_end <= range.source_start + range.range
                {
                    new_vals[index] = SeedRange {
                        source_start: range.dest_start + overlap_start - range.source_start,
                        range: overlap_end - overlap_start,
                    };
                    println!("1 overlap");
                    println!("overlap start {}", overlap_start);
                    println!("overlap end {}", overlap_end);

                    println!("new seed {:?}", new_vals[index]);
                } else if overlap_start < range.source_start
                    && overlap_end > range.source_start + range.range
                {
                    new_vals.push(SeedRange {
                        source_start: seed.source_start,
                        range: overlap_start - seed.source_start,
                    });
                    new_vals.push(SeedRange {
                        source_start: overlap_end,
                        range: seed.source_start + seed.range - overlap_end,
                    });
                    new_vals[index] = SeedRange {
                        source_start: range.dest_start + overlap_start - range.source_start,

                        range: overlap_end - overlap_start,
                    };
                    println!("3 overlap");
                } else if overlap_start == range.source_start {
                    new_vals.push(SeedRange {
                        source_start: seed.source_start,
                        range: overlap_start - seed.source_start,
                    });
                    new_vals[index] = SeedRange {
                        source_start: range.dest_start + overlap_start - range.source_start,

                        range: overlap_end - overlap_start,
                    };
                    println!("2 start overlap");
                } else if overlap_start == range.source_start + range.range {
                    new_vals.push(SeedRange {
                        source_start: overlap_end,
                        range: seed.source_start + seed.range - overlap_end,
                    });
                    new_vals[index] = SeedRange {
                        source_start: range.dest_start + overlap_start - range.source_start,

                        range: overlap_end - overlap_start,
                    };
                    println!("2 end overlap");
                }
            }
            println!("## next seed range");
        }
        seed_ranges = new_vals.clone();
        println!("-------------- next map");
    }
    println!("{:?}", seed_ranges);

    let mut iter = seed_ranges.into_iter();
    let mut res = iter.next().unwrap().source_start;

    for r in iter {
        res = res.min(r.source_start);
    }
    println!("{}", res);
}
