use std::fs;

#[derive(Debug)]
enum Tile {
    Round,
    Square,
    Empty,
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
    let width = grid.first().unwrap().len();
    let mut current_load: Vec<i32> = vec![grid.len() as i32; width];
    let mut res = 0;

    for i in 0..grid.len() {
        for j in 0..width {
            match grid[i][j] {
                Tile::Round => {
                    res += current_load[j];
                    current_load[j] -= 1;
                }
                Tile::Square => {
                    current_load[j] = (grid.len() - i - 1) as i32;
                }
                Tile::Empty => {}
            }
        }
    }

    println!("res = {} ", res);
}
