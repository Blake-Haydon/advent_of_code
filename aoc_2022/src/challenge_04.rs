use std::{fs::read_to_string, path::Path};

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input4.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().filter(|line| part_1_instance(line)).count();
    Some(sum as u32)
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input4.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().filter(|line| part_2_instance(line)).count();
    Some(sum as u32)
}

fn part_1_instance(line: &str) -> bool {
    let (a_start, a_end, b_start, b_end) = parse(line);
    full_overlap(a_start, a_end, b_start, b_end)
}

fn part_2_instance(line: &str) -> bool {
    let (a_start, a_end, b_start, b_end) = parse(line);
    some_overlap(a_start, a_end, b_start, b_end)
}

/// split an input line into four numbers
fn parse(line: &str) -> (u32, u32, u32, u32) {
    let split_line: Vec<&str> = line.split(",").collect();
    let a: Vec<&str> = split_line[0].split("-").collect();
    let b: Vec<&str> = split_line[1].split("-").collect();

    let a_start: u32 = a[0].parse().unwrap();
    let a_end: u32 = a[1].parse().unwrap();
    let b_start: u32 = b[0].parse().unwrap();
    let b_end: u32 = b[1].parse().unwrap();

    (a_start, a_end, b_start, b_end)
}

/// check to see if two ranges fully overlap
fn full_overlap(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    a_start <= b_start && a_end >= b_end || b_start <= a_start && b_end >= a_end
}

/// check to see if two ranges overlap at all
fn some_overlap(a_start: u32, a_end: u32, b_start: u32, b_end: u32) -> bool {
    let start = a_start.max(b_start); // take the larger of the two starts
    let end = a_end.min(b_end); // take the smaller of the two ends

    if start > end {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(485));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(857));
    }
}
