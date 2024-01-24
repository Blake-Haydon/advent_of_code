use std::{collections::HashSet, fs::read_to_string, path::Path};

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input3.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_1_instance).sum();
    Some(sum)
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input3.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let mut sum = 0;
    let input_len = input.lines().count();
    for i in 0..(input_len / 3) {
        sum += part_2_instance(
            input.lines().nth(3 * i + 0).unwrap(),
            input.lines().nth(3 * i + 1).unwrap(),
            input.lines().nth(3 * i + 2).unwrap(),
        );
    }

    Some(sum)
}

fn part_1_instance(line: &str) -> u32 {
    let (s1, s2) = split_string(line);
    let set1: HashSet<u32> = s1.chars().map(score).collect();
    let set2: HashSet<u32> = s2.chars().map(score).collect();
    set1.intersection(&set2).copied().sum()
}

fn part_2_instance(s1: &str, s2: &str, s3: &str) -> u32 {
    let set1: HashSet<u32> = s1.chars().map(score).collect();
    let set2: HashSet<u32> = s2.chars().map(score).collect();
    let set3: HashSet<u32> = s3.chars().map(score).collect();

    let mutual: HashSet<u32> = set1.intersection(&set2).copied().collect();
    let mutual: HashSet<u32> = mutual.intersection(&set3).copied().collect();

    // sum the scores of the mutual items
    mutual.iter().sum()
}

/// score a character
fn score(c: char) -> u32 {
    let uppercase_offset = 26;
    if c.is_uppercase() {
        return c as u32 - 64 + uppercase_offset;
    } else {
        return c as u32 - 96;
    }
}

/// split a string in half
fn split_string(s: &str) -> (&str, &str) {
    let half = s.len() / 2;
    s.split_at(half)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(7889));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(2825));
    }
}
