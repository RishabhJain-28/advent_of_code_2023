use std::fs;

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_11.txt").unwrap();
    let mut matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = matrix.len();
    let m = matrix[0].len();

    for j in 0..m {
        let mut has_galaxy = false;
        for i in 0..n {
            if matrix[i][j] == '#' {
                has_galaxy = true;
            }
        }
        if has_galaxy == false {
            for i in 0..n {
                matrix[i][j] = '=';
            }
        }
    }

    let mut i_offset = 0;
    let mut galaxies = vec![];
    for i in 0..n {
        let mut j_offset = 0;
        let mut has_galaxy = false;
        for j in 0..m {
            if matrix[i][j] == '=' {
                j_offset += 1;
            } else if matrix[i][j] == '#' {
                has_galaxy = true;
                galaxies.push((i + i_offset, j + j_offset));
            }
        }
        if has_galaxy == false {
            i_offset += 1;
        }
    }

    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            res += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }
    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_11.txt").unwrap();
    let mut matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = matrix.len();
    let m = matrix[0].len();

    for j in 0..m {
        let mut has_galaxy = false;
        for i in 0..n {
            if matrix[i][j] == '#' {
                has_galaxy = true;
            }
        }
        if has_galaxy == false {
            for i in 0..n {
                matrix[i][j] = '=';
            }
        }
    }

    let mut i_offset = 0;
    let mut galaxies = vec![];
    for i in 0..n {
        let mut j_offset = 0;
        let mut has_galaxy = false;
        for j in 0..m {
            if matrix[i][j] == '=' {
                j_offset += 1;
            } else if matrix[i][j] == '#' {
                has_galaxy = true;
                galaxies.push((i + i_offset * (1000000 - 1), j + j_offset * (1000000 - 1)));
            }
        }
        if has_galaxy == false {
            i_offset += 1;
        }
    }

    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            res += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }
    println!("res = {res}");
}
