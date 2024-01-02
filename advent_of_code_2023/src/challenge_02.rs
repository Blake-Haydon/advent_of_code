use pest::Parser;
use pest_derive::Parser;
use std::path::Path;

#[derive(Parser)]
#[grammar = "pest/challenge_2.pest"]
struct Challenge2Parser;

pub fn part_1() -> Option<u64> {
    let input_path: &Path = Path::new("./src/inputs/input2.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_1_instance).sum();
    Some(sum)
}

pub fn part_2() -> Option<u64> {
    let input_path: &Path = Path::new("./src/inputs/input2.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_2_instance).sum();
    Some(sum)
}

/// Solve a single instance of the problem (i.e. a single line)
fn part_1_instance(line: &str) -> u64 {
    let game = Game::parse(line);
    if game.is_valid(12, 13, 14) {
        return game.id;
    }
    return 0;
}

fn part_2_instance(line: &str) -> u64 {
    let game = Game::parse(line);

    // Find the max of each colour
    let max_r = game.max_r();
    let max_g = game.max_g();
    let max_b = game.max_b();

    return max_r * max_g * max_b;
}

#[derive(Debug)]
struct Game {
    id: u64,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    r: u64,
    g: u64,
    b: u64,
}

impl Game {
    fn new() -> Game {
        Game {
            id: 0,
            rounds: Vec::new(),
        }
    }

    fn parse(input: &str) -> Game {
        let parse: pest::iterators::Pair<'_, Rule> = Challenge2Parser::parse(Rule::game, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let mut game = Game::new();
        for record in parse.into_inner() {
            match record.as_rule() {
                Rule::round => {
                    let mut round = Round { r: 0, g: 0, b: 0 };
                    for field in record.into_inner() {
                        let mut colour = "";
                        let mut count = 0;
                        match field.as_rule() {
                            Rule::ball => {
                                for ball in field.into_inner() {
                                    match ball.as_rule() {
                                        Rule::ball_colour => colour = ball.as_str(),
                                        Rule::ball_count => {
                                            count = ball.as_str().parse::<u64>().unwrap()
                                        }
                                        _ => unreachable!(),
                                    }
                                }

                                // Assign the count to the correct colour
                                match colour {
                                    "red" => round.r = count,
                                    "green" => round.g = count,
                                    "blue" => round.b = count,
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    game.rounds.push(round);
                }
                Rule::game_id => game.id = record.as_str().parse::<u64>().unwrap(),
                _ => unreachable!(),
            }
        }

        return game;
    }

    fn is_valid(&self, max_r: u64, max_g: u64, max_b: u64) -> bool {
        for round in &self.rounds {
            if !round.is_valid(max_r, max_g, max_b) {
                return false;
            }
        }
        return true;
    }

    /// maximum number of red balls in a game
    fn max_r(&self) -> u64 {
        self.rounds.iter().map(|round| round.r).max().unwrap()
    }

    /// maximum number of green balls in a game
    fn max_g(&self) -> u64 {
        self.rounds.iter().map(|round| round.g).max().unwrap()
    }

    /// maximum number of blue balls in a game
    fn max_b(&self) -> u64 {
        self.rounds.iter().map(|round| round.b).max().unwrap()
    }
}

impl Round {
    fn is_valid(&self, max_r: u64, max_g: u64, max_b: u64) -> bool {
        self.r <= max_r && self.g <= max_g && self.b <= max_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(2439), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(63711), part_2());
    }
}
