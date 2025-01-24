use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Range {
    /// Defines the range [start, end)
    start: i64,
    end: i64,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct RangeMapRule {
    src: Range,
    dest: Range,
}

impl RangeMapRule {
    fn from_string(s: &str) -> RangeMapRule {
        let sub_strs = s.split(" ");
        let as_ints: Vec<i64> = sub_strs
            .map(|s| s.parse::<i64>().expect("Error parsing ints"))
            .collect();

        assert!(as_ints.len() == 3);

        let len = as_ints[2];
        return RangeMapRule {
            src: Range {
                start: as_ints[1],
                end: as_ints[1] + len,
            },
            dest: Range {
                start: as_ints[0],
                end: as_ints[0] + len,
            },
        };
    }

    fn contains(&self, i: i64) -> bool {
        if self.src.start <= i && i < self.src.end {
            return true;
        }
        return false;
    }

    fn map(&self, i: i64) -> i64 {
        assert!(self.contains(i));
        let offset = i - self.src.start;
        return self.dest.start + offset;
    }
}

struct RangeMap {
    rules: Vec<RangeMapRule>,
}

impl FromIterator<RangeMapRule> for RangeMap {
    fn from_iter<I: IntoIterator<Item = RangeMapRule>>(iter: I) -> Self {
        let mut rules = Vec::from_iter(iter);
        rules.sort_by_key(|x| x.src.start);
        return RangeMap { rules: rules };
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

    fn split_range_by_rules(&self, range: Range) -> HashSet<Range> {
        ///
        /// Partitions the given range into continous subranges based on the
        /// boundaries of the contained rules.
        ///
        let mut output = HashSet::<Range>::new();

        let mut boundaries_set = HashSet::<i64>::new();
        boundaries_set.insert(range.start);
        boundaries_set.insert(range.end);
        for rule in &self.rules {
            boundaries_set.insert(rule.src.start);
            boundaries_set.insert(rule.src.end);
        }

        let mut boundaries_sorted = Vec::from_iter(boundaries_set);
        boundaries_sorted.sort();

        // Create Ranges from adjacent pairs in the sorted list
        return boundaries_sorted
            .iter()
            .zip(boundaries_sorted.iter().skip(1))
            .map(|(&s, &e)| Range { start: s, end: e })
            .collect::<HashSet<Range>>();
    }
}

const INPUT_FILE: &str = "src/test_input.txt";

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

// fn reduce_maps(a: RangeMap, b: RangeMap) -> RangeMap {
//     /// Collapses two RangeMaps into one.
//     /// E.g. the mapping x -> |a| -> |b| -> y into x -> |c| -> y

// }

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

    let subranges = maps["soil-to-fertilizer"].split_range_by_rules(Range { start: 0, end: 100 });

    for range in subranges {
        println!("subrange:\t{:?}", range);
    }

    // println!("{}", contents);
}
