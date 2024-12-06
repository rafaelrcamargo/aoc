use std::fs;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    let rows = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    // Part One
    let columns = get_columns(&rows);
    let left_diagonals = get_left_diagonals(&rows);
    let valid_left_diagonals = get_valid_diagonals(&left_diagonals);
    let right_diagonals = get_right_diagonals(&rows);
    let valid_right_diagonals = get_valid_diagonals(&right_diagonals);

    let mut count = 0;
    count += check_windows(&rows);
    count += check_windows(&columns);
    count += check_windows(&valid_left_diagonals);
    count += check_windows(&valid_right_diagonals);
    println!("Part One: {}", count);

    // Part Two
    let windows = rows.len() - 3;
    let mut count = 0;
    for i in 0..=windows {
        for j in 0..=windows {
            let window: Vec<Vec<&str>> = (0..3)
                .map(|di| (0..3).map(|dj| rows[i + di][j + dj]).collect())
                .collect();

            count += check_x_mas(&window);
        }
    }
    println!("Part Two: {}", count);
}

fn get_columns<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut columns: Vec<Vec<T>> = Vec::new();

    for j in 0..cols {
        let mut column: Vec<T> = Vec::new();
        for i in 0..rows {
            column.push(matrix[i][j].clone());
        }
        columns.push(column);
    }

    columns
}
fn get_left_diagonals<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut diagonals: Vec<Vec<T>> = Vec::new();

    for row in 0..(rows + cols - 1) {
        let mut diagonal: Vec<T> = Vec::new();
        for j in 0..row + 1 {
            let i = row - j;
            if i < rows && j < cols {
                diagonal.push(matrix[i][j].clone());
            }
        }
        diagonals.push(diagonal);
    }

    diagonals
}
fn get_right_diagonals<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut diagonals: Vec<Vec<T>> = Vec::new();

    for row in 0..(rows + cols - 1) {
        let mut diagonal: Vec<T> = Vec::new();
        for j in 0..row + 1 {
            let i = row - j;
            if i < rows && j < cols {
                diagonal.push(matrix[i][cols - j - 1].clone());
            }
        }
        diagonals.push(diagonal);
    }

    diagonals
}
fn get_valid_diagonals<T: Clone>(diagonals: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    diagonals
        .iter()
        .filter(|diagonal| diagonal.len() >= 4)
        .map(|diagonal| diagonal.to_owned())
        .collect::<Vec<_>>()
}
fn check_xmas(word: String) -> bool {
    word.contains("XMAS") || word.contains("SAMX")
}
fn check_windows(matrix: &Vec<Vec<&str>>) -> usize {
    let mut count = 0;

    matrix
        .iter()
        .map(|diagonal| {
            diagonal
                .windows(4)
                .map(|chunk| chunk.join(""))
                .collect::<Vec<_>>()
        })
        .flatten()
        .for_each(|diagonal| {
            if check_xmas(diagonal) {
                count += 1;
            }
        });

    count
}
fn check_x_mas(matrix: &Vec<Vec<&str>>) -> usize {
    let d1 = [matrix[0][0], matrix[1][1], matrix[2][2]].join("");
    let d2 = [matrix[0][2], matrix[1][1], matrix[2][0]].join("");

    if check_mas(d1) && check_mas(d2) {
        1
    } else {
        0
    }
}
fn check_mas(word: String) -> bool {
    word.contains("MAS") || word.contains("SAM")
}
