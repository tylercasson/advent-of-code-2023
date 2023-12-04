use std::error::Error;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winners: Vec<u32>,
    results: Vec<u32>,
    points: u32,
}

impl Card {
    pub fn from(card_data: &str) -> Card {
        let mut card = Card {
            id: 0_u32,
            winners: vec![],
            results: vec![],
            points: 0,
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

        dbg!("{:?}", &card);

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
        let wins = self
            .results
            .iter()
            .filter(|a| self.winners.contains(a))
            .count() as u32;

        if wins > 0 {
            // bit shift ftw
            self.points = 1 << (wins - 1);
        }
    }
}

pub struct CardPile {
    cards: Vec<Card>,
    points: u32,
}

impl CardPile {
    pub fn from(pile_data: &str) -> CardPile {
        let mut pile = CardPile {
            cards: vec![],
            points: 0,
        };
        pile_data.lines().for_each(|line| {
            pile.cards.push(Card::from(line));
        });

        pile.calculate_points();

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

    Ok(pile.points.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part1_example_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let expected = "13".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
