use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

const START_CHAR: char = 'S';

type Position = (isize, isize);

fn build_pipe_map(file: &str) -> Result<HashMap<Position, Vec<Position>>, String> {
    let contents: String = fs::read_to_string(file).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut m = HashMap::<Position, Vec<Position>>::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, tile_char) in line.chars().enumerate() {
            let (row, col) = (row as isize, col as isize);
            let above = (row - 1, col);
            let below = (row + 1, col);
            let left = (row, col - 1);
            let right = (row, col + 1);
            let reachable_nodes: Vec<Position> = match tile_char {
                '|' => vec![above, below],
                '-' => vec![left, right],
                'L' => vec![above, right],
                'J' => vec![above, left],
                '7' => vec![below, left],
                'F' => vec![below, right],
                '.' => vec![],
                'S' => vec![above, below, left, right],
                default => return Err(format!("Unrecognized pipe character: {}", tile_char)),
            };

            m.insert((row, col), reachable_nodes);
        }
    }
    Ok(m)
}

fn main() {
    let pipe_map = build_pipe_map("src/test.txt");
    println!("{:?}", pipe_map);
    // println!("{:?}", Dot::with_config(&pipe_map, &[Config::EdgeNoLabel]));
}
