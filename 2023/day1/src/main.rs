use std::fs;

const INPUT_FILE: &str = "src/input.txt";

const INT_STR_MAP: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_digit_substrs(s: &String) -> Vec<(usize, i32)> {

    let mut digits: Vec<(usize, i32)> = Vec::new();
    
    // Search for digits
    for i in (0..10) {
        let int_str = i.to_string();
        let occurences = s.match_indices(int_str.as_str());

        let mut d = occurences.map(|occ| (occ.0, occ.1.parse::<i32>().expect(""))).collect::<Vec<(usize, i32)>>();

        digits.append(&mut d);
    }

    // Search for textual numbers
    for (i, int_str) in INT_STR_MAP.iter().enumerate() {

        let occurences = s.match_indices(int_str);
        let mut d = occurences.map(|occ| (occ.0, i as i32)).collect::<Vec<(usize, i32)>>();

        digits.append(&mut d);
    }

    return digits;
}

fn parse_line(s: &String) -> i32 {

    let mut digits = find_digit_substrs(s);
    digits.sort_by_key(|x| x.0);

    let digit_str = digits.iter().map(|x| x.1.to_string()).collect::<String>();

    let first_and_last = format!("{}{}", 
        digit_str.chars().nth(0).unwrap(),
        digit_str.chars().nth(digit_str.len() - 1).unwrap(),
    );

    return first_and_last.parse::<i32>().unwrap();
}


fn main() {

    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read the file");

    let sum: i32 = contents.split("\n").map(|s| parse_line(&s.to_string())).sum();

    println!("Result: {:?}", sum);

}
