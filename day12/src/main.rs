
extern crate proj_self;
use std::collections::HashMap;

struct Node {
    id: u32,
    links: Vec<u32>,
}

fn str_to_node(s: &str) -> Node {
    let parts: Vec<_> = s.split(" <-> ").collect();
    let id = parts[0].parse().unwrap();
    let links: Vec<_> = parts[1].split(", ").map(|x| x.parse().unwrap()).filter(|x| *x != id).collect();
    Node { id: id, links: links }
}

fn has_path(source: u32, dest: u32, nodes: &HashMap<u32, Node>, visited: &HashMap<u32, bool>) -> bool {
    if source == dest {
        return true;
    }

    let mut v = visited.clone();
    v.insert(source, true);

    let start = &nodes[&source];
    for link in &start.links {
        if *link == dest {
            return true;
        } else if !v.contains_key(link) && has_path(*link, dest, nodes, &v) {
            return true;
        }
    }

    false
}

fn path_count(dest: u32, nodes: &HashMap<u32, Node>) -> Vec<u32> {
    let mut reachable = vec![];
    for (id, _node) in nodes.iter() {
        let visited = HashMap::new();
        if has_path(*id, dest, nodes, &visited) {
            reachable.push(*id);
        }
    }

    reachable
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);
    let nodes: Vec<Node> = lines.iter().map(|x| str_to_node(x)).collect();
    let mut map = HashMap::new();
    for n in nodes {
        map.insert(n.id, n);
    }
    let all = path_count(0, &map);
    println!("Paths to zero: {}", all.len());

    let mut groups = HashMap::new();
    let mut visited = HashMap::new();
    for (id, _node) in map.iter() {
        if !visited.contains_key(id) {
            println!("Checking paths to {}", id);
            let group = path_count(*id, &map);
            for x in &group {
                visited.insert(*x, true);
            }
            groups.insert(group, true);
        }
    }
    println!("Groups: {}", groups.len());
}

#[test]
fn test() {
    let pipes = vec![
        "0 <-> 2",
        "1 <-> 1",
        "2 <-> 0, 3, 4",
        "3 <-> 2, 4",
        "4 <-> 2, 3, 6",
        "5 <-> 6",
        "6 <-> 4, 5",
    ];
    let nodes: Vec<Node> = pipes.iter().map(|x| str_to_node(x)).collect();
    let mut map = HashMap::new();
    for n in nodes {
        map.insert(n.id, n);
    }
    let all = path_count(0, &map);
    assert_eq!(6, all.len());

    let mut groups = HashMap::new();
    for (id, _node) in map.iter() {
        let group = path_count(*id, &map);
        groups.insert(group, true);
    }
    assert_eq!(2, groups.len());
}
