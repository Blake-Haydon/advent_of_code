use std::{cmp::max, fs::read_to_string, path::Path};

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input1.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let mut max_calories: u32 = 0;
    let mut calories: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            // check if the current calories are greater than the max
            max_calories = max(max_calories, calories);

            // reset the current calorie count
            calories = 0;

            // continue to the next line (do not read the empty line)
            continue;
        }

        let num: u32 = line.parse().unwrap();
        calories += num;
    }

    Some(max_calories)
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input1.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let mut calories: u32 = 0;
    let mut all_calories: Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            // add the current calories to the list
            all_calories.push(calories);

            // reset the current calories counter
            calories = 0;

            // continue to the next line (do not read the empty line)
            continue;
        }

        let num: u32 = line.parse().unwrap();
        calories += num;
    }

    // sort by the highest calories and take the top 3
    all_calories.sort();
    all_calories.reverse();
    Some(all_calories.iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(71924));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(210406));
    }
}
