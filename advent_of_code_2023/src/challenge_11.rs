use num::abs;
use std::{collections::HashSet, path::Path};

pub fn part_1() -> Option<i64> {
    let input_path: &Path = Path::new("./src/inputs/input11.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let coords = expand(parse(&input), 2);

    let mut dist_sum = 0;
    for c0 in &coords {
        for c1 in &coords {
            dist_sum += min_dist(&c0, &c1);
        }
    }

    // halve to account for double counting
    Some(dist_sum / 2)
}

pub fn part_2() -> Option<i64> {
    let input_path: &Path = Path::new("./src/inputs/input11.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let coords = expand(parse(&input), 1000000);

    let mut dist_sum = 0;
    for c0 in &coords {
        for c1 in &coords {
            dist_sum += min_dist(&c0, &c1);
        }
    }

    // halve to account for double counting
    Some(dist_sum / 2)
}

#[derive(Debug, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse(input: &str) -> Vec<Coord> {
    let map: Vec<_> = input.split('\n').collect();
    let mut coords: Vec<Coord> = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i].chars().nth(j).unwrap() == '#' {
                coords.push(Coord {
                    x: j as i64,
                    y: i as i64,
                });
            }
        }
    }

    coords
}

fn min_dist(c0: &Coord, c1: &Coord) -> i64 {
    abs(c0.x - c1.x) + abs(c0.y - c1.y)
}

/// The top left corner be (0, 0)
/// If the expansion factor is 2 then every empty space will expand by 2x
fn expand(c: Vec<Coord>, expansion_factor: i64) -> Vec<Coord> {
    let filled_rows: HashSet<_> = c.iter().map(|c| c.y).collect();
    let filled_cols: HashSet<_> = c.iter().map(|c| c.x).collect();

    let max_row = *filled_rows.iter().max().unwrap();
    let max_col = *filled_cols.iter().max().unwrap();

    // store the offset for each current coordinate in the
    // row_offset and col_offset vectors
    let mut row_offset = vec![0];
    let mut col_offset = vec![0];

    let mut row_counter = 0;
    for i in 0..max_row {
        if !filled_rows.contains(&i) {
            row_counter += expansion_factor - 1;
        }

        row_offset.push(row_counter);
    }

    let mut col_counter = 0;
    for i in 0..max_col {
        if !filled_cols.contains(&i) {
            col_counter += expansion_factor - 1;
        }

        col_offset.push(col_counter);
    }

    // expand the coordinates
    let mut expanded_coords = vec![];
    for coord in c {
        expanded_coords.push(Coord {
            x: coord.x + col_offset[coord.x as usize],
            y: coord.y + row_offset[coord.y as usize],
        });
    }
    return expanded_coords;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(9965032), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(550358864332), part_2());
    }
}
