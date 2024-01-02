use pest::Parser;
use pest_derive::Parser;
use std::collections::HashSet;
use std::path::Path;

#[derive(Parser)]
#[grammar = "pest/challenge_4.pest"]
struct Challenge4Parser;

pub fn part_1() -> Option<u64> {
    let input_path: &Path = Path::new("./src/inputs/input4.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(part_1_instance).sum();
    Some(sum)
}

pub fn part_2() -> Option<u64> {
    let input_path: &Path = Path::new("./src/inputs/input4.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    // calculate the number of wins for each card and place in vector
    let wins: Vec<u64> = input
        .lines()
        .map(|line| Card::parse(line).wins())
        .collect::<Vec<u64>>();

    let mut cards = vec![1; input.lines().count()];
    for i in 0..wins.len() {
        let new_cards = cards[i];
        let start_index = i + 1;
        for j in (start_index)..(start_index + wins[i] as usize) {
            cards[j] += new_cards;
        }
    }

    let sum = cards.iter().sum();
    Some(sum)
}

/// Solve a single instance of the problem (i.e. a single line)
fn part_1_instance(line: &str) -> u64 {
    let card = Card::parse(line);
    let wins = card.wins();

    if wins == 0 {
        return 0;
    }

    // if there is a win, then the score is 2^(wins - 1)
    return 2u64.pow((card.wins() - 1).try_into().unwrap());
}

#[derive(Debug)]
struct Card {
    id: u64,
    winners: HashSet<u64>,
    guesses: HashSet<u64>,
}

impl Card {
    fn new() -> Card {
        Card {
            id: 0,
            winners: HashSet::new(),
            guesses: HashSet::new(),
        }
    }

    fn parse(input: &str) -> Card {
        let parse = Challenge4Parser::parse(Rule::card, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let mut card = Card::new();
        for record in parse.into_inner() {
            match record.as_rule() {
                Rule::card_id => {
                    card.id = record.as_str().parse::<u64>().unwrap();
                }
                Rule::winner => {
                    card.winners.insert(record.as_str().parse::<u64>().unwrap());
                }
                Rule::guess => {
                    card.guesses.insert(record.as_str().parse::<u64>().unwrap());
                }
                _ => unreachable!(),
            }
        }

        return card;
    }

    /// Returns the number of wins for this card
    fn wins(&self) -> u64 {
        self.winners.intersection(&self.guesses).count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(21568), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(11827296), part_2());
    }
}
