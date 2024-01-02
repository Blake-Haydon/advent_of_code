use std::{cmp::min, collections::VecDeque, path::Path};

pub fn part_1() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input12.txt");
    let input: String = std::fs::read_to_string(input_path).expect("Error reading file");

    let sum = input.lines().map(|line| part_1_instance(line)).sum();

    // let sum = part_1_instance("?###???????? 3,2,1");

    Some(sum)
}

pub fn part_2() -> Option<u32> {
    let x = part_2_instance("????.#...#... 4,1,1");
    println!("{:?}", x);
    None
}

fn part_1_instance(line: &str) -> u32 {
    let input_group = ItemGroup::parse(line);
    let groups = gen_valid_groups(input_group);

    // count how many groups satisfy the criteria
    groups
        .clone()
        .into_iter()
        .filter(|x| x.satisfies_criteria())
        .count() as u32
}

fn part_2_instance(line: &str) -> u32 {
    let input_group = ItemGroup::parse(line);
    let unfold_input_group = unfold(input_group);
    println!("{:?}", unfold_input_group);
    unfold_input_group.print_items();
    let groups: Vec<ItemGroup> = gen_valid_groups(unfold_input_group);

    // count how many groups satisfy the criteria
    groups
        .clone()
        .into_iter()
        .filter(|x| x.satisfies_criteria())
        .count() as u32
}

fn gen_valid_groups(input_group: ItemGroup) -> Vec<ItemGroup> {
    // generate all possible groups
    let input_group_len = input_group.items.len();
    let mut groups = VecDeque::from(vec![input_group]);
    for i in 0..input_group_len {
        println!("i: {} out of {}", i, input_group_len);
        for group in groups.clone() {
            if group.items[i] == Item::Unknown {
                let mut new_group_0 = group.clone();
                let mut new_group_1 = group.clone();

                new_group_0.items[i] = Item::NotSpring;
                new_group_1.items[i] = Item::Spring;

                groups.pop_front();

                if new_group_0.satisfies_partial_criteria() {
                    groups.push_back(new_group_0);
                }

                if new_group_1.satisfies_partial_criteria() {
                    groups.push_back(new_group_1);
                }
            }
        }
    }

    return groups.into();
}

fn unfold(group: ItemGroup) -> ItemGroup {
    let mut new_group = group.clone();

    // push a new unknown item
    new_group.items.push(Item::Unknown);

    // 5 includes the original group
    for _ in 0..3 {
        for item in group.items.clone() {
            new_group.items.push(item);
        }

        // push a new unknown item
        new_group.items.push(Item::Unknown);

        for i in 0..group.criteria.len() {
            new_group.criteria.push(group.criteria[i]);
        }
    }

    for item in group.items.clone() {
        new_group.items.push(item);
    }

    for i in 0..group.criteria.len() {
        new_group.criteria.push(group.criteria[i]);
    }

    return new_group;
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Unknown,
    Spring,
    NotSpring,
}

#[derive(Debug, Clone)]
struct ItemGroup {
    items: Vec<Item>,
    criteria: Vec<u32>,
}

impl Item {
    fn parse(input: char) -> Item {
        match input {
            '?' => Item::Unknown,
            '.' => Item::NotSpring,
            '#' => Item::Spring,
            _ => panic!("Invalid char"),
        }
    }

    fn print(&self) {
        match self {
            Item::Unknown => print!("?"),
            Item::Spring => print!("#"),
            Item::NotSpring => print!("."),
        }
    }
}

impl ItemGroup {
    fn parse(input: &str) -> ItemGroup {
        let mut split = input.split(" ");
        let items = split.next().unwrap().chars().map(Item::parse).collect();
        let criteria = split
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        ItemGroup { items, criteria }
    }

    fn print_items(&self) {
        for item in &self.items {
            item.print();
        }
        println!();
    }

    /// convert `self.items` to a criteria vector
    fn to_criteria(&self) -> Vec<u32> {
        self.items
            .split(|&x| x == Item::NotSpring)
            .filter(|x| x.len() > 0)
            .map(|x| x.len() as u32)
            .collect()
    }

    /// check if the items in the group satisfy the criteria
    fn satisfies_criteria(&self) -> bool {
        self.to_criteria() == self.criteria
    }

    fn satisfies_partial_criteria(&self) -> bool {
        let mut group = self.clone();

        let mut split = group.items.split(|&x| x == Item::Unknown);
        group.items = split.next().unwrap().to_vec();

        let own_criteria = group.to_criteria();

        let own_criteria_sum = own_criteria.iter().sum::<u32>();
        let criteria_sum = self.criteria.iter().sum::<u32>();

        if own_criteria_sum > criteria_sum {
            return false;
        }

        let min_criteria_len = min(own_criteria.len(), self.criteria.len());

        for i in 0..min_criteria_len {
            if own_criteria[i] > self.criteria[i] {
                return false;
            }
        }
        return true;
    }
}

// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_1() {
//         assert_eq!(Some(7017), part_1());
//     }
// }
