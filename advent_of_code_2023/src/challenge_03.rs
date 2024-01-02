use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input3.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let symbol_coords = symbol_coords(&input);

    let mut valid_part_num: Vec<u32> = vec![];
    let mut num_coords: Vec<Coordinate> = vec![];
    let mut num_digits: Vec<u32> = vec![];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num_coords.push(Coordinate::new(i, j)); // a number spans multiple coordinates
                num_digits.push(c.to_digit(10).unwrap()); // a number can have multiple digits
            } else {
                let num = num_digits.iter().fold(0, |acc, digit| acc * 10 + digit);
                let num_is_close = num_coords.iter().any(|num_coord| {
                    symbol_coords
                        .iter()
                        .any(|symbol_coord: &Coordinate| num_coord.is_close_to(symbol_coord))
                });

                // if the number is close to a symbol, then it is valid.
                if num_is_close {
                    valid_part_num.push(num);
                }

                // clear the temporary variables for the current number
                num_digits = vec![];
                num_coords = vec![];
            }
        }
    }

    Some(valid_part_num.iter().sum())
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input3.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let gear_coords = gear_coords(&input);
    let mut gear_map: HashMap<Coordinate, HashSet<u32>> = gear_coords
        .clone()
        .into_iter()
        .map(|k| (k, HashSet::new()))
        .collect();

    let mut num_coords: Vec<Coordinate> = vec![];
    let mut num_digits: Vec<u32> = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num_coords.push(Coordinate::new(i, j));
                num_digits.push(c.to_digit(10).unwrap());
            } else {
                let num = num_digits.iter().fold(0, |acc, digit| acc * 10 + digit);

                // add the current number to the gear map
                for num_coord in &num_coords {
                    for gear_coord in &gear_coords {
                        if num_coord.is_close_to(gear_coord) {
                            gear_map.get_mut(gear_coord).unwrap().insert(num);
                        }
                    }
                }

                // clear the temporary variables for the current number
                num_digits = vec![];
                num_coords = vec![];
            }
        }
    }

    let gear_sum = gear_map
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().nth(0).unwrap() * v.iter().nth(1).unwrap())
        .sum();

    Some(gear_sum)
}

/// a symbol is any character that is not numeric or a period.
fn is_symbol(c: char) -> bool {
    !(c == '.' || c.is_digit(10))
}

/// a gear is denoted by a '*'
fn is_gear(c: char) -> bool {
    c == '*'
}

/// output a list of all coordinates that contain a symbol.
fn symbol_coords(input: &str) -> Vec<Coordinate> {
    let mut coords = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if is_symbol(c) {
                coords.push(Coordinate::new(i, j));
            }
        }
    }
    coords
}

/// output a list of all coordinates that contain a gear.
fn gear_coords(input: &str) -> Vec<Coordinate> {
    let mut coords = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if is_gear(c) {
                coords.push(Coordinate::new(i, j));
            }
        }
    }
    coords
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    /// if the coordinate is one unit away from the other coordinate
    /// then it is deemed "close".
    fn is_close_to(&self, other: &Coordinate) -> bool {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        return dx <= 1 && dy <= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(546563), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(91031374), part_2());
    }
}
