use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::sync::atomic::AtomicI32;
use core::sync::atomic::Ordering;
use regex::Regex;


const INPUT_FILE: &str = "src/input.txt";

static COUNTER: AtomicI32 = AtomicI32::new(1);
fn get_id() -> i32 { COUNTER.fetch_add(1, Ordering::Relaxed) }

fn get_all_substrings<'a>(s: &'a str, pattern: &str) -> Vec::<(i32, &'a str)>{
    // Returns vector of (start_idx, substr) tuples

    let re = Regex::new(pattern).unwrap();
    return re
        .find_iter(s)
        .map(|m| (m.start() as i32, m.as_str(),))
        .collect();
}

fn get_adj<T>(pos: (i32, i32), graph: &HashMap<(i32, i32), T>) -> Vec<&T> {

    let adj_positions: [(i32, i32); 8]= [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 01), (1, 1),
    ];

    return adj_positions
        .iter()
        .filter_map(|x| graph.get(&(pos.0 + x.0, pos.1 + x.1)))
        .collect();
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read the file");

    // Coords are (row, col)
    let mut symbol_graph: HashMap<(i32, i32), &str> = HashMap::new();
    let mut num_graph: HashMap<(i32, i32), i32> = HashMap::new();  // Map from location to unique ID
    let mut id_map: HashMap<i32, i32> = HashMap::new();  // Map from unique ID to corresponding integer

    for (row, line) in contents.split("\n").enumerate() {

        let all_ints: Vec<(i32, &str)> = get_all_substrings(line, r"\d+");
        for (col, num) in all_ints {
            let as_int: i32 = num.parse().unwrap();
            let id = get_id();

            id_map.insert(id, as_int);
            num
                .chars()
                .enumerate()
                .for_each(|(i, _)| {num_graph.insert((row as i32, col + i as i32), id);});
        }

        let all_symbols: Vec<(i32, &str)> = get_all_substrings(line, r"[^\d\n\.]");
        all_symbols
            .iter()
            .for_each(|e| {symbol_graph.insert((row as i32, e.0), e.1);});
    }

    let mut ids_adj_to_syms: HashSet<i32> = HashSet::new();

    for (pos, sym) in symbol_graph {
        get_adj::<i32>(pos, &num_graph)
            .iter()
            .for_each(|x| {ids_adj_to_syms.insert(**x);});
    }

    let sum: i32 = ids_adj_to_syms
                            .into_iter()
                            .map(|id| id_map.get(&id).unwrap())
                            .sum();

    println!("Sum: {:?}",sum);
}
