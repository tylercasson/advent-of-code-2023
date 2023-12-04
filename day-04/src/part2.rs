use std::{collections::BTreeMap, error::Error};

#[derive(Debug, Clone)]
pub struct Card {
    id: u32,
    winners: Vec<u32>,
    results: Vec<u32>,
    points: u32,
    wins: u32,
}

impl Card {
    pub fn from(card_data: &str) -> Card {
        let mut card = Card {
            id: 0_u32,
            winners: vec![],
            results: vec![],
            points: 0,
            wins: 0,
        };

        let clean_data = card_data.replace("Card ", "");
        let card_parts: Vec<&str> = clean_data.trim().split(':').collect();
        card.id = Card::parse_id(card_parts.first().expect("should have id"));

        let card_halves: Vec<&str> = card_parts
            .last()
            .expect("should have number halves")
            .split(" | ")
            .map(|half| half.trim())
            .collect();

        card.winners =
            Card::parse_numbers(card_halves.first().expect("should have winning numbers"));

        card.results = Card::parse_numbers(card_halves.last().expect("should have own numbers"));

        card.calculate_points();

        card
    }

    fn parse_id(input: &str) -> u32 {
        input.parse::<u32>().expect("should be valid id integer")
    }

    fn parse_numbers(input: &str) -> Vec<u32> {
        input
            .split(' ')
            .filter(|&el| !el.is_empty())
            .map(|number| number.parse::<u32>().expect("should be valid id integer"))
            .collect()
    }

    fn calculate_points(&mut self) {
        self.calculate_wins();

        if self.wins > 0 {
            // bit shift ftw
            self.points = 1 << (self.wins - 1);
        }
    }

    fn calculate_wins(&mut self) {
        self.wins = self
            .results
            .iter()
            .filter(|a| self.winners.contains(a))
            .count() as u32;
    }
}

#[derive(Debug)]
pub struct CardPile {
    cards: Vec<Card>,
    points: u32,
    copy_count: u32,
}

impl CardPile {
    pub fn from(pile_data: &str) -> CardPile {
        let mut pile = CardPile {
            cards: vec![],
            points: 0,
            copy_count: 0,
        };
        pile_data.lines().for_each(|line| {
            pile.cards.push(Card::from(line));
        });

        let mut copy_map: BTreeMap<usize, u32> = BTreeMap::new();

        pile.cards.iter().enumerate().for_each(|(i, card)| {
            let wins = card.wins as usize;
            copy_map.entry(i).or_insert(1);

            // add own card count to next N cards where N = current card wins
            if let Some(count) = copy_map.get_mut(&i) {
                let own_count = &count.clone();
                for j in i + 1..=i + wins {
                    copy_map.entry(j).or_insert(1);
                    if let Some(count) = copy_map.get_mut(&j) {
                        *count += *own_count;
                    }
                }
            }
        });

        pile.calculate_points();
        pile.copy_count = copy_map.values().sum();

        pile
    }

    fn calculate_points(&mut self) {
        self.points = self
            .cards
            .iter()
            .fold(self.points, |sum, card| sum + card.points)
    }
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let pile = CardPile::from(input);

    Ok(pile.copy_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part2_example_input_total_cards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let expected = "30".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
