const TIME: [u64; 4] = [62, 73, 75, 65];
const DISTANCE: [u64; 4] = [644, 1023, 1240, 1023];

pub fn part_1() -> Option<u64> {
    let mul: u64 = (0..4)
        .into_iter()
        .map(|i| wins(TIME[i], DISTANCE[i]))
        .fold(1, |acc, x| acc * x);

    Some(mul)
}

pub fn part_2() -> Option<u64> {
    let time = 62737565;
    let distance = 644102312401023;

    Some(wins(time, distance))
}

fn distance(hold_t: u64, total_t: u64) -> u64 {
    let speed = hold_t;
    let time = total_t - hold_t;
    speed * time
}

/// Calculate the number of wins for a given total time and winning distance
fn wins(total_t: u64, win_d: u64) -> u64 {
    // TODO: use the quadratic formula to solve for the time
    (0..total_t)
        .into_iter()
        .map(|hold_t| distance(hold_t, total_t))
        .filter(|d| d > &win_d)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(393120), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(36872656), part_2());
    }
}
