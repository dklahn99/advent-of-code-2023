fn num_ways_to_win(time: u64, distance_record: u64) -> u64 {
    let time_f = time as f64;
    let distance_record_f = distance_record as f64;

    let lower_bound =
        (-0.5 * (-time_f + (time_f.powf(2.0) - 4.0 * distance_record_f).sqrt())).ceil() as u64;
    let upper_bound =
        (-0.5 * (-time_f - (time_f.powf(2.0) - 4.0 * distance_record_f).sqrt())).floor() as u64;

    return upper_bound - lower_bound + 1;
}

fn num_ways_to_win_2(time: u64, distance_record: u64) -> usize {
    let beats_record: Vec<bool> = (0..time)
        .map(|bt| bt * (time - bt))
        .map(|d| d > distance_record)
        .collect();

    let lower_bound = beats_record.iter().position(|&x| x).unwrap();
    let upper_bound = beats_record.iter().rposition(|&x| x).unwrap();

    return upper_bound - lower_bound + 1;
}

fn main() {
    let x1 = num_ways_to_win(61, 643);
    let x2 = num_ways_to_win(70, 1184);
    let x3 = num_ways_to_win(90, 1362);
    let x4 = num_ways_to_win(66, 1041);
    println!("Part 1: {:?}", x1 * x2 * x3 * x4);

    println!("Part 2: {:?}", num_ways_to_win(61709066, 643118413621041));
}
