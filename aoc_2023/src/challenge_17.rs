pub fn part_1() -> Option<u32> {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let up_moves = vec![Move::Up, Move::Up, Move::Up];
    let down_moves = vec![Move::Down, Move::Down, Move::Down];

    None
}

pub fn part_2() -> Option<u32> {
    None
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}
