use std::{collections::HashMap, fs};

pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_10.txt").unwrap();

    let mut matrix: Vec<Vec<Tile>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile = Tile::from_char(c);
            row.push(tile);
        }
        matrix.push(row);
    }
    for (i, row) in matrix.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::Start {
                let mut vis = HashMap::new();
                let steps = traverse_loop(&matrix, i, j, &mut vis);
                println!("res = {}", steps / 2);
                break;
            }
        }
    }
}

fn traverse_loop(
    matrix: &Vec<Vec<Tile>>,
    start_i: usize,
    start_j: usize,
    vis: &mut HashMap<(usize, usize), bool>,
) -> usize {
    let mut steps = 0;
    let directions_vec = vec![
        (1, 0, Direction::South),
        (0, 1, Direction::East),
        (-1, 0, Direction::North),
        (0, -1, Direction::West),
    ];
    let n = matrix.len() as i32;

    let m = matrix[0].len() as i32;

    let mut i = start_i;
    let mut j = start_j;
    'outer: loop {
        vis.insert((i, j), true);

        for d in directions_vec.iter() {
            let x = if (i as i32) + d.0 >= 0 && (i as i32) + d.0 < n {
                (i as i32 + d.0) as usize
            } else {
                continue;
            };
            let y = if (j as i32) + d.1 >= 0 && (j as i32) + d.1 < m {
                (j as i32 + d.1) as usize
            } else {
                continue;
            };

            let possible =
                matrix[x][y].is_continous(&d.2, true) && matrix[i][j].is_continous(&d.2, false);
            if possible {
                if let Some(has_visited) = vis.get(&(x, y)) {
                    if *has_visited {
                        if start_i == x && start_j == y {
                            steps += 1;
                            return steps;
                        } else {
                            continue;
                        }
                    }
                } else {
                    i = x;
                    j = y;
                    steps += 1;
                    continue 'outer;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthAndEast,
            'J' => Self::NorthAndWest,
            '7' => Self::SouthAndWest,
            'F' => Self::SouthAndEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => {
                panic!("INVALID CHAR")
            }
        }
    }

    fn is_continous(&self, d: &Direction, incoming: bool) -> bool {
        let mut dir = *d;
        if incoming {
            dir = d.opposite();
        }
        match self {
            Tile::Horizontal => matches!(dir, Direction::East | Direction::West),
            Tile::Vertical => matches!(dir, Direction::North | Direction::South),
            Tile::NorthAndEast => matches!(dir, Direction::North | Direction::East),
            Tile::NorthAndWest => matches!(dir, Direction::North | Direction::West),
            Tile::SouthAndEast => matches!(dir, Direction::South | Direction::East),
            Tile::SouthAndWest => matches!(dir, Direction::South | Direction::West),
            Tile::Start => true,
            Tile::Ground => false,
        }
    }
}
