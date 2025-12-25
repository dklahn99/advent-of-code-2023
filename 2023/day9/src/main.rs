use std::fs;

fn derive(input: &Vec<i64>) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::new();
    for (i, n) in input.iter().skip(1).enumerate() {
        out.push(*n - input[i]);
    }

    return out;
}

fn predict(input: &Vec<i64>) -> i64 {
    assert!(input.len() != 0);

    let dxdt = derive(input);
    if dxdt.iter().all(|&x| x == 0i64) {
        return input.iter().last().expect("") + dxdt.iter().last().expect("");
    }

    let dx = predict(&dxdt);
    return input.iter().last().expect("") + dx;
}

fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Unable to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let parsed_lines: Vec<Vec<i64>> = lines
        .iter()
        .map(|l| {
            l.split(" ")
                .map(|str| str.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut part1_predictions: Vec<i64> = Vec::new();
    let mut part2_predictions: Vec<i64> = Vec::new();
    for line in parsed_lines {
        part1_predictions.push(predict(&line));
        part2_predictions.push(predict(&line.into_iter().rev().collect()));
    }
    println!("Part 1: {}", part1_predictions.iter().sum::<i64>());
    println!("Part 2: {}", part2_predictions.iter().sum::<i64>());
}
