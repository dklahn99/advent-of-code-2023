use std::collections::HashMap;
use std::fs;

fn parse_line(line: &str) -> (&str, &str, &str) {
    let node = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];

    return (node, left, right);
}

fn step<'a>(current: &'a str, dir: char, map: &'a HashMap<&str, (&str, &str)>) -> &'a str {
    if dir == 'L' {
        return map[&current].0;
    }
    return map[&current].1;
}

fn compute_path_to_z<'a>(
    start_node: &'a str,
    sequence: &Vec<char>,
    map: &'a HashMap<&str, (&str, &str)>,
) -> Vec<&'a str> {
    let mut sequence_index: usize = 0;
    let mut current_node: &str = start_node;
    let mut path: Vec<&str> = vec![{ current_node }];
    while current_node.chars().last().expect("") != 'Z' {
        let dir = sequence[sequence_index];
        current_node = step(current_node, dir, map);
        path.push(current_node);
        sequence_index = (sequence_index + 1) % sequence.len();
    }

    return path;
}

fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Unable to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let sequence: Vec<char> = lines[0].chars().collect();
    println!("Sequence len: {}", sequence.len());

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines[2..] {
        let (node, left, right) = parse_line(line);
        map.insert(node, (left, right));
    }

    // Part 1
    println!(
        "Part 1: {}",
        compute_path_to_z("AAA", &sequence, &map).len() - 1
    );

    // Part 2
    let start_nodes: Vec<&str> = vec!["MTA", "QNA", "XCA", "BXA", "AAA", "VCA"];
    // let start_nodes: Vec<&str> = vec!["11A"];
    let lengths: Vec<usize> = start_nodes
        .iter()
        .map(|sn| compute_path_to_z(sn, &sequence, &map).len() - 1)
        .collect();
    println!("Part 2: LCM of {:?}", lengths);
}
