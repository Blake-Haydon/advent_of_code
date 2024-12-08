use std::{fs::read_to_string, path::Path};

pub fn part_1() -> Option<i32> {
    let input_path: &Path = Path::new("./src/inputs/input8.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let num_rows: usize = input.lines().count();
    let num_cols: usize = input.lines().next().unwrap().chars().count();

    let values: Vec<i32> = parse(&input);
    Some(visible(&values, num_cols, num_rows))
}

pub fn part_2() -> Option<i32> {
    let input_path: &Path = Path::new("./src/inputs/input8.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let num_rows: usize = input.lines().count();
    let num_cols: usize = input.lines().next().unwrap().chars().count();

    let values: Vec<i32> = parse(&input);
    let mut scenic_scores: Vec<i32> = vec![0; values.len()];

    for row in 0..num_rows {
        for col in 0..num_cols {
            let i = index_2d(row, col, num_cols);
            scenic_scores[i] = scenic_score(row, col, &values, num_cols, num_rows);
        }
    }

    let max_score: i32 = scenic_scores.iter().copied().max().unwrap();
    Some(max_score)
}

/// parse input string into a single vector of numbers
fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as i32)
        .collect()
}

/// Index a 2d array using a 1d array
fn index_2d(row: usize, col: usize, num_cols: usize) -> usize {
    row * num_cols + col
}

/// Calculate the number of trees visible from the outside
fn visible(values: &Vec<i32>, num_cols: usize, num_rows: usize) -> i32 {
    let mut visible: Vec<i32> = vec![0; values.len()];

    // go down all rows
    for col in 0..num_cols {
        let mut max: i32 = -1;
        for n in 0..num_rows {
            let i = index_2d(n, col, num_cols);
            if values[i] > max {
                max = values[i];
                visible[i] = 1;
            }
        }
    }

    // go up all rows
    for col in 0..num_cols {
        let mut max: i32 = -1;
        for n in (0..num_rows).rev() {
            let i = index_2d(n, col, num_cols);
            if values[i] > max {
                max = values[i];
                visible[i] = 1;
            }
        }
    }

    // go right all cols
    for row in 0..num_rows {
        let mut max: i32 = -1;
        for n in 0..num_cols {
            let i = index_2d(row, n, num_cols);
            if values[i] > max {
                max = values[i];
                visible[i] = 1;
            }
        }
    }

    // go left all cols
    for row in 0..num_rows {
        let mut max: i32 = -1;
        for n in (0..num_cols).rev() {
            let i = index_2d(row, n, num_cols);
            if values[i] > max {
                max = values[i];
                visible[i] = 1;
            }
        }
    }

    return visible.iter().filter(|&&n| n == 1).count() as i32;
}

/// calculate the scenic score of a given cell
fn scenic_score(
    row: usize,
    col: usize,
    values: &Vec<i32>,
    num_cols: usize,
    num_rows: usize,
) -> i32 {
    let i = index_2d(row, col, num_cols);
    let max = values[i];

    let mut a: i32 = 0;
    for n in (col + 1)..num_cols {
        a += 1;
        if values[index_2d(row, n, num_cols)] >= max {
            break;
        }
    }

    let mut b: i32 = 0;
    for n in (0..col).rev() {
        b += 1;
        if values[index_2d(row, n, num_cols)] >= max {
            break;
        }
    }

    let mut c: i32 = 0;
    for n in (row + 1)..num_rows {
        c += 1;
        if values[index_2d(n, col, num_cols)] >= max {
            break;
        }
    }

    let mut d: i32 = 0;
    for n in (0..row).rev() {
        d += 1;
        if values[index_2d(n, col, num_cols)] >= max {
            break;
        }
    }

    return a * b * c * d;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(1851));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(574080));
    }
}
