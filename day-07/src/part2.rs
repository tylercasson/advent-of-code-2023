use std::{
    cmp,
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt::Debug,
};

/// Camel Cards
///
/// - A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
///
/// # Scoring
///
/// ## Strongest to Weakest:
///
/// - five of a kind: AAAAA
/// - four of a kind: AA8AA
/// - full house: 23332
/// - three of a kind: TTT98
/// - two pair: 23432
/// - one pair: A23A4
/// - high card: 23456
///
/// ## If two hands are of the same type:
///
/// - compare first card in each hand
///     - stronger card wins
/// - repeat with next card and so on
///
/// ## Reward
///
/// - each hand wins its `bid * rank`
///
/// ## Task
///
/// Calculate the total winnings by adding up the result of all `bid * rank`
///
/// # Part 2 - Joker's Wild
///
/// - Jokers are now wild
/// - use jokers to shift to highest possible identity
///

#[derive(Debug)]
pub struct Card {
    value: u32,
    name: String,
}

impl Card {
    pub fn new(value: u32, name: String) -> Card {
        Card { value, name }
    }

    pub fn from(name: String) -> Card {
        let map = card_map();
        let value = map.get(&name).expect("should be valid card");

        Card::new(*value, name)
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
    score: u32,
}

impl Hand {
    pub fn from(line: &str) -> Hand {
        let (cards, bid) = &line.split_once(' ').unwrap();

        let bid = bid.parse::<u32>().unwrap();
        let cards: Vec<Card> = cards
            .chars()
            .map(|card| Card::from(card.to_string()))
            .collect();

        let mut hand = Hand {
            cards,
            bid,
            score: 0,
        };

        hand.update_score();

        hand
    }

    pub fn identity(&self) -> HandIdentity {
        let mut map: BTreeMap<&str, u32> = BTreeMap::new();

        self.cards.iter().for_each(|card| {
            *map.entry(&card.name).or_insert(0) += 1;
        });

        let mut values: Vec<&u32> = map.values().collect();
        values.sort();
        values.reverse();

        let id = match *values {
            [5] => HandIdentity::FiveOfAKind,
            [4, 1] => HandIdentity::FourOfAKind,
            [3, 2] => HandIdentity::FullHouse,
            [3, 1, 1] => HandIdentity::ThreeOfAKind,
            [2, 2, 1] => HandIdentity::TwoPair,
            [2, 1, 1, 1] => HandIdentity::OnePair,
            _ => HandIdentity::HighCard,
        };

        if let Some((_, count)) = map.iter().find(|(&k, _)| k == "J") {
            match id {
                HandIdentity::FiveOfAKind => HandIdentity::FiveOfAKind,
                HandIdentity::FourOfAKind => HandIdentity::FiveOfAKind,
                // JJJ22, JJ222
                HandIdentity::FullHouse => HandIdentity::FiveOfAKind,
                // JJJ23, JJ222, J2222
                HandIdentity::ThreeOfAKind => match count {
                    2 => HandIdentity::FiveOfAKind,
                    _ => HandIdentity::FourOfAKind,
                },
                // JJ223, J2233
                HandIdentity::TwoPair => match count {
                    2 => HandIdentity::FourOfAKind,
                    _ => HandIdentity::FullHouse,
                },
                // JJ234, 22J34
                HandIdentity::OnePair => HandIdentity::ThreeOfAKind,
                HandIdentity::HighCard => HandIdentity::OnePair,
            }
        } else {
            id
        }
    }

    pub fn update_score(&mut self) {
        let id = self.identity();
        self.score = id.value();
    }
}

#[derive(Debug)]
pub enum HandIdentity {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandIdentity {
    pub fn value(&self) -> u32 {
        match self {
            HandIdentity::FiveOfAKind => 7,
            HandIdentity::FourOfAKind => 6,
            HandIdentity::FullHouse => 5,
            HandIdentity::ThreeOfAKind => 4,
            HandIdentity::TwoPair => 3,
            HandIdentity::OnePair => 2,
            HandIdentity::HighCard => 1,
        }
    }
}

pub fn card_map() -> HashMap<String, u32> {
    HashMap::from([
        ("J".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
        ("T".to_string(), 10),
        ("Q".to_string(), 12),
        ("K".to_string(), 13),
        ("A".to_string(), 14),
    ])
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let lines = input.lines();

    let mut hands: Vec<Hand> = lines.map(Hand::from).collect();

    hands.sort_by(|a, b| {
        if a.score == b.score {
            // compare card by card until no longer equal
            let mut comp = cmp::Ordering::Equal;
            'outer: for (i, a_card) in a.cards.iter().enumerate() {
                let b_card = b.cards.get(i).expect("should be card at index");
                match a_card.value.cmp(&b_card.value) {
                    cmp::Ordering::Equal => continue,
                    c => {
                        comp = c;
                        break 'outer;
                    }
                }
            }
            comp
        } else {
            a.score.cmp(&b.score)
        }
    });

    let total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1));

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let expected = "5905".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
