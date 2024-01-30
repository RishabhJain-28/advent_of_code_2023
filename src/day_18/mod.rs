use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_vec(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::North,
            "R" => Self::East,
            "L" => Self::West,
            "D" => Self::South,
            _ => {
                panic!("Invalid char found")
            }
        }
    }
}
impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            3 => Self::North,
            0 => Self::East,
            2 => Self::West,
            1 => Self::South,
            _ => {
                panic!("Invalid char found")
            }
        }
    }
}

struct PlanItem {
    dir: Direction,
    steps: u32,
}
#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_18.txt").unwrap();

    let plan = input
        .lines()
        .map(|line| {
            let (dir_char, rest) = line.split_once(" ").unwrap();
            let (steps_char, _) = rest.split_once(" ").unwrap();
            let dir = Direction::from(dir_char);
            let steps = steps_char.parse::<u32>().unwrap();
            return PlanItem { dir, steps };
        })
        .collect::<Vec<_>>();
    let mut grid = HashSet::new();
    let mut max_h = 0;
    let mut min_h = 0;

    let mut max_v = 0;
    let mut min_v = 0;

    let mut v = 0;
    let mut h = 0;

    let mut x = 0;
    let mut y = 0;

    for item in plan {
        match item.dir {
            Direction::North => v -= item.steps as isize,
            Direction::South => v += item.steps as isize,
            Direction::East => h += item.steps as isize,
            Direction::West => h -= item.steps as isize,
        }

        max_h = max_h.max(h);
        min_h = min_h.min(h);
        min_v = min_v.min(v);
        max_v = max_v.max(v);

        let (dr, dc) = item.dir.get_vec();

        for _ in 0..item.steps {
            x = x + dr;
            y = y + dc;
            grid.insert((x, y));
        }
    }

    let v = (min_v, max_v);
    let h = (min_h, max_h);

    let mut start = (1, 1);
    let mut seen_in_v = HashSet::new();
    'outer: for i in v.0..(v.1 + 1) {
        for j in h.0..(h.1 + 1) {
            if !grid.contains(&(i, j)) {
                if seen_in_v.contains(&j) {
                    start = (i, j);
                    break 'outer;
                }
            } else {
                seen_in_v.insert(j);
            }
        }
    }
    flood_fill(&mut grid, start, v, h);
    let res = grid.len();
    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_18.txt").unwrap();
    let plan = input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(" ").unwrap();
            let (_, rest) = rest.split_once(" ").unwrap();
            let mut chars = rest.chars();
            chars.next(); // remove (
            chars.next(); // remove #
            chars.next_back(); // remove )
            let last = chars.next_back().unwrap();
            let dir = Direction::from(last.to_digit(10).unwrap() as usize);
            let step_str = &chars.collect::<String>();
            let steps = u32::from_str_radix(step_str, 16).unwrap();
            return PlanItem { dir, steps };
        })
        .collect::<Vec<_>>();

    let mut x = 0i64;
    let mut y = 0i64;
    let mut points = vec![];
    let perimeter = plan.iter().map(|item| item.steps as i64).sum::<i64>();

    for item in plan {
        let (dr, dc) = item.dir.get_vec();
        x = x + (dr as i64 * item.steps as i64);
        y = y + (dc as i64 * item.steps as i64);
        points.push((x, y));
    }
    points.push(points.first().unwrap().clone());
    //shoelace algo
    let area = points
        .windows(2)
        .map(|window| {
            let b = window.first().unwrap();
            let a = window.last().unwrap();
            let term = (a.0 * b.1) - (a.1 * b.0);
            term
        })
        .sum::<i64>();

    println!("res = {}", (area / 2) + (perimeter / 2) + 1);
}

fn flood_fill(
    grid: &mut HashSet<(isize, isize)>,
    start: (isize, isize),
    v: (isize, isize),
    h: (isize, isize),
) {
    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let mut open = BinaryHeap::new();
    open.push(start);

    while !open.is_empty() {
        let (x, y) = open.pop().unwrap();
        for dir in &dirs {
            let (dr, dc) = dir.get_vec();
            let i = x + dr;
            let j = y + dc;
            if i >= v.0 && i <= v.1 && j >= h.0 && j <= h.1 && !grid.contains(&(i, j)) {
                grid.insert((i, j));
                open.push((i, j));
            }
        }
    }
}

#[allow(dead_code)]
fn print_grid(
    grid: &HashSet<(isize, isize)>,
    v: (isize, isize),
    h: (isize, isize),
    start: (isize, isize),
) {
    for i in v.0..(v.1 + 1) {
        for j in h.0..(h.1 + 1) {
            let dug = grid.contains(&(i as isize, j as isize));
            let c = if (i, j) == start {
                'S'
            } else if dug {
                '#'
            } else {
                '.'
            };
            print!("{c}");
        }
        println!();
    }
}
