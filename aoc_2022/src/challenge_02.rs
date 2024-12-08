use std::{cmp::Ordering, fs::read_to_string, path::Path};

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input2.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_1_instance).sum();
    Some(sum)
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input2.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_2_instance).sum();
    Some(sum)
}

fn part_1_instance(line: &str) -> u32 {
    let parsed_line: Vec<&str> = line.split_whitespace().collect();
    let opposition_move = match parsed_line[0] {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => unreachable!(),
    };
    let my_move = match parsed_line[1] {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => unreachable!(),
    };

    game_score(&my_move, &opposition_move)
}

fn part_2_instance(line: &str) -> u32 {
    let parsed_line: Vec<&str> = line.split_whitespace().collect();
    let opposition_move = match parsed_line[0] {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => unreachable!(),
    };
    let desired_outcome = match parsed_line[1] {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!(),
    };

    // lookup table for the move that will result in the desired outcome
    let my_move = match desired_outcome {
        Outcome::Lose => match opposition_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Draw => opposition_move,
        Outcome::Win => match opposition_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
    };

    game_score(&my_move, &opposition_move)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

/// a is the player's move, b is the opposition's move
/// returns the outcome of the game for the player
fn play(a: &Move, b: &Move) -> Outcome {
    if a == b {
        Outcome::Draw
    } else if a > b {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

/// returns the score for the player for a single round
fn game_score(a: &Move, b: &Move) -> u32 {
    match play(a, b) {
        Outcome::Lose => 0 + a.score(),
        Outcome::Draw => 3 + a.score(),
        Outcome::Win => 6 + a.score(),
    }
}

// implement ordering where a "bigger" move beats a "smaller" move
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            (Move::Paper, Move::Scissors) => Some(Ordering::Less),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            // otherwise they are the same and therefore equal
            _ => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(13221));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(13131));
    }
}
