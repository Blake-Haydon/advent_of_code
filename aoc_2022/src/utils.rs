use crate::{
    challenge_01, challenge_02, challenge_03, challenge_04, challenge_05, challenge_06,
    challenge_07, challenge_08, challenge_09, challenge_25,
};
use std::fmt::Display;

pub struct Challenge<T> {
    pub id: u32,
    pub part_1: fn() -> Option<T>,
    pub part_2: fn() -> Option<T>,
}

impl<T: Display> Challenge<T> {
    pub fn run(&self) {
        let p1 = (self.part_1)();
        let p2 = (self.part_2)();

        // Print results from this challenge
        println!("Challenge {}", self.id);
        match p1 {
            Some(p1) => println!("  Part 1: ✅ {}", p1),
            None => println!("  Part 1: ❌"),
        }
        match p2 {
            Some(p2) => println!("  Part 2: ✅ {}", p2),
            None => println!("  Part 2: ❌"),
        }
    }
}

pub fn run_all_challenges() {
    Challenge {
        id: 1,
        part_1: challenge_01::part_1,
        part_2: challenge_01::part_2,
    }
    .run();

    Challenge {
        id: 2,
        part_1: challenge_02::part_1,
        part_2: challenge_02::part_2,
    }
    .run();

    Challenge {
        id: 3,
        part_1: challenge_03::part_1,
        part_2: challenge_03::part_2,
    }
    .run();

    Challenge {
        id: 4,
        part_1: challenge_04::part_1,
        part_2: challenge_04::part_2,
    }
    .run();

    Challenge {
        id: 5,
        part_1: challenge_05::part_1,
        part_2: challenge_05::part_2,
    }
    .run();

    Challenge {
        id: 6,
        part_1: challenge_06::part_1,
        part_2: challenge_06::part_2,
    }
    .run();

    Challenge {
        id: 7,
        part_1: challenge_07::part_1,
        part_2: challenge_07::part_2,
    }
    .run();

    Challenge {
        id: 8,
        part_1: challenge_08::part_1,
        part_2: challenge_08::part_2,
    }
    .run();

    Challenge {
        id: 9,
        part_1: challenge_09::part_1,
        part_2: challenge_09::part_2,
    }
    .run();

    Challenge {
        id: 25,
        part_1: challenge_25::part_1,
        part_2: challenge_25::part_2,
    }
    .run();
}
