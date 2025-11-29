use std::collections::HashMap;
use std::fs;

fn parse_line(line: &str) -> (&str, &str, &str) {
    let node = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];

    return (node, left, right);
}

fn part1(sequence: &str, map: &HashMap<&str, (&str, &str)>) {
    let mut sequence_index: usize = 0;
    let mut current_node: &str = "AAA";
    let mut num_steps: u32 = 0;
    while current_node != "ZZZ" {
        if sequence.chars().nth(sequence_index).expect("OOB") == 'R' {
            current_node = map[&current_node].1;
        } else {
            current_node = map[&current_node].0;
        }

        sequence_index = (sequence_index + 1) % sequence.chars().count();
        num_steps += 1;
    }

    println!("Part 1: Num Steps: {}", num_steps);
}

fn step<'a>(current: &'a str, dir: char, map: &'a HashMap<&str, (&str, &str)>) -> &'a str {
    if dir == 'L' {
        return map[&current].0;
    }
    return map[&current].1;
}

#[derive(Debug)]
struct Cycle<'a> {
    nodes: Vec<(&'a str, usize)>,
    loop_start: usize,
}

fn find_loop<'a>(
    start_node: &'a str,
    sequence: &Vec<char>,
    map: &'a HashMap<&str, (&str, &str)>,
) -> Cycle<'a> {
    let mut current_node = (start_node, 0);
    let mut already_seen: HashMap<(&str, usize), usize> = HashMap::new();
    let mut nodes_visited: Vec<(&str, usize)> = Vec::new();
    let mut sequence_index: usize = 0;
    loop {
        nodes_visited.push(current_node);
        if already_seen.contains_key(&current_node) {
            break;
        }
        already_seen.insert(current_node, nodes_visited.len() - 1);

        let dir = sequence[sequence_index];
        current_node = (step(current_node.0, dir, map), sequence_index);
        sequence_index = (sequence_index + 1) % sequence.len()
    }

    let loop_start = already_seen[nodes_visited.last().expect("")];
    nodes_visited.pop();
    return Cycle {
        nodes: nodes_visited,
        loop_start,
    };
}

fn part2(sequence: Vec<char>, map: &HashMap<&str, (&str, &str)>) {
    let start_nodes: Vec<&str> = vec!["MTA", "QNA", "XCA", "BXA", "AAA", "VCA"];
    // let end_nodes: Vec<&str> = vec!["BLZ", "NPZ", "BJZ", "PBZ", "ZZZ", "PMZ"];

    let mut loops: Vec<Cycle> = Vec::new();
    for sn in start_nodes {
        let l = find_loop(sn, &sequence, map);
        println!(
            "loop start index: {}\t sequence len: {}",
            l.loop_start,
            l.nodes.len()
        );
        loops.push(l);
    }
    // println!("Cycle start index: {}", loops[0].loop_start);
    // for node in &loops[0].nodes {
    //     println!("{:?}", node);
    // }

    // let mut sequence_index: usize = 0;
    // let mut num_steps: u32 = 0;
    // while !current_nodes
    //     .iter()
    //     .map(|x| end_nodes.contains(x))
    //     .reduce(|acc, e| acc && e)
    //     .unwrap()
    // {
    //     if num_steps % 100000 == 0 {
    //         println!("Steps: {} Current nodes: {:?}", num_steps, current_nodes);
    //     }
    //     let dir = sequence.chars().nth(sequence_index).expect("OOB");
    //     for i in 0..current_nodes.len() {
    //         current_nodes[i] = step(current_nodes[i], dir, map);
    //     }
    //     sequence_index = (sequence_index + 1) % sequence.chars().count();
    //     num_steps += 1;
    // }
    // println!("Part 2: Num Steps: {}", num_steps);
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

    // part1(sequence, &map);

    // let out = find_loop("AAA", sequence.chars().collect(), &map);
    part2(sequence, &map);
}
