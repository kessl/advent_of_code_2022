struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }

    fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id]
    }

    fn find_child(&self, current_node: &Node, name: &str) -> Option<NodeId> {
        current_node
            .children
            .iter()
            .find(|&node_id| self.get(*node_id).name == name)
            .map(|opt| *opt)
    }
}

type NodeId = usize;

#[derive(Debug, PartialEq)]
enum Kind {
    File,
    Directory,
}

#[derive(Debug)]
struct Node {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    name: String,
    size: Option<usize>,
    kind: Kind,
}

fn build_tree(input: &str) -> Arena {
    let mut arena = Arena::new();

    let root = Node {
        parent: None,
        children: vec![],
        name: "/".to_string(),
        size: None,
        kind: Kind::Directory,
    };
    let mut current_node_id = arena.push(root);

    let iter = input.lines().skip(1);
    for line in iter {
        let mut tokens = line.split_whitespace();
        let first = tokens.next().expect("valid command");
        match first {
            "$" => {
                // command
                match tokens.next() {
                    Some("cd") => {
                        // assume we only cd into dirs we have seen before
                        let change_dir_name = tokens.next().expect("directory name");
                        if change_dir_name == ".." {
                            current_node_id = arena
                                .get_mut(current_node_id)
                                .parent
                                .expect("valid cd .. command");
                            continue;
                        }
                        let current_node = arena.get(current_node_id);
                        let dir_node_id = arena
                            .find_child(current_node, change_dir_name)
                            .expect("unknown dir");
                        current_node_id = dir_node_id;
                    }
                    _ => {} // skip other commands
                }
            }
            "dir" => {
                let dir_name = tokens.next().expect("directory name");
                let new_node = Node {
                    parent: Some(current_node_id),
                    children: vec![],
                    name: dir_name.to_string(),
                    size: None,
                    kind: Kind::Directory,
                };
                let new_node_id = arena.push(new_node);
                let current_node = arena.get_mut(current_node_id);
                current_node.children.push(new_node_id);
            }
            file_size => {
                // ls output - file
                let file_name = tokens.next().expect("file name");
                let new_node = Node {
                    parent: Some(current_node_id),
                    children: vec![],
                    name: file_name.to_string(),
                    size: Some(file_size.parse::<usize>().expect("numeric file size")),
                    kind: Kind::File,
                };
                let new_node_id = arena.push(new_node);
                let current_node = arena.get_mut(current_node_id);
                current_node.children.push(new_node_id);
            }
        }
    }

    arena
}

fn compute_directory_sizes(arena: &mut Arena, node_id: NodeId) -> usize {
    let node = arena.get(node_id);
    let node_size = node.size;
    let children = node.children.clone();
    let size = match node_size {
        Some(existing_size) => existing_size,
        None => children
            .iter()
            .map(|&node_id| compute_directory_sizes(arena, node_id))
            .sum(),
    };
    let mut node = arena.get_mut(node_id);
    node.size = Some(size);
    size
}

// plan: parse input and build a tree structure, then walk the tree and compute directory sizes
pub fn main() {
    let input = include_str!("./input.txt");

    // parse input into a tree
    let mut arena = build_tree(input);

    // compute directory sizes
    compute_directory_sizes(&mut arena, 0);

    let small_total = arena
        .nodes
        .iter()
        .filter(|node| node.kind == Kind::Directory)
        .map(|node| node.size.expect("computed sizes"))
        .filter(|&size| size <= 100_000)
        .sum::<usize>();
    println!("day07a: {small_total}");

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let unused_space = total_space - arena.get(0).size.expect("already computed");
    let free_at_least = needed_space - unused_space;
    let smallest_to_delete = arena
        .nodes
        .iter()
        .filter(|node| node.kind == Kind::Directory)
        .map(|node| node.size.expect("computed sizes"))
        .filter(|&size| size >= free_at_least)
        .min()
        .expect("smallest to delete exists");
    println!("day07b: {smallest_to_delete}");
}
