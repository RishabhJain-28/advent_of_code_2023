use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs, thread, time,
};

const MAX_CONSECUTIVE_STEPS: usize = 3;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
    Empty,
}

impl Direction {
    fn get_delta_vec(&self) -> (i32, i32) {
        match &self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
            Self::Empty => {
                panic!("No directtion vector on empty");
            }
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Empty => Direction::Empty,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Hash, Clone, Ord, Copy, Debug)]
struct Node {
    f: i32,
    h: i32,
    g: i32,
    x: i32,
    y: i32,
    dir: Direction,
    steps: usize,
}
impl Node {
    fn new(g: i32, h: i32, x: i32, y: i32, dir: Direction, steps: usize) -> Self {
        Node {
            f: g + h,
            g,
            h,
            x,
            y,
            dir,
            steps,
        }
    }
    fn score(&self) -> i32 {
        return self.g + self.h;
    }
}

pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_17.txt").unwrap();
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let res = find_route(&grid);
    println!("res = {:?}", res);
}

fn find_route(grid: &Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut vis = HashMap::new();
    let mut open = BinaryHeap::new();
    let mut close = HashSet::new();
    let mut parent = HashMap::new();

    open.push(Reverse(Node::new(
        0,
        0,
        0,
        0,
        Direction::Empty,
        MAX_CONSECUTIVE_STEPS,
    )));

    let dirs = vec![
        Direction::West,
        Direction::North,
        Direction::East,
        Direction::South,
    ];
    let mut ld = Direction::Empty;
    let mut ls = 0;
    let final_x = height - 1;
    let final_y = width - 1;
    'outer: loop {
        let node = match open.pop() {
            Some(val) => val.0,
            None => {
                println!("open list finished before reaching end");
                break;
            }
        };
        // println!("node : {:?}", node);
        res = res + 1;
        vis.insert((node.x, node.y), node.dir);
        close.insert(node.clone());

        for dir in &dirs {
            let steps = if *dir == node.dir {
                node.steps - 1
            } else {
                MAX_CONSECUTIVE_STEPS
            };
            if node.dir.opposite() == *dir {
                continue;
            }
            let (dr, dc) = dir.get_delta_vec();
            let x = dr + node.x;
            let y = dc + node.y;
            if x >= 0 && x < height && y >= 0 && y < width && steps > 0 {
                if x == final_x && y == final_y {
                    // println!("found end");
                    ld = *dir;
                    ls = steps;
                    parent.insert((x, y, *dir, steps), node.clone());
                    break 'outer;
                }
                let g = grid[x as usize][y as usize] as i32 + node.g;
                let h = i32::abs(final_x - x) + i32::abs(final_y - y);
                let new_node = Node::new(g, h, x, y, *dir, steps);
                if open.iter().any(|n| {
                    n.0.x == x
                        && n.0.y == y
                        && (n.0.dir == *dir || n.0.dir == Direction::Empty)
                        && n.0.steps == steps
                        && n.0.score() <= new_node.score()
                }) {
                    continue;
                }
                if close.iter().any(|n| {
                    n.x == x
                        && n.y == y
                        && (n.dir == *dir || n.dir == Direction::Empty)
                        && n.steps == steps
                        && n.score() <= new_node.score()
                }) {
                    continue;
                }
                open.push(Reverse(new_node));
                parent.insert((x, y, *dir, steps), node);
            }
        }
    }

    // println!("peeked nodes = {} ", res);
    let mut res = 0;
    let mut vis = HashMap::new();
    let mut x = height - 1;
    let mut y = width - 1;
    let mut dir = ld;
    let mut steps = ls;
    loop {
        if x == 0 && y == 0 {
            break;
        }
        let node = match parent.get(&(x, y, dir, steps)) {
            Some(node) => node,
            None => {
                panic!("no parent node found")
            }
        };
        res += grid[x as usize][y as usize];
        vis.insert((node.x, node.y), node.dir);
        x = node.x;
        y = node.y;
        dir = node.dir;
        steps = node.steps;
    }

    // print_path(&grid, &mut vis);
    return res;
}

#[allow(dead_code)]
fn print_path(grid: &Vec<Vec<usize>>, visited_with_dir: &mut HashMap<(i32, i32), Direction>) {
    let height = grid.len();
    let width = grid[0].len();

    for i in 0..height {
        for j in 0..width {
            if let Some(d) = visited_with_dir.get(&(i as i32, j as i32)) {
                let c = match d {
                    Direction::East => '>',
                    Direction::West => '>',
                    Direction::North => '^',
                    Direction::South => 'v',
                    Direction::Empty => 'E',
                };
                print!("{c}");
            } else {
                print!("{}", grid[i][j])
            }
        }
        println!();
    }

    println!();
    let ten_millis = time::Duration::from_millis(30);

    thread::sleep(ten_millis);
    clearscreen::clear().unwrap();
}
