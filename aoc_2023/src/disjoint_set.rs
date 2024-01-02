use std::collections::HashSet;

#[derive(Debug)]
pub struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    /// Creates a new [`DisjointSet`].
    pub fn new(n: usize) -> Self {
        DisjointSet {
            parent: (0..n).collect(),
        }
    }

    /// find the root node of the set.
    pub fn find(&mut self, x: usize) -> usize {
        // TODO: convert this into a more efficient algorithm.
        if self.parent[x] != x {
            return self.find(self.parent[x]);
        }
        x
    }

    /// union two sets together.
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
        }
    }

    /// check if two sets are in the same set.
    /// This is the same as `find(x) == find(y)`.
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// get the number of disjoint sets.
    /// this is the same as the number of distinct roots.
    pub fn num_sets(&mut self) -> usize {
        (0..self.parent.len())
            .map(|x| self.find(x))
            .collect::<HashSet<_>>()
            .len()
    }
}

mod tests {
    use super::DisjointSet;

    #[test]
    fn test_part_1() {
        let n = 10;
        let d = DisjointSet::new(n);
        println!("d = {:?}", d);
    }
}
