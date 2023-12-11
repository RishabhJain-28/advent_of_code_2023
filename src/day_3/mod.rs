use std::fs;

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_3.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect())
    }

    let mut i = 0;
    let mut res = 0;
    let mut start: i32 = -1;
    let mut current_num: i32 = -1;
    let mut add_to_sum = false;
    while i < grid.len() {
        let mut j = 0;

        while j < grid[i].len() {
            let c = grid[i][j];

            match c {
                '0'..='9' => {
                    if start == -1 {
                        start = j as i32;
                        current_num = grid[i][j].to_digit(10).unwrap() as i32;
                    } else {
                        current_num = current_num * 10 + grid[i][j].to_digit(10).unwrap() as i32;
                    }

                    if i > 0 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i - 1][j]);
                    }
                    if j > 0 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i][j - 1]);
                    }

                    if i < grid.len() - 1 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i + 1][j]);
                    }
                    if j < grid[i].len() - 1 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i][j + 1]);
                    }

                    if i > 0 && j > 0 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i - 1][j - 1]);
                    }
                    if i > 0 && j < grid[i].len() - 1 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i - 1][j + 1]);
                    }

                    if i < grid.len() - 1 && j > 0 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i + 1][j - 1]);
                    }
                    if i < grid.len() - 1 && j < grid[i].len() - 1 {
                        add_to_sum = add_to_sum || is_valid_symbol(grid[i + 1][j + 1]);
                    }
                }
                _ => {
                    if add_to_sum {
                        res += current_num;
                    }
                    add_to_sum = false;
                    start = -1;
                    current_num = -1;
                }
            }
            j += 1;
        }
        if add_to_sum {
            res += current_num;
        }
        add_to_sum = false;
        start = -1;
        current_num = -1;
        i += 1;
    }

    println!("res = {}", res);
}
fn is_valid_symbol(c: char) -> bool {
    match c {
        '0'..='9' | '.' => false,
        _ => true,
    }
}
