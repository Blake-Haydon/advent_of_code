use std::{fs::read_to_string, path::Path};

pub fn part_1() -> Option<usize> {
    let input_path: &Path = Path::new("./src/inputs/input6.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    for i in 0.. {
        let start_i = i;
        let end_i = i + 4;
        let s: &str = &input[start_i..end_i];
        if all_different(s) {
            return Some(end_i);
        }
    }
    None
}

pub fn part_2() -> Option<usize> {
    let input_path: &Path = Path::new("./src/inputs/input6.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    for i in 0.. {
        let start_i = i;
        let end_i = i + 14;
        let s: &str = &input[start_i..end_i];
        if all_different(s) {
            return Some(end_i);
        }
    }
    None
}

/// given a string, determine if all characters are different
fn all_different(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.dedup();
    chars.len() == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(1262));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(3444));
    }
}
