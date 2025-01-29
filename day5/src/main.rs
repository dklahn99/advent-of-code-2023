use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Range {
    /// Defines the range [start, end)
    start: i64,
    end: i64,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
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

    fn contains(&self, i: i64, reverse: Option<bool>) -> bool {
        let range: &Range;
        if reverse.unwrap_or(false) {
            range = &self.dest;
        } else {
            range = &self.src;
        }

        if range.start <= i && i < range.end {
            return true;
        }
        return false;
    }

    fn map(&self, i: i64, reverse: Option<bool>) -> i64 {
        // println!("DEBUG: {:?}\ti: {:?}\treverse: {:?}", self, i, reverse);
        assert!(self.contains(i, reverse));

        if reverse.unwrap_or(false) {
            let offset = i - self.dest.start;
            return self.src.start + offset;
        } else {
            let offset = i - self.src.start;
            return self.dest.start + offset;
        };
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
    fn map(&self, i: i64, reverse: Option<bool>) -> i64 {
        for rule in &self.rules {
            if rule.contains(i, reverse) {
                return rule.map(i, reverse);
            }
        }
        return i;
    }

    ///
    /// Partitions the given range into continous subranges based on the
    /// boundaries of the contained rules. Returns a set of tuples where
    /// the first element is a range and the second element is the corresponding
    /// rule if it exists.
    ///
    fn split_range_by_rules(&self, range: Range) -> HashSet<(Range, Option<&RangeMapRule>)> {
        let mut output: HashSet<(Range, Option<&RangeMapRule>)> = HashSet::new();

        let mut i: i64 = range.start;
        for rule in &self.rules {
            // self.rules is sorted by rule.src.start in the constructor

            // If i is after the end of the rule, skip the rule.
            // This should only happen at the very beginning when i = range.start
            if rule.src.end <= i {
                assert!(i == range.start);
                continue;
            }

            // If there is a gap in the rules between i and rule.src.start
            if i < rule.src.start {
                output.insert((
                    Range {
                        start: i,
                        end: rule.src.start,
                    },
                    None,
                ));
                i = rule.src.start;
            }

            output.insert((
                Range {
                    start: i,
                    end: rule.src.end,
                },
                Some(rule),
            ));
            i = cmp::min(range.end, rule.src.end);

            if range.end <= i {
                break;
            }
        }

        // If there is remaining space between i and range.end
        if i < range.end {
            output.insert((
                Range {
                    start: i,
                    end: range.end,
                },
                None,
            ));
        }

        return output;
    }

    /// Create a new set of rules that are equivalent to the operation
    /// x -> |rule| -> |map| -> y
    fn merge_rules(rule: &RangeMapRule, map: &RangeMap) -> HashSet<RangeMapRule> {
        let mut output: HashSet<RangeMapRule> = HashSet::new();
        let range_rules = map.split_range_by_rules(rule.dest);

        for (range, map_rule) in range_rules {
            let dest: Range;
            if map_rule.is_some() {
                dest = Range {
                    start: map_rule.unwrap().map(range.start, None),
                    end: map_rule.unwrap().map(range.end - 1, None) + 1,
                };
            } else {
                dest = range
            }
            let result = RangeMapRule {
                src: Range {
                    start: rule.map(range.start, Some(true)),
                    end: rule.map(range.end - 1, Some(true)) + 1,
                },
                dest: dest,
            };
            output.insert(result);
        }

        return output;
    }

    /// Collapses two RangeMaps into one.
    /// E.g. the mapping x -> |self| -> |other| -> y into x -> |new| -> y
    fn reduce(&self, other: &RangeMap) -> RangeMap {
        let self_boundaries: HashSet<i64> = (&self)
            .rules
            .iter()
            .flat_map(|r| [r.dest.start, r.dest.end])
            .collect();
        let other_boundaries: HashSet<i64> = other
            .rules
            .iter()
            .flat_map(|r| [r.src.start, r.src.end])
            .collect();
        let mut boundaries: Vec<i64> = self_boundaries.union(&other_boundaries).copied().collect();
        boundaries.sort();

        let rules = boundaries
            .iter()
            .zip(boundaries.iter().skip(1))
            .map(|(&s, &e)| RangeMapRule {
                src: Range {
                    start: self.map(s, Some(true)),
                    end: self.map(e, Some(true)),
                },
                dest: Range {
                    start: other.map(s, None),
                    end: other.map(e, None),
                },
            })
            .collect::<HashSet<RangeMapRule>>();

        return RangeMap::from_iter(rules);
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
    return chain_lookup(maps[map_name].map(i, None), maps, &sequence[1..]);
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

    // let subranges = maps["soil-to-fertilizer"].split_range_by_rules(Range { start: 0, end: 100 });
    // for range in subranges {
    //     println!("subrange:\t{:?}", range);
    // }

    // let rule = RangeMapRule {
    //     src: Range {
    //         start: 98,
    //         end: 100,
    //     },
    //     dest: Range { start: 50, end: 52 },
    // };
    // let result = RangeMap::merge_rules(&rule, &maps["soil-to-fertilizer"]);

    let reduced = maps["seed-to-soil"].reduce(&maps["soil-to-fertilizer"]);
    for rule in &reduced.rules {
        println!("rule:\t{:?}", rule);
    }
    println

    // println!("{}", contents);
}
