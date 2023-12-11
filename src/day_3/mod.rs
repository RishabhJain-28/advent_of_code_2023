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
#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_3.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect())
    }

    let mut i = 0;
    let mut res = 0;
    let upper_lower: [[i32; 2]; 3] = [[-1, -1], [-1, 0], [-1, 1]];
    let same_row: [[i32; 2]; 2] = [[0, 1], [0, -1]];
    let lower_row: [[i32; 2]; 3] = [[1, 0], [1, 1], [1, -1]];
    while i < grid.len() {
        let mut j = 0;

        while j < grid[i].len() {
            let c = grid[i][j];

            match c {
                '*' => {
                    let mut nums: Vec<i32> = Vec::new();

                    for d in upper_lower {
                        let x: i32 = i as i32 + d[0];
                        let y: i32 = j as i32 + d[1];

                        if x >= 0
                            && y >= 0
                            && (x as usize) < grid.len()
                            && (y as usize) < grid[i].len()
                        {
                            let c = grid[x as usize][y as usize];

                            match c {
                                '0'..='9' => {
                                    let num: i32 = parse_num(&grid, x as usize, y as usize) as i32;
                                    nums.push(num);
                                    if grid[i - 1][j] != '.' {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    for d in same_row {
                        let x: i32 = i as i32 + d[0];
                        let y: i32 = j as i32 + d[1];

                        if x >= 0
                            && y >= 0
                            && (x as usize) < grid.len()
                            && (y as usize) < grid[i].len()
                        {
                            let c = grid[x as usize][y as usize];

                            match c {
                                '0'..='9' => {
                                    let num: i32 = parse_num(&grid, x as usize, y as usize) as i32;
                                    nums.push(num);
                                }
                                _ => {}
                            }
                        }
                    }
                    for d in lower_row {
                        let x: i32 = i as i32 + d[0];
                        let y: i32 = j as i32 + d[1];

                        if x >= 0
                            && y >= 0
                            && (x as usize) < grid.len()
                            && (y as usize) < grid[i].len()
                        {
                            let c = grid[x as usize][y as usize];

                            match c {
                                '0'..='9' => {
                                    let num: i32 = parse_num(&grid, x as usize, y as usize) as i32;
                                    nums.push(num);
                                    if grid[i + 1][j] != '.' {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    if nums.len() == 2 {
                        res += nums[0] * nums[1];
                    }
                }
                _ => {}
            };
            j += 1;
        }
        i += 1;
    }

    println!("res = {}", res);
}

fn parse_num(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let i = x;
    let mut j = y;
    let start: usize;
    loop {
        let c = grid[i][j];
        match c {
            '0'..='9' => {}
            _ => {
                start = j + 1;
                break;
            }
        }
        if j == 0 {
            start = 0;
            break;
        }
        j -= 1;
    }
    let mut res: u32 = 0;

    j = start;
    while j < grid.len() {
        let c = grid[i][j];
        match c {
            '0'..='9' => {
                res = res * 10 + (c.to_digit(10).unwrap());
            }
            _ => {
                break;
            }
        }
        j += 1;
    }
    return res;
}
