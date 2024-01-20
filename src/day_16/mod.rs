use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;
#[derive(Debug, PartialEq, Copy, Clone)]
enum Mirror {
    VerticalSplitter,
    HorizontalSplitter,
    AngledlCockwise,
    AngledAnitClockwise,
    Empty,
}

impl Mirror {
    fn reflect(&self, curr_dir: Direction) -> Vec<Direction> {
        return match (self, curr_dir) {
            (Mirror::VerticalSplitter, West | East) => vec![North, South],
            (Mirror::HorizontalSplitter, North | South) => vec![West, East],
            (Mirror::AngledlCockwise, East) => vec![North],
            (Mirror::AngledlCockwise, West) => vec![South],
            (Mirror::AngledlCockwise, North) => vec![East],
            (Mirror::AngledlCockwise, South) => vec![West],
            (Mirror::AngledAnitClockwise, East) => vec![South],
            (Mirror::AngledAnitClockwise, West) => vec![North],
            (Mirror::AngledAnitClockwise, North) => vec![West],
            (Mirror::AngledAnitClockwise, South) => vec![East],
            _ => vec![curr_dir],
        };

        //         let valid_dirs = match self {
        //             Mirror::VerticalSplitter => {
        //                 match curr_dir {

        //                 }
        //             },
        //             Mirror::HorizontalSplitter => vec![Direction::North,Direction::South],
        //             Mirror::AngledlCockwise => vec![
        //                 (Direction::South, Direction::East),
        //                 (Direction::North, Direction::West),
        //                 (Direction::East, Direction::South),
        //                 (Direction::West, Direction::Nort),
        //             ],
        //             // Mirror::AngledAnitClockwise => todo!(),
        //             Mirror::AngledAnitClockwise => vec![
        //                 (Direction::South, Direction::West),
        //                 (Direction::North, Direction::East),
        //             ],
        //             Mirror::Empty => todo!(),
        //         }
        // ;
        // return match (self, curr_dir) {
        //     (Self::VerticalSplitter) => ,
        //     _ => curr_dir,
        // };
    }
}

#[derive(Default, Clone)]
struct Grid {
    grid: HashMap<(i32, i32), Mirror>,
    visited: HashSet<(i32, i32, Direction)>,
    marked: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Direction {
    fn get_delta_vec(&self) -> (i32, i32) {
        match &self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from(format!("height: {}, width: {}\n", self.height, self.width));
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(mirror) = self.grid.get(&(i, j)) {
                    let c = match mirror {
                        Mirror::VerticalSplitter => "|",
                        Mirror::HorizontalSplitter => "-",
                        Mirror::AngledlCockwise => "/",
                        Mirror::AngledAnitClockwise => "\\",
                        Mirror::Empty => ".",
                    };
                    res += c;
                } else {
                    res += ".";
                }
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}

impl Grid {
    #[allow(dead_code)]
    fn print_marked(&self) {
        let mut res = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                if self.marked.contains(&(i, j)) {
                    res += "#";
                } else {
                    res += ".";
                }
            }
            res += "\n";
        }
        println!("{}", res)
    }

    fn mark_visited(&mut self, i: i32, j: i32, dir: Direction) -> bool {
        self.marked.insert((i, j));
        self.visited.insert((i, j, dir))
    }
}
#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_16.txt").unwrap();
    let mut grid = Grid::default();
    let mut height = 0;
    for (i, line) in input.lines().enumerate() {
        height += 1;
        for (j, c) in line.chars().enumerate() {
            let mirror = match c {
                '|' => Mirror::VerticalSplitter,
                '-' => Mirror::HorizontalSplitter,
                '\\' => Mirror::AngledAnitClockwise,
                '/' => Mirror::AngledlCockwise,
                '.' => Mirror::Empty,
                _ => {
                    panic!("invalid character");
                }
            };
            if mirror != Mirror::Empty {
                grid.grid.insert((i as i32, j as i32), mirror);
            }
        }
        grid.width = line.len() as i32;
    }
    grid.height = height;

    follow(&mut grid, Direction::East, 0, 0);

    println!("res = {}", grid.marked.len());
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_16.txt").unwrap();
    let mut grid = Grid::default();
    let mut height = 0;
    for (i, line) in input.lines().enumerate() {
        height += 1;
        for (j, c) in line.chars().enumerate() {
            let mirror = match c {
                '|' => Mirror::VerticalSplitter,
                '-' => Mirror::HorizontalSplitter,
                '\\' => Mirror::AngledAnitClockwise,
                '/' => Mirror::AngledlCockwise,
                '.' => Mirror::Empty,
                _ => {
                    panic!("invalid character");
                }
            };
            if mirror != Mirror::Empty {
                grid.grid.insert((i as i32, j as i32), mirror);
            }
        }
        grid.width = line.len() as i32;
    }
    grid.height = height;

    let mut res = 0;

    for i in 0..grid.height {
        let mut new_grid = grid.clone();
        follow(&mut new_grid, East, i, 0);
        let energised = new_grid.marked.len();
        res = res.max(energised)
    }
    for i in 0..grid.height {
        let mut new_grid = grid.clone();
        follow(&mut new_grid, West, i, grid.width);
        let energised = new_grid.marked.len();
        res = res.max(energised)
    }

    for i in 0..grid.width {
        let mut new_grid = grid.clone();
        follow(&mut new_grid, South, 0, i);
        let energised = new_grid.marked.len();
        res = res.max(energised)
    }
    for i in 0..grid.width {
        let mut new_grid = grid.clone();
        follow(&mut new_grid, North, grid.height, i);
        let energised = new_grid.marked.len();
        res = res.max(energised)
    }

    println!("res = {}", res);
}

fn follow(grid: &mut Grid, dir: Direction, x: i32, y: i32) {
    let (dr, dc) = dir.get_delta_vec();
    let mut i = x;
    let mut j = y;

    while i >= 0 && j >= 0 && i < grid.height && j < grid.width && !grid.grid.contains_key(&(i, j))
    {
        if !grid.mark_visited(i, j, dir) {
            return;
        };
        i += dr;
        j += dc;
    }

    if let Some(mirror) = grid.grid.get(&(i, j)).copied() {
        grid.mark_visited(i, j, dir);
        let reflected_dirs = mirror.reflect(dir);
        for dir in reflected_dirs {
            let (dr, dc) = dir.get_delta_vec();
            follow(grid, dir, i + dr, j + dc);
        }
    }
}
