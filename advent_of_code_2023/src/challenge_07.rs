use std::path::Path;

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input7.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    // sort the hands by score
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse_hand_pt1).collect();
    hands.sort_by(|a, b| score_pt1(&a.cards).cmp(&score_pt1(&b.cards)));

    let output: u32 = (0..hands.len())
        .map(|i| &hands[i].bid * (1 + i as u32))
        .sum();
    return Some(output);
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input7.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    // sort the hands by score
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse_hand_pt2).collect();
    hands.sort_by(|a: &Hand, b| score_pt2(&a.cards).cmp(&score_pt2(&b.cards)));

    let output: u32 = (0..hands.len())
        .map(|i| &hands[i].bid * (1 + i as u32))
        .sum();
    return Some(output);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

/// Map a card to a number for part 1
fn card_map_pt1(c: char) -> u32 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid card"),
    }
}

/// Map a card to a number for part 2
fn card_map_pt2(c: char) -> u32 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid card"),
    }
}

// Replace 'J' with all possible cards
fn expand_hand(input: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut all_hands = vec![input.clone()];
    for i in 0..5 {
        for hand in all_hands.clone() {
            if hand[i] == 0 {
                for j in 1..13 {
                    let mut new_hand = hand.clone();
                    new_hand[i] = j;
                    all_hands.push(new_hand);
                }
            }
        }
    }

    return all_hands;
}

fn score_pt1(cards: &Vec<u32>) -> u32 {
    regular_score(cards) + special_score(cards)
}

fn score_pt2(cards: &Vec<u32>) -> u32 {
    // do not expand when both hands have the same special score
    // instead just compute the regular score
    regular_score(cards) + expand_hand(cards).iter().map(special_score).max().unwrap()
}

fn regular_score(cards: &Vec<u32>) -> u32 {
    cards[0] * u32::pow(13, 4)
        + cards[1] * u32::pow(13, 3)
        + cards[2] * u32::pow(13, 2)
        + cards[3] * u32::pow(13, 1)
        + cards[4] * u32::pow(13, 0)
}

fn special_score(cards: &Vec<u32>) -> u32 {
    let base_score = u32::pow(13, 5);
    let mut c = cards.clone();
    c.sort();

    // five of a kind
    if c[0] == c[4] {
        return 7 * base_score;
    }
    // four of a kind
    if (c[0] == c[3]) || (c[1] == c[4]) {
        return 6 * base_score;
    }
    // full house
    if (c[0] == c[2] && c[3] == c[4]) || (c[0] == c[1] && c[2] == c[4]) {
        return 5 * base_score;
    }
    // three of a kind
    if (c[0] == c[2]) || (c[1] == c[3]) || (c[2] == c[4]) {
        return 4 * base_score;
    }
    // two pair
    if (c[0] == c[1] && c[2] == c[3])
        || (c[0] == c[1] && c[3] == c[4])
        || (c[1] == c[2] && c[3] == c[4])
    {
        return 3 * base_score;
    }
    // one pair
    if (c[0] == c[1]) || (c[1] == c[2]) || (c[2] == c[3]) || (c[3] == c[4]) {
        return 2 * base_score;
    }
    // high card
    if (c[0] != c[1]) && (c[1] != c[2]) && (c[2] != c[3]) && (c[3] != c[4]) {
        return 1 * base_score;
    }
    return 0;
}

impl Hand {
    fn parse_hand_pt1(input: &str) -> Hand {
        let mut split = input.split_whitespace();
        let cards = split
            .next()
            .unwrap()
            .chars()
            .into_iter()
            .map(card_map_pt1)
            .collect::<Vec<u32>>();
        let bid = split.next().unwrap().parse::<u32>().unwrap();

        assert_eq!(cards.len(), 5);
        Hand { cards, bid }
    }

    fn parse_hand_pt2(input: &str) -> Hand {
        let mut split = input.split_whitespace();
        let cards = split
            .next()
            .unwrap()
            .chars()
            .into_iter()
            .map(card_map_pt2)
            .collect::<Vec<u32>>();
        let bid = split.next().unwrap().parse::<u32>().unwrap();

        assert_eq!(cards.len(), 5);
        Hand { cards, bid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(253313241), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(253362743), part_2());
    }
}
