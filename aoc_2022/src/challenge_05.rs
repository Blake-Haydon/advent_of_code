use std::{fs::read_to_string, path::Path};

pub fn part_1() -> Option<String> {
    let input_moves_path: &Path = Path::new("./src/inputs/input5_moves.txt");
    let input_tower_path: &Path = Path::new("./src/inputs/input5_tower.txt");

    let input_moves: String = read_to_string(input_moves_path).expect("Error reading file");
    let input_tower: String = read_to_string(input_tower_path).expect("Error reading file");

    // parse the moves and tower
    let moves: Vec<Move> = input_moves.lines().map(Move::parse).collect();
    let mut stack_list: StackList<char> = StackList::<char>::parse(input_tower, 9);

    // pop and push the values according to the moves
    for m in moves {
        for _ in 0..m.n {
            let value: Option<char> = stack_list.pop(m.from);
            match value {
                Some(v) => stack_list.push(m.to, v),
                None => break,
            };
        }
    }

    Some(top_string(stack_list))
}

pub fn part_2() -> Option<String> {
    let input_moves_path: &Path = Path::new("./src/inputs/input5_moves.txt");
    let input_tower_path: &Path = Path::new("./src/inputs/input5_tower.txt");

    let input_moves: String = read_to_string(input_moves_path).expect("Error reading file");
    let input_tower: String = read_to_string(input_tower_path).expect("Error reading file");

    // parse the moves and tower
    let moves: Vec<Move> = input_moves.lines().map(Move::parse).collect();
    let mut stack_list: StackList<char> = StackList::<char>::parse(input_tower, 9);

    for m in moves {
        // pop n values from stack
        let mut popped_values = Vec::new();
        for _ in 0..m.n {
            let value: Option<char> = stack_list.pop(m.from);
            match value {
                Some(v) => popped_values.push(v),
                None => break,
            };
        }

        // reverse the popped values
        popped_values.reverse();

        // place on new stack
        for v in popped_values {
            stack_list.push(m.to, v);
        }
    }

    Some(top_string(stack_list))
}

/// return the top string of a stack list
fn top_string(stack_list: StackList<char>) -> String {
    let mut output_string: String = String::new();
    for stack in stack_list.list {
        match stack.last() {
            Some(v) => output_string.push(*v),
            None => {}
        }
    }
    output_string
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n: u32,
}

impl Move {
    fn new(from: usize, to: usize, n: u32) -> Move {
        Move { from, to, n }
    }

    fn parse(line: &str) -> Move {
        let split_line: Vec<&str> = line.split(' ').collect();
        let ones_index_offset = 1;

        // use zeros indexing for all the numbers
        let n = split_line[1].parse::<u32>().unwrap();
        let from = split_line[3].parse::<usize>().unwrap() - ones_index_offset;
        let to = split_line[5].parse::<usize>().unwrap() - ones_index_offset;

        Move::new(from, to, n)
    }
}

#[derive(Debug)]
struct StackList<T> {
    list: Vec<Vec<T>>,
}

impl<T: std::fmt::Debug> StackList<T> {
    fn new(n: usize) -> StackList<T> {
        let mut list: Vec<Vec<T>> = Vec::new();
        for _ in 0..n {
            list.push(Vec::new());
        }

        StackList { list }
    }

    fn parse(str: String, n: usize) -> StackList<char> {
        let mut stack_list: StackList<char> = StackList::new(n);
        for line in str.lines() {
            for (i, c) in line.chars().enumerate() {
                if c.is_alphanumeric() {
                    let index = (i - 1) / 4;
                    stack_list.place_at_bottom(index, c);
                    continue;
                }
            }
        }
        stack_list
    }

    fn place_at_bottom(&mut self, index: usize, val: T) {
        self.list[index].insert(0, val);
    }

    fn push(&mut self, index: usize, val: T) {
        self.list[index].push(val);
    }

    fn pop(&mut self, index: usize) -> Option<T> {
        self.list[index].pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some("LJSVLTWQM".to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some("BRQWDBBJM".to_string()));
    }
}
