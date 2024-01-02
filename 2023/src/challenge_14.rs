use std::{cmp::max, path::Path};

pub fn part_1() -> Option<u32> {
    to_left(
        &[true, false, true, false, true],
        &[false, false, false, false, false],
    );

    //     let input = "OOOO.#.O..
    // OO..#....#
    // OO..O##..O
    // O..#.OO...
    // ........#.
    // ..#....#.#
    // ..O..#.O.O
    // ..O.......
    // #....###..
    // #....#....";

    //     let input2 = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";

    //     // let input_path: &Path = Path::new("./src/inputs/input14.txt");
    //     // let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    //     let mut grid = parse(&input);

    //     grid = tilt_north(grid);
    //     let score = count(&grid);

    //     return Some(score);
    None
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input14.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let input = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    //     let input2 = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";

    let mut grid: Vec<Vec<char>> = parse(&input);

    for _ in 0..1 {
        grid = spin_cycle(grid);
    }

    let score = count(&grid);

    return Some(score);
}

// fn tilt_north(rock_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     let rows = rock_grid.len();
//     let cols = rock_grid[0].len();
//     let max_iters = max(rows, cols);

//     let mut new_grid = rock_grid.clone();

//     for _ in 0..max_iters {
//         for i in 1..rows {
//             for j in 0..cols {
//                 let curr_char = rock_grid[i][j];
//                 let up_char = rock_grid[i - 1][j];
//                 if curr_char == 'O' && up_char == '.' {
//                     // swap
//                     new_grid[i][j] = '.';
//                     new_grid[i - 1][j] = 'O';
//                 }
//             }
//         }
//     }

//     return new_grid;
// }

fn tilt_south(rock_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = rock_grid.len();
    let cols = rock_grid[0].len();
    let max_iters = max(rows, cols);

    let mut new_grid = rock_grid.clone();

    for _ in 0..max_iters {
        for i in (0..(rows - 1)).rev() {
            for j in 0..cols {
                let curr_char = rock_grid[i][j];
                let down_char = rock_grid[i + 1][j];
                if curr_char == 'O' && down_char == '.' {
                    // swap
                    new_grid[i][j] = '.';
                    new_grid[i + 1][j] = 'O';
                }
            }
        }
    }

    return new_grid;
}

fn tilt_east(rock_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = rock_grid.len();
    let cols = rock_grid[0].len();
    let max_iters = max(rows, cols);

    let mut new_grid = rock_grid.clone();

    for _ in 0..max_iters {
        for j in (0..cols - 1).rev() {
            for i in 0..rows {
                let curr_char = rock_grid[i][j];
                let right_char = rock_grid[i][j + 1];
                if curr_char == 'O' && right_char == '.' {
                    // swap
                    new_grid[i][j] = '.';
                    new_grid[i][j + 1] = 'O';
                }
            }
        }
    }

    return new_grid;
}

fn tilt_west(rock_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = rock_grid.len();
    let cols = rock_grid[0].len();
    let max_iters = max(rows, cols);

    let mut new_grid = rock_grid.clone();

    for _ in 0..max_iters {
        for i in 0..rows {
            for j in 1..cols {
                let curr_char: char = rock_grid[i][j];
                let left_char: char = rock_grid[i][j - 1];
                if curr_char == 'O' && left_char == '.' {
                    // swap
                    new_grid[i][j] = '.';
                    new_grid[i][j - 1] = 'O';
                }
            }
        }
    }

    return new_grid;
}

fn spin_cycle(rock_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = rock_grid.clone();

    // new_grid = tilt_north(new_grid);

    // println!("------------------");
    // for i in 0..new_grid.len() {
    //     println!("{:?}", new_grid[i]);
    // }
    new_grid = tilt_west(new_grid);
    // println!("------------------");
    // for i in 0..new_grid.len() {
    //     println!("{:?}", new_grid[i]);
    // }
    // println!("WEST DONE above");

    new_grid = tilt_south(new_grid);
    // println!("------------------");
    // for i in 0..new_grid.len() {
    //     println!("{:?}", new_grid[i]);
    // }
    new_grid = tilt_east(new_grid);

    return new_grid;
}

fn count(rock_grid: &Vec<Vec<char>>) -> u32 {
    let size = rock_grid.len();
    let mut score = 0;
    for i in 0..size {
        let multiplier = size - i;
        let count = &rock_grid[i].iter().filter(|x| **x == 'O').count();
        score += (multiplier * count) as u32;
    }

    return score;
}

fn to_left(rocks: &[bool], obstacles: &[bool]) -> Vec<bool> {
    let mut xx: Vec<u32> = Vec::new();

    xx.push(0);

    for i in 0..rocks.len() {
        if obstacles[i] {
            xx.push(0);
        } else if rocks[i] {
            let j = xx.len() - 1;
            xx[j] += 1;
        }
    }

    // allocate the rocks
    let mut rock_alloc: Vec<bool> = vec![false; rocks.len()];
    let mut i = 0;
    for x in xx {
        for _ in 0..x {
            rock_alloc[i] = true;
            i += 1;
        }
        i += 1;
    }

    // print!("rock alloc: {:?}\n", rock_alloc);
    return rock_alloc;
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut rock_grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        rock_grid.push(line.chars().collect());
    }

    return rock_grid;
}
