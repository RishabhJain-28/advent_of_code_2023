use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Round,
    Square,
    Empty,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_14.txt").unwrap();
    let mut grid = Vec::<Vec<Tile>>::new();

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                'O' => Tile::Round,
                '.' => Tile::Empty,
                '#' => Tile::Square,
                _ => panic!("invalid char found"),
            })
            .collect();
        grid.push(row)
    }

    tilt(&mut grid, Direction::North);
    print_grid(&grid);
    let res = calculate_load(&grid);
    println!("res = {} ", res);
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_14.txt").unwrap();
    let mut grid = Vec::<Vec<Tile>>::new();

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                'O' => Tile::Round,
                '.' => Tile::Empty,
                '#' => Tile::Square,
                _ => panic!("invalid char found"),
            })
            .collect();
        grid.push(row)
    }
    let dirs = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    let mut map = HashMap::<Vec<Vec<Tile>>, usize>::new();
    let iter = 1000000000;
    let mut i = 0;
    let mut start = 0;
    let mut end = 0;
    'outer: while i < iter {
        i += 1;

        for d in dirs.iter().clone() {
            tilt(&mut grid, *d);
        }
        let key = grid
            .iter()
            .map(|r| r.clone().into_iter().collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>();

        if let Some(old) = map.insert(key, i) {
            start = old;
            end = i;
            break 'outer;
        }
    }

    let left = (iter - start) % (end - start);
    for _ in 0..left {
        for d in dirs {
            tilt(&mut grid, d)
        }
    }

    let res = calculate_load(&grid);
    println!("res = {} ", res);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Tile>>) {
    for r in grid {
        for c in r {
            let tile = match *c {
                Tile::Empty => '.',
                Tile::Round => 'O',
                Tile::Square => '#',
            };

            print!("{}", tile)
        }
        println!()
    }
    println!()
}

fn tilt(grid: &mut Vec<Vec<Tile>>, dir: Direction) {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let (dr, dc, row, col) = match dir {
        Direction::North => (
            -1,
            0,
            (0..h).collect::<Vec<i32>>(),
            (0..w).collect::<Vec<i32>>(),
        ),
        Direction::East => (
            0,
            1,
            (0..h).rev().collect::<Vec<i32>>(),
            (0..w).rev().collect::<Vec<i32>>(),
        ),
        Direction::West => (
            0,
            -1,
            (0..h).collect::<Vec<i32>>(),
            (0..w).collect::<Vec<i32>>(),
        ),
        Direction::South => (
            1,
            0,
            (0..h).rev().collect::<Vec<i32>>(),
            (0..w).rev().collect::<Vec<i32>>(),
        ),
    };

    for i in row {
        for j in col.clone() {
            match grid[i as usize][j as usize] {
                Tile::Round => {
                    let mut x = i;

                    let mut y = j;
                    while x + dr >= 0
                        && x + dr < h
                        && y + dc >= 0
                        && y + dc < w
                        && grid[(x + dr) as usize][(y + dc) as usize] == Tile::Empty
                    {
                        x += dr;
                        y += dc;
                    }

                    grid[i as usize][j as usize] = Tile::Empty;
                    grid[x as usize][y as usize] = Tile::Round;
                }
                _ => {}
            }
        }
    }
}

fn calculate_load(grid: &Vec<Vec<Tile>>) -> usize {
    let width = grid.first().unwrap().len();
    let mut res: usize = 0;

    for i in 0..grid.len() {
        for j in 0..width {
            match grid[i][j] {
                Tile::Round => {
                    res += grid.len() - i;
                }
                _ => {}
            }
        }
    }
    res
}

#[allow(dead_code)]
fn calculate_load_part1(grid: &Vec<Vec<Tile>>) -> usize {
    let width = grid.first().unwrap().len();
    let mut current_load = vec![grid.len(); width];
    let mut res: usize = 0;

    for i in 0..grid.len() {
        for j in 0..width {
            match grid[i][j] {
                Tile::Round => {
                    current_load[j] -= 1;
                    res += grid.len() - i;
                }
                Tile::Square => {
                    current_load[j] = grid.len() - i - 1;
                }
                Tile::Empty => {}
            }
        }
    }
    res
}
