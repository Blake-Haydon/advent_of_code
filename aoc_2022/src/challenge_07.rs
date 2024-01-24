use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fs::read_to_string,
    path::Path,
    rc::Rc,
};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<HashMap<String, Rc<Node>>>, // can have multiple children
    parent: Option<Rc<Node>>,                     // can only be a single parent
}

impl Node {
    fn new_root(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(HashMap::new()),
            parent: None,
        })
    }

    fn new_child(parent: Rc<Node>, name: String, value: i32) -> Rc<Node> {
        let child: Rc<Node> = Rc::new(Node {
            value,
            children: RefCell::new(HashMap::new()),
            parent: Some(Rc::clone(&parent)),
        });
        parent.children.borrow_mut().insert(name, Rc::clone(&child));
        return Rc::clone(&child);
    }

    fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }

    /// returns the depth of the node
    fn depth(&self) -> i32 {
        match &self.parent {
            Some(p) => 1 + p.depth(),
            None => 0,
        }
    }

    /// Prints the tree in a directory-like format
    #[allow(dead_code)]
    fn print_tree(&self, name: &str) {
        let spaces = "  ".repeat(self.depth() as usize);

        // if we we are a leaf, print the value
        if self.is_leaf() {
            println!("{spaces}- {} (file, size={})", name, self.value);
        // if not we are a directory, print the name and total size
        } else {
            println!("{spaces}- {} (dir, size={}) ", name, self.total_size());
        }

        let c: Ref<'_, HashMap<String, Rc<Node>>> = self.children.borrow();
        let mut keys: Vec<&String> = c.keys().collect();
        keys.sort(); // sort for deterministic output

        for k in keys {
            self.to_child(k).unwrap().print_tree(k);
        }
    }

    /// returns the parent of the node
    fn to_parent(&self) -> Option<Rc<Node>> {
        match &self.parent {
            Some(p) => Some(Rc::clone(p)),
            None => None,
        }
    }

    /// returns the child of the node
    fn to_child(&self, name: &str) -> Option<Rc<Node>> {
        match self.children.borrow().get(name) {
            Some(c) => Some(Rc::clone(c)),
            None => None,
        }
    }

    /// total size of the node and all its children
    fn total_size(&self) -> i32 {
        let mut total_size = self.value;
        for child in self.children.borrow().values() {
            total_size += child.total_size();
        }
        total_size
    }

    fn get_all_children(node: Rc<Node>) -> Vec<Rc<Node>> {
        let mut nodes: Vec<Rc<Node>> = Vec::new();
        nodes.push(Rc::clone(&node));
        for child in node.children.borrow().values() {
            nodes.append(&mut Node::get_all_children(Rc::clone(&child)));
        }
        nodes
    }
}

/// Parse the file tree input and return the root node of the filesystem i.e. "/"
fn parse_fs(input: &str) -> Rc<Node> {
    let root: Rc<Node> = Node::new_root(0);

    let root_parent: Rc<Node> = Rc::clone(&root);
    let root_dir_name: String = "/".to_string();
    let root_dir_value: i32 = 0;
    let fs_root = Node::new_child(root_parent, root_dir_name, root_dir_value);

    let mut curr: Rc<Node> = Rc::clone(&root);
    for line in input.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        match split_line[0] {
            // filesystem action
            "$" => {
                match split_line[1] {
                    "cd" => {
                        // change directory: go up
                        if split_line[2] == ".." {
                            curr = curr.to_parent().unwrap();
                        // change directory: go down into child
                        } else {
                            curr = curr.to_child(split_line[2]).unwrap()
                        }
                    }
                    "ls" => {}
                    _ => {
                        unreachable!("invalid command")
                    }
                }
            }
            // directory action: create a new directory
            "dir" => {
                let name = split_line[1].to_string();
                let parent: Rc<Node> = Rc::clone(&curr);
                let value = 0;
                Node::new_child(parent, name, value);
            }
            // file action: create a new file
            _ => {
                let name = split_line[1].to_string();
                let parent: Rc<Node> = Rc::clone(&curr);
                let size = split_line[0].parse::<i32>().unwrap();
                Node::new_child(parent, name, size);
            }
        }
    }

    fs_root
}

pub fn part_1() -> Option<i32> {
    let input_path: &Path = Path::new("./src/inputs/input7.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let fs_root: Rc<Node> = parse_fs(&input);
    let mut total: i32 = 0;
    for node in Node::get_all_children(fs_root) {
        if node.is_leaf() {
            continue;
        }

        let dir_size: i32 = node.total_size();
        if dir_size < 100_000 {
            total += dir_size;
        }
    }

    Some(total)
}

pub fn part_2() -> Option<i32> {
    let input_path: &Path = Path::new("./src/inputs/input7.txt");
    let input: String = read_to_string(input_path).expect("Error reading file");

    let fs_root = parse_fs(&input);

    let total_space: i32 = 70_000_000;
    let min_unused_space: i32 = 30_000_000;
    let min_used_space: i32 = total_space - min_unused_space;
    let used_space: i32 = fs_root.total_size();

    let mut min_dir_size: i32 = total_space;
    for node in Node::get_all_children(fs_root) {
        if node.is_leaf() {
            continue;
        }

        let dir_size: i32 = node.total_size();
        let new_used_space: i32 = used_space - dir_size;
        if new_used_space < min_used_space && dir_size < min_dir_size {
            min_dir_size = dir_size;
        }
    }

    Some(min_dir_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), Some(1428881));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), Some(10475598));
    }
}
