use num::integer::lcm;
use std::{collections::HashMap, fs::read_to_string, path::Path};

struct Network {
    instructions: String,
    end_states: Vec<String>, // accepted ending nodes
    c: String,               // current node
    r_nodes: HashMap<String, String>,
    l_nodes: HashMap<String, String>,
}

impl Network {
    fn new(start_state: String, end_states: Vec<String>, instructions: String) -> Network {
        Network {
            instructions,
            end_states,
            c: start_state, // starting states is the initial current state
            r_nodes: HashMap::new(),
            l_nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, c: String, r: String, l: String) {
        self.r_nodes.insert(c.clone(), r);
        self.l_nodes.insert(c.clone(), l);
    }

    /// run the network until the end (from state 'AAA' to 'ZZZ')
    /// and return the number of steps taken
    fn run(&mut self) -> usize {
        for pc in 0.. {
            let i = pc % self.instructions.len(); // current instruction

            match self.instructions.chars().nth(i) {
                Some('R') => {
                    self.c = self.r_nodes.get(&self.c).unwrap().clone();
                }
                Some('L') => {
                    self.c = self.l_nodes.get(&self.c).unwrap().clone();
                }
                _ => {
                    panic!("Invalid instruction");
                }
            }

            if self.end_states.contains(&self.c) {
                return pc + 1;
            }
        }
        unreachable!()
    }
}

fn parse_line(input: &str) -> (String, String, String) {
    let mut parts = input.split(" = ");
    let c = parts.next().unwrap().to_string();
    let rl = parts.next().unwrap();

    let mut rl_parts = rl.split(", ");
    let l = rl_parts.next().unwrap().replace("(", "").replace(")", "");
    let r = rl_parts.next().unwrap().replace("(", "").replace(")", "");

    return (c, r, l);
}

pub fn part_1() -> Option<u64> {
    let instructions_path: &Path = Path::new("./src/inputs/input8/instructions.txt");
    let lr_map_path: &Path = Path::new("./src/inputs/input8/lr_map.txt");

    let instructions: String = read_to_string(instructions_path).expect("Error reading file");
    let lr_map: String = read_to_string(lr_map_path).expect("Error reading file");

    let start_state = "AAA".to_string();
    let end_states = vec!["ZZZ".to_string()];
    let mut network = Network::new(start_state, end_states, instructions);

    lr_map.lines().into_iter().for_each(|line| {
        let (c, r, l) = parse_line(line);
        network.add_node(c, r, l);
    });

    let steps = network.run();
    Some(steps as u64)
}

pub fn part_2() -> Option<u64> {
    let instructions_path: &Path = Path::new("./src/inputs/input8/instructions.txt");
    let lr_map_path: &Path = Path::new("./src/inputs/input8/lr_map.txt");

    let instructions: String = read_to_string(instructions_path).expect("Error reading file");
    let lr_map: String = read_to_string(lr_map_path).expect("Error reading file");

    let start_states: Vec<String> = ["DVA", "JQA", "PTA", "CRA", "AAA", "BGA"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let end_states: Vec<String> = ["QVZ", "QGZ", "JXZ", "TQZ", "ZZZ", "QTZ"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut period: Vec<u64> = Vec::new();
    for start_state in start_states {
        let mut network = Network::new(start_state, end_states.clone(), instructions.clone());
        lr_map.lines().into_iter().for_each(|line| {
            let (c, r, l) = parse_line(line);
            network.add_node(c, r, l);
        });

        period.push(network.run() as u64);
    }

    let lcm_period: u64 = period
        .iter()
        .fold(1, |acc, x| lcm(acc, (*x).try_into().unwrap()));

    Some(lcm_period)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(13939), part_1());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(8906539031197), part_2());
    }
}
