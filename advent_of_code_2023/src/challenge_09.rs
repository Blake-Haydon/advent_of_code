use std::cmp::PartialEq;
use std::{fs::read_to_string, path::Path};

pub fn part_1() -> Option<i64> {
    let input_path: &Path = Path::new("./src/inputs/input9.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let mut sum: i64 = 0;
    for line in input.lines() {
        let x = line
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        sum += part_1_instance(&x);
    }

    return Some(sum);
}

pub fn part_2() -> Option<i64> {
    let input_path: &Path = Path::new("./src/inputs/input9.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let mut sum: i64 = 0;
    for line in input.lines() {
        let x = line
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        sum += part_2_instance(&x);
    }

    return Some(sum);
}

fn diff(x: &Vec<i64>) -> Vec<i64> {
    let x_offset = x.iter().skip(1);
    let difference = |(a, b)| b - a;
    x.iter().zip(x_offset).map(difference).collect()
}

fn all_same<T: PartialEq + Copy>(vec: &Vec<T>, val: T) -> bool {
    vec.iter().all(|&x| x == val)
}

fn part_1_instance(x: &Vec<i64>) -> i64 {
    let mut x = x.clone();
    let mut val = vec![];

    val.push(*x.last().unwrap());
    while !all_same(&x, 0) {
        x = diff(&x);
        val.push(*x.last().unwrap());
    }

    val.iter().fold(0, |acc, x| acc - x);

    // sum all of the last values to get the solution
    return val.iter().sum();
}

fn part_2_instance(x: &Vec<i64>) -> i64 {
    let mut x = x.clone();
    let mut val = vec![];

    val.push(*x.first().unwrap());
    while !all_same(&x, 0) {
        x = diff(&x);
        val.push(*x.first().unwrap());
    }

    // sum all of the values from the before extrapolation
    return val.iter().rev().fold(0, |acc, x| x - acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(1684566095), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(1136), part_2());
    }
}
