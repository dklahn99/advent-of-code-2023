use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
struct RangeMapRule {
    src_start: i64,
    dest_start: i64,
    len: i64,
}

impl RangeMapRule {
    fn from_string(s: &str) -> RangeMapRule {
        let sub_strs = s.split(" ");
        let as_ints: Vec<i64> = sub_strs
            .map(|s| s.parse::<i64>().expect("Error parsing ints"))
            .collect();

        assert!(as_ints.len() == 3);

        return RangeMapRule {
            src_start: as_ints[1],
            dest_start: as_ints[0],
            len: as_ints[2],
        };
    }

    fn contains(&self, i: i64) -> bool {
        let upper_bound = self.src_start + self.len;
        if self.src_start <= i && i < upper_bound {
            return true;
        }
        return false;
    }

    fn map(&self, i: i64) -> i64 {
        assert!(self.contains(i));
        let offset = i - self.src_start;
        return self.dest_start + offset;
    }
}

struct RangeMap {
    rules: HashSet<RangeMapRule>,
}

impl FromIterator<RangeMapRule> for RangeMap {
    fn from_iter<I: IntoIterator<Item = RangeMapRule>>(iter: I) -> Self {
        return RangeMap {
            rules: HashSet::from_iter(iter),
        };
    }
}

impl RangeMap {
    fn map(&self, i: i64) -> i64 {
        for rule in &self.rules {
            if rule.contains(i) {
                return rule.map(i);
            }
        }
        return i;
    }
}

const INPUT_FILE: &str = "src/input.txt";

fn parse_maps(lines: Vec<&str>) -> HashMap<&str, RangeMap> {
    let mut output: HashMap<&str, RangeMap> = HashMap::new();

    let mut map_name = "";
    let mut map_rules = HashSet::<RangeMapRule>::new();
    for line in lines {
        if line.len() == 0 {
            println!("Adding map {:?}", map_name);
            output.insert(map_name, RangeMap::from_iter(map_rules));
            map_name = "";
            map_rules = HashSet::<RangeMapRule>::new();
        } else if line.contains(":") {
            map_name = line.split(" ").collect::<Vec<&str>>()[0];
        } else {
            map_rules.insert(RangeMapRule::from_string(line));
        }
    }

    return output;
}

fn chain_lookup(i: i64, maps: &HashMap<&str, RangeMap>, sequence: &[&str]) -> i64 {
    if sequence.len() == 0 {
        return i;
    }

    let map_name = sequence[0];
    return chain_lookup(maps[map_name].map(i), maps, &sequence[1..]);
}

fn main() {
    let contents: String = fs::read_to_string(INPUT_FILE).expect("Unable to read the file");

    // Parse out seeds line
    let lines: Vec<&str> = contents.split("\n").collect::<Vec<_>>();
    let seed_nums = lines[0][7..]
        .split(" ")
        .map(|s| s.parse::<i64>().expect("Error parsing int"));

    // Parse out list of split maps
    let maps = parse_maps(Vec::<&str>::from(&lines[2..]));

    // Traverse maps
    let map_sequence = Vec::from([
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]);

    let locations = seed_nums.map(|i| chain_lookup(i, &maps, &map_sequence));
    println!("Part 1: min location {:?}", locations.min().unwrap());

    // println!("{}", contents);
}
