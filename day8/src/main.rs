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

fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Unable to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let sequence: &str = lines[0];

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines[2..] {
        let (node, left, right) = parse_line(line);
        map.insert(node, (left, right));
    }

    part1(sequence, &map);
}
