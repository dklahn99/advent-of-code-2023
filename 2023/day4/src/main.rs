use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

const INPUT_FILE: &str = "src/input.txt";

fn main() {
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read the file");

    let mut part1_score: i32 = 0;
    let mut part2_count: i32 = 0;
    let mut copy_stack: VecDeque<i32> = VecDeque::from([1; 206]);

    for line in contents.split("\n") {

        let winning_nums: HashSet<i32> = HashSet::from_iter(
            line[10..39]
                .split(" ")
                .filter_map(|x| x.parse::<i32>().ok())
        );
        let have_nums: HashSet<i32> = HashSet::from_iter(
            line[42..]
                .split(" ")
                .filter_map(|x| x.parse::<i32>().ok())
        );

        let num_in_both = winning_nums.intersection(&have_nums).count() as i32;

        if num_in_both > 0 {
            part1_score += 2_i32.pow((num_in_both - 1) as u32);
        }

        let copies = copy_stack.pop_front().unwrap();
        part2_count += copies;

        copy_stack.range_mut(..num_in_both as usize).for_each(|i| *i += copies);
    }

    println!("Part 1: {:?}", part1_score);
    println!("Part 2: {:?}", part2_count);
}
