use counter::Counter;
use std::cmp::max_by;
use std::cmp::Ordering;
use std::fs;

const CARD_STRENGTH: &str = "J23456789TQKA";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    handtype: HandType,
    cards: String,
    best_hand: String,
}

impl Hand {
    pub fn new(cards: &String) -> Self {
        let best_hand = replace_jokers(cards);
        let char_counts = best_hand.chars().collect::<Counter<_>>();
        let count_counts = char_counts.values().collect::<Counter<_>>();

        let handtype = match (
            count_counts.get(&2),
            count_counts.get(&3),
            count_counts.get(&4),
            count_counts.get(&5),
        ) {
            (Some(_), Some(_), _, _) => HandType::FullHouse,
            (_, _, _, Some(_)) => HandType::FiveOfAKind,
            (_, _, Some(_), _) => HandType::FourOfAKind,
            (_, Some(_), _, _) => HandType::ThreeOfAKind,
            (Some(&2), _, _, _) => HandType::TwoPair,
            (Some(&1), _, _, _) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        return Self {
            handtype,
            cards: String::from(cards),
            best_hand: String::from(best_hand),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_key = (
            &self.handtype,
            self.cards
                .chars()
                .map(|c| CARD_STRENGTH.find(c).expect("Invalid card character"))
                .collect::<Vec<_>>(),
        );

        let other_key = (
            &other.handtype,
            other
                .cards
                .chars()
                .map(|c| CARD_STRENGTH.find(c).expect("Invalid card character"))
                .collect::<Vec<_>>(),
        );

        return self_key.cmp(&other_key);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn replace_jokers(cards: &String) -> String {
    let num_jokers = cards.chars().filter(|c| *c == 'J').count();
    if num_jokers == 0 {
        return String::from(cards);
    }
    if num_jokers == 5 {
        return String::from("AAAAA");
    }

    let char_counts = cards
        .chars()
        .filter(|c| *c != 'J')
        .collect::<Counter<char>>();
    let highest_count = char_counts
        .most_common()
        .first()
        .expect("Error: no char counts")
        .1;

    let highest_count_chars: Vec<char> = char_counts
        .iter()
        .filter(|count| *count.1 == highest_count)
        .map(|x| *x.0)
        .collect();

    let replace_joker_with_index = highest_count_chars
        .iter()
        .map(|c| CARD_STRENGTH.find(*c).expect("Invalid card character"))
        .max()
        .expect("Error getting highest count char");
    let mut replace_joker_with = CARD_STRENGTH
        .chars()
        .nth(replace_joker_with_index)
        .expect("Error getting nth char");

    let result = cards.replace("J", replace_joker_with.to_string().as_str());

    return result;
}

fn main() {
    // Note: see commit 616feeb9 for the solution to part 1

    let contents: String = fs::read_to_string("src/input.txt").expect("Unable to read the file");
    let lines = contents.split("\n");

    let mut hands = lines
        .map(|l| l.split(' '))
        .map(|mut s| {
            (
                Hand::new(&s.next().unwrap().to_string()),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    hands.sort();

    let part2_result = hands
        .iter()
        .enumerate()
        .fold(0, |sum, (rank, (hand, bid))| sum + (rank + 1) * bid);
    println!("Part 2: {:?}", part2_result);
}
