use counter::Counter;
use std::cmp::Ordering;
use std::fs;

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
}

impl Hand {
    pub fn new(hand_str: &str) -> Self {
        let char_counts = hand_str.chars().collect::<Counter<_>>();
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
            handtype: handtype,
            cards: hand_str.to_string(),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        const CARD_STRENGTH: &str = "23456789TJQKA";
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

fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Unable to read the file");
    let lines = contents.split("\n");

    let mut hands = lines
        .map(|l| l.split(' '))
        .map(|mut s| {
            (
                Hand::new(s.next().unwrap()),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    hands.sort();

    let part1_result = hands
        .iter()
        .enumerate()
        .fold(0, |sum, (rank, (hand, bid))| sum + (rank + 1) * bid);
    println!("Part 1: {:?}", part1_result);
}
