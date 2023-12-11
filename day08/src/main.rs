use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let (dir_str, nodes_str) = input.split_once("\n\n").unwrap();

    let dirs = dir_str
        .chars()
        .map(|c| Direction::from_char(c))
        .collect::<Vec<_>>();
    let graph = construct_graph(nodes_str);

    let part_one_res = count_steps(&graph, "AAA", &dirs);
    println!("{part_one_res}");
    let part_two_res = count_multi_steps(&graph, &dirs);
    println!("{part_two_res}");
}

enum Direction {
    L,
    R,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Unknown direction"),
        }
    }
    fn get_index(&self) -> usize {
        match self {
            Direction::L => 0,
            Direction::R => 1,
        }
    }
}

fn construct_graph(data: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    data.lines().for_each(|l| {
        let (v, n) = l.split_once("=").unwrap();
        let node = v.trim();
        let neighbors = n
            .trim()
            .strip_prefix("(")
            .and_then(|s|
                s.strip_suffix(")").and_then(|s|
                    Some(
                        s
                            .split(",")
                            .map(|n| n.trim())
                            .collect::<Vec<_>>()
                    )
                )
            )
            .unwrap();
        graph.insert(node, neighbors);
    });
    return graph;
}

fn count_steps(graph: &HashMap<&str, Vec<&str>>, start: &str, dirs: &Vec<Direction>) -> u64 {
    let mut curr = start.clone();
    let mut dir_idx = 0;
    let mut steps = 0;
    while !curr.ends_with('Z') {
        let dir = &dirs[dir_idx];
        curr = &graph.get(curr).unwrap()[dir.get_index()];
        dir_idx = if dir_idx + 1 >= dirs.len() { 0 } else { dir_idx + 1 };
        steps += 1;
    }
    return steps;
}

fn count_multi_steps(graph: &HashMap<&str, Vec<&str>>, dirs: &Vec<Direction>) -> u64 {
    let start_nodes = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|n| *n)
        .collect::<Vec<_>>();
    // we really only need to find the LCM for the steps it takes for each node to reach its first endpoint
    let node_steps = start_nodes
        .iter()
        .map(|n| count_steps(&graph, *n, &dirs))
        .collect::<Vec<_>>();

    return lcm(&node_steps);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// LCM(a,b) = ab/GCD(a,b) and LCM(a,b,c) = LCM(LCM(a,b),c), so apply this to the input array
fn lcm(numbers: &Vec<u64>) -> u64 {
    numbers.iter().fold(numbers[0], |a, &b| (a * b) / gcd(a, b))
}
