use std::fs;

pub fn solve_1() {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;

    let input = fs::read_to_string("inputs/day_2").unwrap();
    let mut res = 0;
    'outer: for line in input.lines() {
        let game = parse_input(line);
        // println!("{game:?}");
        for set in game.sets {
            for ball in set {
                match ball {
                    BallSet::blue(c) => {
                        if c > BLUE_LIMIT {
                            continue 'outer;
                        }
                    }
                    BallSet::red(c) => {
                        if c > RED_LIMIT {
                            continue 'outer;
                        }
                    }
                    BallSet::green(c) => {
                        if c > GREEN_LIMIT {
                            continue 'outer;
                        }
                    }
                }
            }
        }
        res += game.id;
    }

    println!("{res}");
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<Vec<BallSet>>,
}

#[derive(Debug)]
enum BallSet {
    red(u32),
    blue(u32),
    green(u32),
}

pub fn parse_input(line: &str) -> Game {
    let (pre, rest) = line.split_once(':').unwrap();
    let (_, id_string) = pre.trim().split_once(" ").unwrap();
    let id = id_string.parse::<u32>().unwrap();
    let mut sets = Vec::new();

    for s in rest.split(';') {
        let mut counts = Vec::new();
        for ball_count in s.split(',') {
            let (count_str, color) = ball_count.trim().split_once(' ').unwrap();
            let count = count_str.parse::<u32>().unwrap();
            let set_item = match color {
                "red" => BallSet::red(count),
                "blue" => BallSet::blue(count),
                "green" => BallSet::green(count),
                _ => {
                    panic!("wrong input => mismatch color value")
                }
            };
            counts.push(set_item);
        }

        sets.push(counts)
    }

    return Game { id, sets };
}
