use std::fs;

const START_CHAR: char = 'S';

fn find_start(lines: &Vec<&str>) -> Option<(usize, usize)> {
    for (row, l) in lines.iter().enumerate() {
        if let Some(col) = l.chars().position(|s| s == START_CHAR) {
            return Some((row, col));
        }
    }
    None
}

fn main() {
    let contents: String = fs::read_to_string("src/test.txt").expect("Unable to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let start_pos = find_start(&lines);
    println!("{:?}", start_pos);
}
