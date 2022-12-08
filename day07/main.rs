// TODO: clean this mess up

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

fn print(arena: &Arena, this_node: &Node, level: usize) -> String {
    let mut str = format!(
        "{}├─ {} {}\n",
        "│ ".repeat(level),
        this_node.name,
        this_node.size.unwrap_or(0)
    );
    for node_id in this_node.children.iter() {
        let node = arena.get(*node_id);
        str.push_str(&print(arena, node, level + 1));
    }
    str
}

impl std::fmt::Display for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = &self.nodes[0];
        write!(f, "{}", print(self, root, 0))
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

fn sizes(arena: &mut Arena, node_id: NodeId) -> usize {
    let node = arena.get(node_id);
    let node_size = node.size;
    let children = node.children.clone();
    let size = match node_size {
        Some(existing_size) => existing_size,
        None => children.iter().map(|&node_id| sizes(arena, node_id)).sum(),
    };
    let mut node = arena.get_mut(node_id);
    node.size = Some(size);
    size
}

pub fn main() {
    let input = include_str!("./input.txt").lines().skip(1);
    let mut nodes = Arena::new();

    let root = Node {
        parent: None,
        children: vec![],
        name: "/".to_string(),
        size: None,
        kind: Kind::Directory,
    };
    let mut current_node_id = nodes.push(root);

    for line in input {
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
                            current_node_id = nodes
                                .get_mut(current_node_id)
                                .parent
                                .expect("valid cd .. command");
                            continue;
                        }

                        let current_node = nodes.get(current_node_id);
                        let dir_node_id = nodes
                            .find_child(current_node, change_dir_name)
                            .expect("unknown dir");
                        current_node_id = dir_node_id;
                    }
                    _whatever => {} // skip other commands
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
                let new_node_id = nodes.push(new_node);
                let current_node = nodes.get_mut(current_node_id);
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
                let new_node_id = nodes.push(new_node);
                let current_node = nodes.get_mut(current_node_id);
                current_node.children.push(new_node_id);
            }
        }
    }

    println!("{}", sizes(&mut nodes, 0));
    println!("{}", nodes);

    let small_total = nodes
        .nodes
        .iter()
        .filter(|node|{ node.kind == Kind::Directory })
        .map(|node| node.size.expect("computed sizes"))
        .filter(|&size| size <= 100_000)
        .sum::<usize>();
    println!("day06a: {small_total}");

    let unused_space = 70_000_000 - nodes.get(0).size.expect("already computed");
    let free_at_least = 30_000_000 - unused_space;
    let smallest_to_delete = nodes
        .nodes
        .iter()
        .filter(|node|{ node.kind == Kind::Directory })
        .map(|node| node.size.expect("computed sizes"))
        .filter(|&size| size >= free_at_least)
        .min()
        .expect("smallest to delete exists");
    println!("day06b: {smallest_to_delete}");
}
