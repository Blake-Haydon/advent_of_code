use std::path::Path;

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input1.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_1_instance).sum();
    Some(sum)
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input1.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_2_instance).sum();
    Some(sum)
}

/// Solve a single instance of the problem
fn part_1_instance(line: &str) -> u32 {
    let ints = extract_digits(line);
    if ints.len() > 0 {
        return 10 * ints.first().unwrap() + ints.last().unwrap();
    }
    return 0;
}

/// solve a single instance of the problem
fn part_2_instance(input: &str) -> u32 {
    let mut ints = vec![];

    let radix = 10;
    for i in 0..input.len() {
        let sub_input = input.get(i..).unwrap();
        if sub_input.find("zero") == Some(0) {
            ints.push(0);
        }
        if sub_input.find("one") == Some(0) {
            ints.push(1);
        }
        if sub_input.find("two") == Some(0) {
            ints.push(2);
        }
        if sub_input.find("three") == Some(0) {
            ints.push(3);
        }
        if sub_input.find("four") == Some(0) {
            ints.push(4);
        }
        if sub_input.find("five") == Some(0) {
            ints.push(5);
        }
        if sub_input.find("six") == Some(0) {
            ints.push(6);
        }
        if sub_input.find("seven") == Some(0) {
            ints.push(7);
        }
        if sub_input.find("eight") == Some(0) {
            ints.push(8);
        }
        if sub_input.find("nine") == Some(0) {
            ints.push(9);
        }

        // add next char to ints array if it is a digit
        let ith_char = input.chars().nth(i).unwrap();
        if ith_char.is_digit(radix) {
            let d = ith_char.to_digit(radix).unwrap();
            ints.push(d);
        }
    }

    return 10 * ints.first().unwrap() + ints.last().unwrap();
}

/// Extract all digits from a string and place them in a vector
fn extract_digits(input: &str) -> Vec<u32> {
    let predicate = |c: &char| c.is_digit(10);
    let radix = 10;
    input
        .chars()
        .filter(predicate)
        .map(|c| c.to_digit(radix).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_instances() {
        let inputs = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let outputs = vec![12, 38, 15, 77];

        for i in 0..inputs.len() {
            let expected = outputs[i];
            let actual = part_1_instance(inputs[i]);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_part_2_instances() {
        let inputs = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        // 29, 83, 13, 24, 42, 14, and 76
        let outputs = vec![29, 83, 13, 24, 42, 14, 76];

        for i in 0..inputs.len() {
            let expected = outputs[i];
            let actual = part_2_instance(inputs[i]);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_part_1() {
        assert_eq!(Some(54081), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(54649), part_2());
    }
}
