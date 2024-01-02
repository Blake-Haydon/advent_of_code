use crate::disjoint_set::DisjointSet;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::path::Path;
use std::{cmp, fs, hash};

struct Edge<T> {
    from: T,
    to: T,
}

impl Debug for Edge<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "{}->{}", self.from, self.to);
    }
}

struct Graph<T> {
    edges: Vec<Edge<T>>,
}

impl<T: Hash + Eq> Graph<T> {
    fn new() -> Self {
        Graph { edges: vec![] }
    }

    fn add_edge(&mut self, edge: Edge<T>) {
        self.edges.push(edge);
    }

    fn unique_vertices(&self) -> HashSet<&T> {
        let mut set = HashSet::new();
        for edge in &self.edges {
            set.insert(&edge.from);
            set.insert(&edge.to);
        }

        set
    }
}

pub fn part_1() -> Option<u32> {
    let mut set = DisjointSet::new(5);

    // println!("set = {:?}", set);
    set.union(0, 1);
    // println!("set = {:?}", set);
    set.union(0, 2);
    // println!("set = {:?}", set);
    set.union(1, 3);
    set.union(3, 3);
    set.union(1, 4);

    // println!("set = {:?}", set.find(0));
    // println!("set = {:?}", set.find(1));
    // println!("set = {:?}", set.find(2));
    // println!("set = {:?}", set.find(3));
    // println!("set = {:?}", set.find(4));

    // println!("set = {:?}", set);

    // println!("set length = {:?}", set.num_sets());
    None
}

pub fn part_2() -> Option<u32> {
    let input_path: &Path = Path::new("./src/inputs/input25.txt");
    let input: String = fs::read_to_string(input_path).expect("Error reading file");

    // create graph
    let mut g: Graph<String> = Graph::new();

    // add edges to graph
    input
        .lines()
        .map(parse_line)
        .flatten()
        .for_each(|e| g.add_edge(e));

    // get unique vertices
    let vertices = g.unique_vertices();
    // println!("vertices = {:?}", vertices);

    // create disjoint set
    let mut set = DisjointSet::new(g.edges.len() * 2);

    for i in 0..g.edges.len() {
        set.union(i, i + 1);
    }

    // find number of disjoint sets
    let num_sets = set.num_sets();

    None
}

fn parse_line(input: &str) -> Vec<Edge<String>> {
    let input = input.replace(":", "");
    let input_vec: Vec<&str> = input.split(' ').collect();

    let mut edges = vec![];
    for i in 1..input_vec.len() {
        edges.push(Edge {
            from: input_vec[0].to_string(),
            to: input_vec[i].to_string(),
        });
    }
    return edges;
}
