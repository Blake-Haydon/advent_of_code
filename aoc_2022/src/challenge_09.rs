use std::{collections::HashSet, fs::read_to_string, path::Path};

pub fn part_1() -> Option<usize> {
    let input_path: &Path = Path::new("./src/inputs/input9.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let moves: Vec<Move> = input.lines().map(Move::parse).collect();

    let head_positions: Vec<Position> = play(moves);
    let tail_positions: Vec<Position> = follow(&head_positions);

    let mut positions: HashSet<Position> = HashSet::new();
    for p in tail_positions {
        positions.insert(p);
    }

    Some(positions.len())
}

pub fn part_2() -> Option<usize> {
    let input_path: &Path = Path::new("./src/inputs/input9.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    let moves: Vec<Move> = input.lines().map(Move::parse).collect();

    let head_positions: Vec<Position> = play(moves);
    let tail_positions_1: Vec<Position> = follow(&head_positions);
    let tail_positions_2: Vec<Position> = follow(&tail_positions_1);
    let tail_positions_3: Vec<Position> = follow(&tail_positions_2);
    let tail_positions_4: Vec<Position> = follow(&tail_positions_3);
    let tail_positions_5: Vec<Position> = follow(&tail_positions_4);
    let tail_positions_6: Vec<Position> = follow(&tail_positions_5);
    let tail_positions_7: Vec<Position> = follow(&tail_positions_6);
    let tail_positions_8: Vec<Position> = follow(&tail_positions_7);
    let tail_positions_9: Vec<Position> = follow(&tail_positions_8);

    println!("{}", tail_positions_9.len());
    for (i, p) in tail_positions_9.iter().enumerate() {
        println!("{}: {:?}", i, p);
    }

    let mut positions: HashSet<Position> = HashSet::new();
    for p in tail_positions_9 {
        positions.insert(p);
    }

    Some(positions.len())
}

#[derive(Debug, Clone, Hash, Copy)]
struct Position(i32, i32);

impl Position {
    fn new() -> Position {
        Position(0, 0)
    }

    fn set(&mut self, other: &Position) {
        self.0 = other.0;
        self.1 = other.1;
    }

    fn up(&mut self, n: i32) {
        self.1 += n;
    }

    fn down(&mut self, n: i32) {
        self.1 -= n;
    }

    fn left(&mut self, n: i32) {
        self.0 -= n;
    }

    fn right(&mut self, n: i32) {
        self.0 += n;
    }

    fn within_radius(&self, other: &Position, radius: i32) -> bool {
        (self.0 - other.0).abs() <= radius && (self.1 - other.1).abs() <= radius
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Position {}

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    fn parse(input: &str) -> Move {
        let split_input: Vec<&str> = input.split(" ").collect();
        let direction: &str = split_input[0];
        let distance: i32 = split_input[1].parse().unwrap();
        match direction {
            "U" => Move::Up(distance),
            "D" => Move::Down(distance),
            "L" => Move::Left(distance),
            "R" => Move::Right(distance),
            _ => panic!("Invalid move"),
        }
    }
}

fn play(moves: Vec<Move>) -> Vec<Position> {
    let mut pos: Position = Position::new();
    let mut positions: Vec<Position> = Vec::new();
    moves.iter().for_each(|m| match *m {
        Move::Up(n) => {
            for _ in 0..n {
                pos.up(1);
                positions.push(pos);
            }
        }
        Move::Down(n) => {
            for _ in 0..n {
                pos.down(1);
                positions.push(pos);
            }
        }
        Move::Left(n) => {
            for _ in 0..n {
                pos.left(1);
                positions.push(pos);
            }
        }
        Move::Right(n) => {
            for _ in 0..n {
                pos.right(1);
                positions.push(pos);
            }
        }
    });
    return positions;
}

fn follow(head_positions: &Vec<Position>) -> Vec<Position> {
    let mut tail_pos = Position::new();
    let mut tail_positions: Vec<Position> = Vec::new();
    for (i, head_pos) in head_positions.iter().enumerate() {
        // if we are close enough to the head, do not move the tail
        if head_pos.within_radius(&tail_pos, 1) {
            tail_positions.push(tail_pos)
        } else {
            tail_pos.set(&head_positions[i - 1]);
            tail_positions.push(tail_pos);
        }
    }
    return tail_positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(6503));
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(), Some(574080));
    // }
}
