use std::fs;

const INPUT_FILE: &str = "src/input.txt";

#[derive(Debug)]
struct CubeNums {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct Game {
    num: i32,
    rounds: Vec<CubeNums>,
}

fn parse_round(s: &str) -> CubeNums {
    // Takes a string of the form "x red, y green, z blue"
    // may be any subset in any order

    let splits = s.split(", ");

    let mut round = CubeNums {red: 0, green: 0, blue: 0};

    for split in splits {
        let dice_info: Vec<&str> = split.split(" ").collect();
        let num = dice_info.get(0).unwrap().parse::<i32>().unwrap();

        match *dice_info.get(1).unwrap() {
            "red" => round.red = num,
            "green" => round.green = num,
            "blue" => round.blue = num,
            &_ => panic!("unknown color"),
        }
    }

    return round;
}

fn parse_game(s: &str) -> Game {

    let colon_split: Vec<&str> = s.split(": ").collect();

    let game_num = colon_split.get(0).unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
    let rounds: Vec<CubeNums> = colon_split.get(1).unwrap().split("; ").map(|s| parse_round(s)).collect();

    return Game{
        num: game_num,
        rounds: rounds,
    };
}

fn game_possible(game: Game, cube_nums: &CubeNums) -> Option<i32> {

    for round in game.rounds {

        if round.red > cube_nums.red { return None };
        if round.green > cube_nums.green { return None };
        if round.blue > cube_nums.blue { return None };
    }

    return Some(game.num);
}

fn min_cube_counts(game: &Game) -> CubeNums {
    return CubeNums {
        red: game.rounds.iter().map(|r| r.red).max().unwrap(),
        green: game.rounds.iter().map(|r| r.green).max().unwrap(),
        blue: game.rounds.iter().map(|r| r.blue).max().unwrap()
    }
}

fn power(nums: Vec<i32>) -> i32 {
    return nums.into_iter().reduce(|acc, e| acc * e).unwrap();
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read the file");
    let games = contents.split("\n").map(|s| parse_game(s));

    let cube_nums = CubeNums {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result: i32 = games.map(|g| min_cube_counts(&g)).map(|c| power(Vec::from([c.red, c.green, c.blue]))).sum();
    println!("{:#?}", result);
}
